use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Query {
	StartsWith(String),
	Contains(String),
	EndsWith(String),
}
impl Query {
	pub fn starts_with(s: &str) -> Self { Self::StartsWith(s.to_lowercase()) }
	pub fn contains(s: &str) -> Self { Self::Contains(s.to_lowercase()) }
	pub fn ends_with(s: &str) -> Self { Self::EndsWith(s.to_lowercase()) }
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
			self.entries.entry(word.to_lowercase()).or_insert(vec![]).push((id, weight));
		}
	}

	pub fn optimize(&mut self) {
		self.entries.values_mut().for_each(dedup_weighted);
	}

	pub fn query(&self, result: &mut Vec<(u32, i32)>, query: &Query) {
		const EXACT_MATCH_BONUS: i32 = i32::MAX / 2;
		const LENGTH_PENALTY: i32 = 2000;

		match query {
			Query::StartsWith(s) => {
				result.extend(
					self.entries.range(s.clone()..)
					.take_while(|(key, _)| key.starts_with(s))
					.flat_map(|(key, value)| {
						let match_quality =
							EXACT_MATCH_BONUS * (key == s) as i32
							-LENGTH_PENALTY   * (key.len() as i32 - s.len() as i32);

						value.iter().map(move|(id, weight)| (*id, *weight + match_quality))
					})
				);
			},
			Query::EndsWith(s) => {
				result.extend(
					self.entries.iter()
					.filter(|(key, _)| key.ends_with(s))
					.flat_map(|(key, value)| {
						let match_quality =
							EXACT_MATCH_BONUS * (key == s) as i32
							-LENGTH_PENALTY   * (key.len() as i32 - s.len() as i32);

						value.iter().map(move|(id, weight)| (*id, *weight + match_quality))
					})
				);
			},
			Query::Contains(s) => {
				result.extend(
					self.entries.iter()
					.filter(|(key, _)| key.contains(s))
					.flat_map(|(key, value)| {
						let match_quality =
							EXACT_MATCH_BONUS * (key == s) as i32
							-LENGTH_PENALTY   * (key.len() as i32 - s.len() as i32);

						value.iter().map(move|(id, weight)| (*id, *weight + match_quality))
					})
				);
			}
		}
	}

	pub fn search(&self, text: &str) -> Vec<(u32, i32)> {
        let mut ids = Vec::new();

		for word in split_words(text) {
			self.query(
				&mut ids,
				&match word {
					word if word.starts_with('*') && word.ends_with('*') => Query::contains(&word[1..word.len() - 1]),
					word if word.starts_with('*') => Query::ends_with(&word[1..]),
					word if word.ends_with('*') => Query::starts_with(&word[..word.len() - 1]),
					word => Query::starts_with(word),
				}
			);
		}

		dedup_weighted(&mut ids);
		ids.sort_unstable_by_key(|(_, weight)| std::cmp::Reverse(*weight));

		ids
    }

	// pub fn write_stats(&self, path: &str) {
	// 	let mut line_writer = std::io::LineWriter::new(std::fs::File::create(path).unwrap());
	// 	writeln!(line_writer, "Totals: {} words, {} entries", self.entries.len(), self.entries.values().map(|v| v.len()).sum::<usize>()).unwrap();

	// 	let mut entries_by_count = self.entries.iter().map(|(k, v)| (k.as_str(), v.len())).collect::<Vec<_>>();
	// 	entries_by_count.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
	// 	for (k, v) in entries_by_count {
	// 		writeln!(line_writer, "{k} {v}").unwrap();
	// 	}
	// }
}

fn dedup_weighted(ids: &mut Vec<(u32, i32)>) {
	ids.sort_unstable_by_key(|(id, weight)| (*id, std::cmp::Reverse(*weight)));
	ids.dedup_by_key(|id| id.0);
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
	];

	text
	.split(|c: char| {
		c.is_whitespace() ||
		(c.is_ascii_punctuation() && c != '*') ||
		c.is_ascii_digit()
	})
	.filter(|word|
		!word.is_empty() &&
		!STOP_WORDS.contains(word)
	)
}
