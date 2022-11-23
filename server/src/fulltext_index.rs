use std::collections::HashMap;

use itertools::Itertools;

pub struct FullTextIndex {
    pub entries: HashMap<[char; 3], Vec<u32>>,
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
    //     let tuples = texts.flat_map(|s| s.chars().tuple_windows::<(char, char, char)>());
    //     for (a, b, c) in tuples {
    //         self.entries.entry([a, b, c]).or_insert(vec![]).push(id);
    //     }
    // }

    pub fn insert(&mut self, text: &str, id: u32) {
        let tuples = text.chars().tuple_windows::<(char, char, char)>();
        for (a, b, c) in tuples {
            self.entries.entry([a, b, c]).or_insert(vec![]).push(id);
        }
    }

    pub fn remove_duplicates(&mut self) {
        for (_, ids) in self.entries.iter_mut() {
            ids.sort();
            ids.dedup();
        }
    }

    pub fn broadphase_search(&self, text: &str) -> Option<Vec<u32>> {
        text.chars()
        .tuple_windows()
        .unique()
        .filter_map(|(a, b, c)| self.entries.get(&[a, b, c]))
        .min_by_key(|ids| ids.len())
        .cloned()
    }
}
