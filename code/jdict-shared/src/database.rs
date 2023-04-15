use std::{collections::HashMap, path::Path};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{jmdict::{JMdict, Entry, Priorities, Priority}, kanjidic::{Kanjidic, Character}, kanjivg::{KanjiVG, Kanji}, FullTextIndex, util::{print_time, decompress}, phonetic, fulltext_index::Query};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub jmdict_file: String,
    pub kanjidic_file: String,
    pub kanjivg_file: String,
}

pub struct Dicts {
	pub dict: JMdict,
	pub kanjidic: Kanjidic,
	pub kanjivg: KanjiVG,
}

pub struct DictData<'a> {
	pub dict: &'a [u8],
	pub kanjidic: &'a [u8],
	pub kanjivg: &'a [u8],
}

#[derive(Default)]
pub struct Database {
    pub dict: JMdict,
    pub dict_index: FullTextIndex,
    pub kanjidic: Kanjidic,
    pub kanjidic_index: HashMap<char, u32>,
    pub kanjivg: KanjiVG,
    pub kanjivg_index: HashMap<char, u32>,
}
impl Database {
	pub fn build(data: Dicts) -> Self {
        let kanjivg_index = print_time(
            || build_kanjivg_index(&data.kanjivg),
            |time| println!("Built KanjiVG index in {:?}", time)
        );
        let kanjidic_index = print_time(
            || build_kanjidic_index(&data.kanjidic),
            |time| println!("Built kanjidic index in {:?}", time)
        );
        let dict_index = print_time(
            || build_jmdict_index(&data.dict),
            |time| println!("Built JMdict index in {:?}", time)
        );

		Self {
			dict: data.dict,
            kanjidic: data.kanjidic,
            kanjivg: data.kanjivg,
            dict_index,
            kanjidic_index,
            kanjivg_index,
		}
	}

	pub fn from_bytes(data: DictData) -> Self {
		Self::build(Dicts {
			dict:     print_time(
				|| JMdict::  parse(&decompress(data.dict).unwrap()).unwrap(),
				|time| println!("Parsed JMdict in {:?}", time)
			),
			kanjidic: print_time(
				|| Kanjidic::parse(&decompress(data.kanjidic).unwrap()).unwrap(),
				|time| println!("Parsed kanjidic in {:?}", time)
			),
			kanjivg: print_time(
				|| KanjiVG:: parse(&decompress(data.kanjivg).unwrap()).unwrap(),
				|time| println!("Parsed KanjiVG in {:?}", time)
			),
		})
	}

    pub fn load(config: &Config) -> Self {
        Self::build(Dicts {
			kanjidic: print_time(
				|| Kanjidic::load(Path::new(config.kanjidic_file.as_str())).unwrap(),
				|time| println!("Loaded kanjidic in {:?}", time)
			),
			kanjivg: print_time(
				|| KanjiVG::load(Path::new(config.kanjivg_file.as_str())).unwrap(),
				|time| println!("Loaded KanjiVG in {:?}", time)
			),
			dict: print_time(
				|| JMdict::load(Path::new(config.jmdict_file.as_str())).unwrap(),
				|time| println!("Loaded JMdict in {:?}", time)
			),
		})
    }

    pub fn search(&self, query: &str) -> Vec<Entry> {
        let mut results = Vec::new();

        for (similar, distance) in phonetic::similar_sounding_words(query, 10000000) {
            println!(" Search alt: {} ({})", similar, distance);
            self.dict_index.query(&mut results, &Query::exactly(&similar).with_weight(-distance));
        }
        self.dict_index.query(&mut results, &query.parse().unwrap());

        FullTextIndex::dedup_weighted(&mut results);
        FullTextIndex::sort_results(&mut results);

        results.iter()
        .map(|entry_idx| self.dict.entries[entry_idx.0 as usize].clone())
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
        for (i, kanji) in entry.kanji.iter().enumerate() {
            dict_index.insert_weighted(
				&kanji.value,
				idx as u32,
				WEIGHTING_KANJI.rate(&kanji.priorities, [i, i])
			);
        }
        for (i, reading) in entry.readings.iter().enumerate() {
            dict_index.insert_weighted(
				&reading.value,
				idx as u32,
				WEIGHTING_READING.rate(&reading.priority, [i, i])
			);
            if let Some(romaji) = &reading.romaji {
                dict_index.insert_weighted(
					romaji,
					idx as u32,
					WEIGHTING_READING.rate(&reading.priority, [i, i])
				);
            }
        }
        for (p1, sense) in entry.senses.iter().enumerate() {
            for (p2, gloss) in sense.glosses.iter().enumerate() {
                dict_index.insert_weighted(
					&gloss.value,
					idx as u32,
					WEIGHTING_MEANING.rate(&Vec::default(), [p1, p2])
				);
            }
        }
    }
    dict_index.optimize();
    dict_index
}

fn build_kanjidic_index(kanjidic: &Kanjidic) -> HashMap<char, u32> {
    kanjidic.characters.iter().enumerate()
    .map(|(idx, entry)| (entry.literal.chars().next().unwrap(), idx as u32))
    .collect()
}

fn build_kanjivg_index(kanjidic: &KanjiVG) -> HashMap<char, u32> {
    for kanji in kanjidic.kanji.iter() {
        if kanji.kanji.is_empty() {
            println!("{:?}", kanji);
        }
    }

    kanjidic.kanji.iter().enumerate()
    .map(|(idx, entry)| (entry.kanji.chars().next().unwrap(), idx as u32))
    .collect()
}


pub struct Weighting {
	base: i32,
	position_penalties: [i32; 2],
}
impl Weighting {
	pub fn rate(&self, priorities: &Priorities, position: [usize; 2]) -> i32 {
		self.base
		+ prio_rating(priorities)
		- self.position_penalties[0] * position[0] as i32
		- self.position_penalties[1] * position[1] as i32
	}
}
pub const WEIGHTING_KANJI:   Weighting = Weighting { base: 30, position_penalties: [400, 400] };
pub const WEIGHTING_READING: Weighting = Weighting { base: 20, position_penalties: [400, 400] };
pub const WEIGHTING_MEANING: Weighting = Weighting { base: 1, position_penalties: [200, 200] };

pub fn prio_rating(priorities: &Priorities) -> i32 {
    priorities.iter().map(|p| match p {
        Priority::News1 => 2000,
        Priority::News2 => 1000,
        Priority::Ichi1 => 2000,
        Priority::Ichi2 => 1000,
        Priority::Spec1 => 2000,
        Priority::Spec2 => 1000,
        Priority::Gai1 => 2000,
        Priority::Gai2 => 1000,
        Priority::NF(_) => 0,
    })
    .sum::<i32>()
}
