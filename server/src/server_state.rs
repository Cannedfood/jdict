use std::{collections::HashMap, time::Duration};

use itertools::Itertools;

use crate::{fulltext_index::FullTextIndex, jmdict::{JMdict, Entry}, kanjidic::{Kanjidic, Character}, jmdict_result_rating::rate_entry_match};

pub struct ServerState {
    pub dict: JMdict,
    pub dict_index: FullTextIndex,
    pub kanjidic: Kanjidic,
    pub kanjidic_index: HashMap<char, u32>,
}

impl ServerState {
    pub fn new() -> Self {
        let (kanjidic, kanjidic_time) = measure_time(|| Kanjidic::parse("../res/kanjidic2.xml"));
        println!("Parsed kanjidic in {:?}", kanjidic_time);
        let (kanjidic_index, kanjidic_index_time) = measure_time(|| build_kanjidic_index(&kanjidic));
        println!("Built kanjidic index in {:?}", kanjidic_index_time);

        let (dict, dict_time) = measure_time(|| JMdict::parse("../res/JMdict_e.xml"));
        println!("Parsed JMdict in {:?}", dict_time);
        let (index, index_time) = measure_time(|| build_jmdict_index(&dict));
        println!("Built JMdict index in {:?}", index_time);

        Self {
            dict,
            dict_index: index,
            kanjidic,
            kanjidic_index,
        }
    }

    pub fn search(&self, query: &str) -> Vec<Entry> {
        self.dict_index.broadphase_search(query).unwrap_or_default().iter()
        .map(|entry_idx| &self.dict.entries[*entry_idx as usize])
        .map(|entry| (entry, rate_entry_match(&entry, query)))
        .filter(|(_, rating)| rating > &0)
        .sorted_by(|(_, rating1), (_, rating2)| rating2.cmp(rating1))
        .map(|(entry, _)| entry.clone())
        .collect()
    }

    pub fn search_kanji(&self, query: &str) -> Vec<Character> {
        query.chars()
        .filter_map(|c| self.kanjidic_index.get(&c))
        .map(|idx| self.kanjidic.characters[*idx as usize].clone())
        .collect()
    }
}

fn build_jmdict_index(dict: &JMdict) -> FullTextIndex {
    let mut dict_index = FullTextIndex::new();
    for (idx, entry) in dict.entries.iter().enumerate() {
        for kanji in &entry.kanji {
            dict_index.insert(&kanji.value, idx as u32);
        }
        for reading in &entry.readings {
            dict_index.insert(&reading.value, idx as u32);
            if let Some(romaji) = &reading.romaji {
                dict_index.insert(&romaji, idx as u32);
            }
        }
        for sense in &entry.senses {
            for gloss in &sense.glosses {
                dict_index.insert(&gloss.value, idx as u32);
            }
        }
    }
    dict_index.remove_duplicates();
    dict_index
}

fn build_kanjidic_index(kanjidic: &Kanjidic) -> HashMap<char, u32> {
    kanjidic.characters.iter().enumerate()
    .map(|(idx, entry)| (entry.literal.chars().next().unwrap(), idx as u32))
    .collect()
}

fn measure_time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = std::time::Instant::now();
    let result = f();
    (result, start.elapsed())
}
