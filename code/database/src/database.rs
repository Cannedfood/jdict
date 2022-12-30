use std::{collections::HashMap, path::Path};
use itertools::Itertools;
use crate::{fulltext_index::FullTextIndex, jmdict::{JMdict, Entry}, kanjidic::{Kanjidic, Character}, jmdict_result_rating::rate_entry_match, util::print_time};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub public_path: String,
    pub jmdict_file: String,
    pub kanjidic_file: String,
}

pub struct Database {
    pub config: Config,
    pub dict: JMdict,
    pub dict_index: FullTextIndex,
    pub kanjidic: Kanjidic,
    pub kanjidic_index: HashMap<char, u32>,
}
impl Database {
    pub fn new(config: Config) -> Self {
        let kanjidic = print_time(
            || Kanjidic::parse(Path::new(config.kanjidic_file.as_str())),
            |time| println!("Parsed kanjidic in {:?}", time)
        );
        let kanjidic_index = print_time(
            || build_kanjidic_index(&kanjidic),
            |time| println!("Built kanjidic index in {:?}", time)
        );

        let dict = print_time(
            || JMdict::parse(Path::new(config.jmdict_file.as_str())),
            |time| println!("Parsed JMdict in {:?}", time)
        );
        let dict_index = print_time(
            || build_jmdict_index(&dict),
            |time| println!("Built JMdict index in {:?}", time)
        );

        Self {
            config,
            dict,
            dict_index,
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

    pub fn contained_kanji_chars(&self, text: &str) -> Vec<Character> {
        text.chars()
        .unique()
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
