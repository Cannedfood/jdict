use std::collections::HashMap;

use itertools::Itertools;
use unicode_blocks::{find_unicode_block, is_cjk_block};

pub type Syllable = [char; 3];

pub struct FullTextIndex {
    pub entries: HashMap<Syllable, Vec<u32>>,
}

impl FullTextIndex {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    // pub fn insert_many<'a, I>(&mut self, texts: I, id: u32)
    //     where I: Iterator<Item = &'a str>
    // {
    //     let tuples = texts.flat_map(|s| s.chars().tuple_windows::<Syllable>());
    //     for (a, b, c) in tuples {
    //         self.entries.entry([a, b, c]).or_insert(vec![]).push(id);
    //     }
    // }

    pub fn insert(&mut self, text: &str, id: u32) {
        for syllable in syllables(text) {
            self.entries.entry(syllable).or_insert(vec![]).push(id);
        }
    }

    pub fn remove_duplicates(&mut self) {
        for (_, ids) in self.entries.iter_mut() {
            ids.sort();
            ids.dedup();
        }
    }

    pub fn broadphase_search(&self, text: &str) -> Option<Vec<u32>> {
        syllables(text)
        .unique()
        .filter_map(|syllable| self.entries.get(&syllable))
        .min_by_key(|ids| ids.len())
        .cloned()
    }
}

fn first_syllable(s: &str) -> Syllable {
    let mut chars = s.chars();
    let a = chars.next().unwrap_or('\0');
    let b = chars.next().unwrap_or('\0');
    let c = chars.next().unwrap_or('\0');

    let a_codeblock = find_unicode_block(a).unwrap();
    let b_codeblock = find_unicode_block(b).unwrap();
    let c_codeblock = find_unicode_block(c).unwrap();

    if is_cjk_block(a_codeblock) || a_codeblock != b_codeblock {
        [a, '\0', '\0']
    }
    else if a_codeblock != c_codeblock {
        [a, b, '\0']
    }
    else {
        [a, b, c]
    }
}

fn syllables<'a>(s: &'a str) -> impl Iterator<Item = Syllable> + 'a {
    s.char_indices()
    .map(
        |c| first_syllable(&s[c.0..])
    )
}