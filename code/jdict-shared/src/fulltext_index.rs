use std::collections::HashMap;

use itertools::Itertools;
use unicode_blocks::{find_unicode_block, is_cjk_block};

pub type Syllable = [char; 3];

#[derive(Default)]
pub struct FullTextIndex {
    pub entries: HashMap<Syllable, Vec<u32>>,
}

impl FullTextIndex {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, text: &str, id: u32) {
        for syllable in syllables(text, false) {
            self.entries.entry(syllable).or_insert(vec![]).push(id);
        }
    }

    pub fn remove_duplicates(&mut self) {
        for (_, ids) in self.entries.iter_mut() {
            ids.sort();
            ids.dedup();
        }
    }

    // Returns a set of IDs that match all syllables in the text; May contain false positives
    pub fn broadphase_search(&self, text: &str) -> Vec<u32> {
        // Get all entries that contain any of the syllables
        let mut syllable_sets =
            syllables(text, true).unique()
            .filter_map(|syllable| self.entries.get(&syllable))
            .collect_vec();

        // If no syllables were found, return an empty set
        if syllable_sets.is_empty() {
            return Vec::new();
        }

        // Return entries that contain all syllables
        // For that we start with the "rarest" syllables, and then check if all other syllables contain it

        // Sort by rarity, so that we scan over the smallest sets first
        syllable_sets.sort_by_key(|entries| entries.len());

        let first_entry = &syllable_sets[0];
        first_entry.iter()
        .filter_map(|id| {
            if syllable_sets.iter().skip(1).all(|entries| entries.binary_search(id).is_ok()) {
                Some(*id)
            }
            else {
                None
            }
        })
        .collect()
    }
}

fn is_word_separator(c: char) -> bool {
    c.is_whitespace() || c.is_ascii_punctuation()
}

fn first_syllable(s: &str, minimal: bool) -> Option<Syllable> {
    let mut chars = s.chars();
    let a = chars.next().unwrap_or('\0');
    let b = chars.next().unwrap_or('\0');
    let c = chars.next().unwrap_or('\0');

    let starts_with_word_separator = is_word_separator(a);
    let contains_word_separator = is_word_separator(a) || is_word_separator(b) || is_word_separator(c);

    if starts_with_word_separator {
        return None;
    }
    if minimal && contains_word_separator {
        return None;
    }

    let a_codeblock = find_unicode_block(a).unwrap();
    let b_codeblock = find_unicode_block(b).unwrap();
    let c_codeblock = find_unicode_block(c).unwrap();

    let codeblock_max_len =
        if is_kana_block(a_codeblock) { 2 }
        else if is_cjk_block(a_codeblock) { 1 }
        else { 3 };

    Some(
        if a_codeblock != b_codeblock || codeblock_max_len <= 1 {
            [a, '\0', '\0']
        }
        else if a_codeblock != c_codeblock || codeblock_max_len <= 2 {
            [a, b, '\0']
        }
        else {
            [a, b, c]
        }
    )
}

fn syllables(s: &str, minimal: bool) -> impl Iterator<Item = Syllable> + '_ {
    s
    .split(is_word_separator)
    .flat_map(move |word| {
        word
        .char_indices()
        .filter_map(
            move|(char_position, _char)| first_syllable(&word[char_position..], minimal)
        )
    })
}

fn is_kana_block(block: unicode_blocks::UnicodeBlock) -> bool {
    block == unicode_blocks::HIRAGANA ||
    block == unicode_blocks::KATAKANA ||
    block == unicode_blocks::KATAKANA_PHONETIC_EXTENSIONS
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn test_syllables() {
        let text = "Hello world! こんにちは";
        let syllables = super::syllables(text, false).collect_vec();
        assert_eq!(syllables, [
            ['H', 'e', 'l'],
            ['e', 'l', 'l'],
            ['l', 'l', 'o'],
            ['l', 'o', '\0'],
            ['o', '\0', '\0'],
            ['w', 'o', 'r'],
            ['o', 'r', 'l'],
            ['r', 'l', 'd'],
            ['l', 'd', '\0'],
            ['d', '\0', '\0'],
            ['こ', 'ん', '\0'],
            ['ん', 'に', '\0'],
            ['に', 'ち', '\0'],
            ['ち', 'は', '\0'],
            ['は', '\0', '\0'],
        ]);
    }

    #[test]
    fn test_syllables_minimal() {
        let text = "Hello world! こんにちは";
        let syllables = super::syllables(text, true).collect_vec();
        assert_eq!(syllables, [
            ['H', 'e', 'l'],
            ['e', 'l', 'l'],
            ['l', 'l', 'o'],
            ['w', 'o', 'r'],
            ['o', 'r', 'l'],
            ['r', 'l', 'd'],
            ['こ', 'ん', '\0'],
            ['ん', 'に', '\0'],
            ['に', 'ち', '\0'],
            ['ち', 'は', '\0'],
        ]);
    }
}
