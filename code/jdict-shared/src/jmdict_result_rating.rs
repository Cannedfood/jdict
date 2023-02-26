#![allow(dead_code)]

use crate::jmdict::{self, Priorities, Priority};

const MATCH_EXACT: i32         = 400000;
const MATCH_STARTS_WITH: i32   = 300000;
const MATCH_ENDS_WITH: i32     = 200000;
const MATCH_CONTAINS: i32      = 100000;
const MATCH_FUZZY_PENALTY: i32 =   1000;

const HAS_NEWS1: i32 = 2000;
const HAS_NEWS2: i32 = 1000;
const HAS_ICHI1: i32 = 2000;
const HAS_ICHI2: i32 = 1000;
const HAS_SPEC1: i32 = 2000;
const HAS_SPEC2: i32 = 1000;
const HAS_GAI1: i32  = 2000;
const HAS_GAI2: i32  = 1000;
// static_assert(has_news1 + has_news2 + has_ichi1 + has_ichi2 + has_spec1 + has_spec2 + has_gai1 + has_gai2 < match_contains);

const WORD_LENGTH: i32 = 2000;
const MAX_WORD_LENGTH: i32 = 100;

const KANJI: i32           = 30;
const READING_KANA: i32    = 20;
const READING_ROMAJI: i32  = 20;
const MEANING: i32         = 1;
const SEQUENCE_NUMBER: i32 = 1000000;

const POSITION_PENALTY_KANJI: i32   = 400;
const POSITION_PENALTY_READING: i32 = 400;
const POSITION_PENALTY_SENSE: i32   = 200;
const POSITION_PENALTY_GLOSS: i32   = 400;

const HIGHLIGHTED_GLOSS: i32 = 10;

fn rate_text_match(text: &str, query: &str) -> i32 {
    if text == query { MATCH_EXACT }
    else if text.starts_with(query) { MATCH_STARTS_WITH }
    else if text.ends_with(query) { MATCH_ENDS_WITH }
    else if text.contains(query) { MATCH_CONTAINS }
    else { 0 }
}

fn simplify_sense_word(text: &str) -> &str {
    let text = text.strip_prefix("to ").unwrap_or(text);

    text
}

fn rate_text_match_sense(text: &str, query: &str) -> i32 {
    let simplified_text = simplify_sense_word(text);
    let simplified_query = simplify_sense_word(query);
    let is_fuzzy = simplified_text != text || simplified_query != query;

    let result = rate_text_match(simplified_text, simplified_query);

    if is_fuzzy { result - MATCH_FUZZY_PENALTY }
    else { result }
}

pub fn rate_entry_match(entry: &jmdict::Entry, query: &str) -> i32 {
    let mut match_score = 0;
    for (i, kanji) in entry.kanji.iter().enumerate() {
        let mut kanji_score = rate_text_match(&kanji.value, query) * KANJI;
        if kanji_score > 0 {
            kanji_score += prio_rating(&kanji.priorities);
            kanji_score -= (i*i) as i32 * POSITION_PENALTY_KANJI;
        }
        match_score = match_score.max(kanji_score);
    }
    for reading in &entry.readings {
        let mut reading_score = 0;
        reading_score = reading_score.max(rate_text_match(&reading.value, query) * READING_KANA);
        if let Some(romaji) = &reading.romaji {
            reading_score = reading_score.max(rate_text_match(romaji, query) * READING_ROMAJI);
        }
        if reading_score != 0 {
            reading_score += prio_rating(&reading.priority);
        }
        match_score = match_score.max(reading_score);
    }
    for sense in &entry.senses {
        for gloss in &sense.glosses {
            match_score = match_score.max(rate_text_match_sense(&gloss.value, query)) * MEANING;
        }
    }
    if match_score == 0 {
        return 0;
    }

    match_score
}

fn prio_rating(priorities: &Priorities) -> i32 {
    priorities.iter().map(|p| match p {
        Priority::News1 => HAS_NEWS1,
        Priority::News2 => HAS_NEWS2,
        Priority::Ichi1 => HAS_ICHI1,
        Priority::Ichi2 => HAS_ICHI2,
        Priority::Spec1 => HAS_SPEC1,
        Priority::Spec2 => HAS_SPEC2,
        Priority::Gai1 => HAS_GAI1,
        Priority::Gai2 => HAS_GAI2,
        Priority::NF(_) => 0,
    })
    .sum::<i32>()
}
