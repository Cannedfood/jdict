use crate::jmdict;
use crate::kana::{romaji_to, KanaType};

pub struct SearchWeights {
    pub kanji: u32,
    pub kanji_position_penalty_pct: u32,
    pub reading: u32,
    pub reading_position_penalty_pct: u32,
    pub sense: u32,
    pub sense_position_penalty_pct: u32,
    pub gloss_position_penalty_pct: u32,

    pub exact: u32,
    pub word_exact: u32,
    pub starts_with: u32,
    pub word_starts_with: u32,
    pub contains: u32,
    pub position_penalty_pct: u32,
}
impl Default for SearchWeights {
    fn default() -> Self {
        Self {
            kanji: 3,
            kanji_position_penalty_pct: 100,
            reading: 2,
            reading_position_penalty_pct: 100,
            sense: 1,
            sense_position_penalty_pct: 100,
            gloss_position_penalty_pct: 100,

            exact: 5,
            word_exact: 4,
            starts_with: 3,
            word_starts_with: 2,
            contains: 1,
            position_penalty_pct: 100,
        }
    }
}

pub fn search(
    text: &str,
    weights: &SearchWeights,
    entries: &[jmdict::Entry],
    result: &mut Vec<(u32, u32)>,
) {
    result.clear();

    if text.trim().len() < 3 {
        return;
    }

    let mut groups = Vec::<Vec<String>>::new();
    for piece in text.split_whitespace() {
        let mut group = vec![piece.to_string()];

        let (failures, hiragana) = romaji_to(KanaType::Hiragana, piece);
        if failures == 0 {
            group.push(hiragana);
            // TODO: Try to deconjugate verbs and adjectives
        }

        let (failures, katakana) = romaji_to(KanaType::Katakana, piece);
        if failures == 0 {
            group.push(katakana);
        }

        groups.push(group);
    }
    if groups.is_empty() {
        return;
    }

    // Sort groups by length, so that we try to match longer pieces first.
    // This should improve performance, because longer strings are less likely to match, and we can reject more entries in the first pass.
    groups.sort_by_cached_key(|group| group.iter().map(|w| w.len()).sum::<usize>());

    println!("Searching for {:?}", groups);

    let mut iter = groups.into_iter();

    let first_group = iter.next().unwrap();
    for (i, entry) in entries.iter().enumerate() {
        let score = rate_match(&first_group, entry, weights);
        if score > 0 {
            result.push((i as u32, score));
        }
    }

    for remaining_group in iter {
        result.retain_mut(|(entry_idx, score)| {
            let entry = &entries[*entry_idx as usize];
            *score *= rate_match(&remaining_group, entry, weights);
            *score > 0
        });
    }

    result.sort_unstable_by_key(|(_, score)| std::cmp::Reverse(*score));
}

fn rate_match(
    contains_one_of: &Vec<String>,
    entry: &jmdict::Entry,
    weights: &SearchWeights,
) -> u32 {
    let mut score = 0;
    for piece in contains_one_of {
        for (kanji_idx, kanji) in entry.kanji.iter().enumerate() {
            if let Some(match_score) = text_match(weights, piece, &kanji.text) {
                let kanji_score =
                    position_penalty(weights.kanji_position_penalty_pct, kanji_idx as u32)
                        * match_score
                        * weights.kanji;

                score = score.max(kanji_score.ceil());
            }
        }

        for (reading_idx, reading) in entry.reading.iter().enumerate() {
            if let Some(match_score) = text_match(weights, piece, &reading.text) {
                let reading_score =
                    position_penalty(weights.reading_position_penalty_pct, reading_idx as u32)
                        * match_score
                        * weights.reading;

                score = score.max(reading_score.ceil());
            }
        }

        for (sense_idx, sense) in entry.sense.iter().enumerate() {
            for (gloss_idx, gloss) in sense.glosses.iter().enumerate() {
                if let Some(match_score) = text_match(weights, piece, &gloss.text) {
                    let penalty = position_penalty(
                        weights.sense_position_penalty_pct,
                        sense_idx as u32 + gloss_idx as u32,
                    ) * position_penalty(
                        weights.gloss_position_penalty_pct,
                        sense_idx as u32 + gloss_idx as u32,
                    );

                    let gloss_score = penalty * match_score * weights.sense;

                    score = score.max(gloss_score.ceil())
                }
            }
        }
    }
    score
}

struct Fraction {
    pub numerator:   u32,
    pub denominator: u32,
}
impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
    pub fn ceil(&self) -> u32 { self.numerator.div_ceil(self.denominator) }
}
impl std::ops::Mul<u32> for Fraction {
    type Output = Fraction;
    fn mul(self, rhs: u32) -> Fraction { Self::new(self.numerator * rhs, self.denominator) }
}
impl std::ops::Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}
fn position_penalty(penalty_pct: u32, position: u32) -> Fraction {
    Fraction::new(1, 1 + position * penalty_pct / 100)
}

fn text_match(weights: &SearchWeights, term: &str, text: &str) -> Option<u32> {
    fn next_char(text: &str, pos: usize) -> Option<char> { text[pos..].chars().next() }
    fn prev_char(text: &str, pos: usize) -> Option<char> { text[..pos].chars().next_back() }

    let pos = text.find(term)?;

    let exact_match = text == term;
    let starts_with = pos == 0;
    let word_starts_with = pos == 0
        || prev_char(text, pos)
            .map(|c| !c.is_alphabetic())
            .unwrap_or(false);
    let word_exact_match = word_starts_with
        && (pos + term.len() == text.len()
            || next_char(text, pos + term.len())
                .map(|c| !c.is_alphabetic())
                .unwrap_or(false));

    Some(if exact_match {
        weights.exact
    }
    else if word_exact_match {
        weights.word_exact
    }
    else if starts_with {
        weights.starts_with
    }
    else if word_starts_with {
        weights.word_starts_with
    }
    else {
        weights.contains
    })
}
