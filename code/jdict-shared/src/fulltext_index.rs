use std::{collections::BTreeMap, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryType {
    StartsWith,
    Contains,
    EndsWith,
    Exactly,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub query_type: QueryType,
    pub query: String,
    pub base_weight: i32,
}
impl Query {
    pub fn new(query_type: QueryType, query: &str) -> Self {
        Self { query_type, query: normalize_string(query), base_weight: 0, }
    }
	pub fn starts_with(s: &str) -> Self { Self::new(QueryType::StartsWith, s) }
	pub fn contains(s: &str)    -> Self { Self::new(QueryType::Contains,   s) }
	pub fn ends_with(s: &str)   -> Self { Self::new(QueryType::EndsWith,   s) }
    pub fn exactly(s: &str)     -> Self { Self::new(QueryType::Exactly,    s) }

    pub fn with_weight(self, weight: i32) -> Self {
        Self {
            base_weight: weight,
            ..self
        }
    }
}
impl FromStr for Query {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            word if word.starts_with('*') && word.ends_with('*') => Ok(Query::contains(&word[1..word.len() - 1])),
            word if word.starts_with('*') => Ok(Query::ends_with(&word[1..])),
            word if word.ends_with('*') => Ok(Query::starts_with(&word[..word.len() - 1])),
            word => Ok(Query::starts_with(word)),
        }
    }
}

#[derive(Default, Clone)]
pub struct FullTextIndex {
	pub entries: BTreeMap<String, Vec<(u32, i32)>>,
}
impl FullTextIndex {
	pub fn new() -> Self { Self::default() }

	pub fn insert(&mut self, text: &str, id: u32) {
		self.insert_weighted(text, id, 0);
	}
	pub fn insert_weighted(&mut self, text: &str, id: u32, weight: i32) {
		for word in split_words(text) {
			self.entries.entry(normalize_string(word)).or_insert(vec![]).push((id, weight));
		}
	}

	pub fn optimize(&mut self) {
		self.entries.values_mut().for_each(FullTextIndex::dedup_weighted);
		// self.write_stats("./fulltext-stats.txt");
	}

	pub fn query(&self, result: &mut Vec<(u32, i32)>, query: &Query) {
		const EXACT_MATCH_BONUS: i32 = i32::MAX / 2;
		const LENGTH_PENALTY: i32 = 2000;

        let q = &query.query;
        let base = query.base_weight;

		match query.query_type {
            QueryType::Exactly => {
                result.extend(
                    self.entries.get(&query.query).into_iter()
                    .flatten()
                    .map(|(id, weight)| (*id, *weight + EXACT_MATCH_BONUS + base))
                );
            },
			QueryType::StartsWith => {
				result.extend(
					self.entries.range(q.clone()..)
					.take_while(|(key, _)| key.starts_with(q))
					.flat_map(|(key, value)| {
						let match_quality =
							EXACT_MATCH_BONUS * (key == q) as i32
							-LENGTH_PENALTY   * (key.len() as i32 - q.len() as i32);

						value.iter().map(move|(id, weight)| (*id, *weight + match_quality + base))
					})
				);
			},
			QueryType::EndsWith => {
				result.extend(
					self.entries.iter()
					.filter(|(key, _)| key.ends_with(q))
					.flat_map(|(key, value)| {
						let match_quality =
							EXACT_MATCH_BONUS * (key == q) as i32
							-LENGTH_PENALTY   * (key.len() as i32 - q.len() as i32);

						value.iter().map(move|(id, weight)| (*id, *weight + match_quality + base))
					})
				);
			},
			QueryType::Contains => {
				result.extend(
					self.entries.iter()
					.filter(|(key, _)| key.contains(q))
					.flat_map(|(key, value)| {
						let match_quality =
							EXACT_MATCH_BONUS * (key == q) as i32
							-LENGTH_PENALTY   * (key.len() as i32 - q.len() as i32);

						value.iter().map(move|(id, weight)| (*id, *weight + match_quality + base))
					})
				);
			}
		}
	}

	// pub fn search(&self, apply_queries: impl FnOnce(&Self, &mut Vec<(u32, i32)>)) -> Vec<(u32, i32)> {
    //     let mut results = Vec::new();

    //     apply_queries(self, &mut results);

	// 	FullTextIndex::dedup_weighted(&mut results);
	// 	FullTextIndex::sort_results(&mut results);

	// 	results
    // }

	// pub fn write_stats(&self, path: &str) {
	// 	let mut line_writer = std::io::LineWriter::new(std::fs::File::create(path).unwrap());
	// 	writeln!(line_writer, "Totals: {} words, {} entries", self.entries.len(), self.entries.values().map(|v| v.len()).sum::<usize>()).unwrap();

	// 	let words   = (self.entries.keys().map(|k| k.bytes().len()).sum::<usize>()) as f32 / (1024.0*1024.0);
	// 	let entries = (self.entries.values().map(|v| v.len()).sum::<usize>() * 4) as f32   / (1024.0*1024.0);
	// 	writeln!(line_writer, "Memory usage: words: {words:.2}MiB, entries: {entries:.2}MiB, total: {:.2}MiB", words + entries).unwrap();

	// 	let mut entries_by_count = self.entries.iter().map(|(k, v)| (k.as_str(), v.len())).collect::<Vec<_>>();
	// 	entries_by_count.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
	// 	for (k, v) in entries_by_count {
	// 		writeln!(line_writer, "{k} {v}").unwrap();
	// 	}
	// }

    pub fn sort_results(results: &mut [(u32, i32)]) {
        results.sort_unstable_by_key(|(_, weight)| std::cmp::Reverse(*weight));
    }

    pub fn dedup_weighted(ids: &mut Vec<(u32, i32)>) {
        ids.sort_unstable_by_key(|(id, weight)| (*id, std::cmp::Reverse(*weight)));
        ids.dedup_by_key(|id| id.0);
    }
}

fn normalize_string(s: &str) -> String {
	s.chars().map(|c| match c {
		'０'..='９' => char::from_u32(c as u32 - '０' as u32 + '0' as u32).unwrap(),
		'Ａ'..='Ｚ' => char::from_u32(c as u32 - 'Ａ' as u32 + 'a' as u32).unwrap(),
		'ａ'..='ｚ' => char::from_u32(c as u32 - 'ａ' as u32 + 'a' as u32).unwrap(),
		'A'..='Z' => char::from_u32(c as u32 - 'A' as u32 + 'a' as u32).unwrap(),
		c => c,
	}).collect()
}

fn split_words(text: &str) -> impl Iterator<Item = &str> + '_ {
	const STOP_WORDS: &[&str] = &[
		"out",
		"as",
		"esp",
		"up",
		"person",
		"from",
		"at",
		"an",
		"by",
		"be",
		"g",
		"or",
		"on",
		"with",
		"for",
		"e",
		"etc",
		"one",
		"and",
		"s",
		"in",
		"the",
		"a",
		"of",
		"to",
		"not",
		"it",
		"is"
	];

	text
	.split(|c: char| {
		c.is_whitespace() ||
		(c.is_ascii_punctuation() && c != '*') ||
		c.is_ascii_digit() ||
		c == '・'
	})
	.filter(|word|
		!word.is_empty() &&
		!STOP_WORDS.contains(word)
	)
}
