use std::{collections::HashMap, path::Path};
use itertools::Itertools;
use crate::{fulltext_index::FullTextIndex, jmdict::{JMdict, Entry}, kanjidic::{Kanjidic, Character}, jmdict_result_rating::rate_entry_match, util::print_time, kanjivg::{KanjiVG, Kanji}};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub jmdict_file: String,
    pub kanjidic_file: String,
    pub kanjivg_file: String,
}

#[derive(Default)]
pub struct Database {
    pub config: Config,
    pub dict: JMdict,
    pub dict_index: FullTextIndex,
    pub kanjidic: Kanjidic,
    pub kanjidic_index: HashMap<char, u32>,
    pub kanjivg: KanjiVG,
    pub kanjivg_index: HashMap<char, u32>,
}
impl Database {
    pub fn load(config: Config) -> Self {
        let kanjivg = print_time(
            || KanjiVG::load(Path::new(config.kanjivg_file.as_str())),
            |time| println!("Parsed KanjiVG in {:?}", time)
        );
        let kanjidic = print_time(
            || Kanjidic::load(Path::new(config.kanjidic_file.as_str())),
            |time| println!("Parsed kanjidic in {:?}", time)
        );
        let dict = print_time(
            || JMdict::load(Path::new(config.jmdict_file.as_str())),
            |time| println!("Parsed JMdict in {:?}", time)
        );

        let kanjivg_index = print_time(
            || build_kanjivg_index(&kanjivg),
            |time| println!("Built KanjiVG index in {:?}", time)
        );
        let kanjidic_index = print_time(
            || build_kanjidic_index(&kanjidic),
            |time| println!("Built kanjidic index in {:?}", time)
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
            kanjivg,
            kanjivg_index,
        }
    }

    pub fn search(&self, query: &str) -> Vec<Entry> {
        self.dict_index.broadphase_search(query).iter()
        .map(|entry_idx| &self.dict.entries[*entry_idx as usize])
        .map(|entry| (entry, rate_entry_match(&entry, query)))
        .filter(|(_, rating)| rating > &0)
        .sorted_by(|(_, rating1), (_, rating2)| rating2.cmp(rating1))
        .map(|(entry, _)| entry.clone())
        .collect()
    }

    pub fn contained_kanji_chars(&self, text: &str) -> (Vec<Character>, Vec<Kanji>) {
        let uniq_chars =
            text.chars()
            .unique();

        let chars =
            uniq_chars.clone()
            .filter_map(|c| self.kanjidic_index.get(&c))
            .map(|idx| self.kanjidic.characters[*idx as usize].clone())
            .collect();

        let kanjivg =
            uniq_chars
            .filter_map(|c| self.kanjivg_index.get(&c))
            .map(|idx| self.kanjivg.kanji[*idx as usize].clone())
            .collect();

        (chars, kanjivg)
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

fn build_kanjivg_index(kanjidic: &KanjiVG) -> HashMap<char, u32> {
    for kanji in kanjidic.kanji.iter() {
        if kanji.kanji == "" {
            println!("{:?}", kanji);
        }
    }

    kanjidic.kanji.iter().enumerate()
    .map(|(idx, entry)| (entry.kanji.chars().next().unwrap(), idx as u32))
    .collect()
}
