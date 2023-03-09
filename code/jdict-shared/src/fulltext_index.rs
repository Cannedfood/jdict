use std::{collections::BTreeMap, ops::Bound};

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
		for (_key, ids) in self.entries.iter_mut() {
			ids.sort_unstable_by_key(|(id, weight)| (*id, std::cmp::Reverse(*weight)));
			ids.dedup_by_key(|id| id.0);
		}

		// self.write_stats("fulltext_index_stats.txt");
	}

	pub fn search(&self, text: &str) -> Vec<(u32, i32)> {
        let mut ids = Vec::new();

		for word in split_words(text) {
			let lowercase = word.to_lowercase();
			let mut cursor = self.entries.lower_bound(Bound::Included(&lowercase));
			while let Some((key, value)) = cursor.key_value() && key.starts_with(&lowercase) {
				let exact_match = key == &lowercase;
				let length_penalty = (key.len() as i32 - word.len() as i32) * 2000;

				for (id, weight) in value {
					ids.push((
						*id,
						if exact_match { *weight + i32::MAX / 2 }
						else { *weight - length_penalty }
					));
				}

				cursor.move_next();
			}
		}

		ids.sort_unstable_by_key(|(id, weight)| (*id, std::cmp::Reverse(*weight)));
		ids.dedup_by_key(|id| id.0);
		ids.sort_by_key(|(_, weight)| std::cmp::Reverse(*weight));

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
	.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation() || c.is_ascii_digit())
	.filter(|word|
		!word.is_empty() &&
		!STOP_WORDS.contains(word)
	)
}
