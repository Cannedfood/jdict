use crate::kana::to_romaji;

const SIMILAR_SOUNDS: &[(i32, &[&str])] = &[
    (1, &["a", "aa"]),
    (1, &["e", "ee", "ei"]),
    (1, &["i", "ii"]),
    (1, &["o", "oo", "ou"]),
    (1, &["u", "uu"]),
    (1, &["s", "z"]),
    (2, &["zu", "dzu", "tsu"]),
    (2, &["j", "ch"]),
    (3, &["chu", "chou"]),
];

pub fn similar_sounding_words(s: &str, max_cost: i32) -> Vec<(String, i32)> {
    let romaji = to_romaji(s);

    let mut result = Vec::new();

    similar_sounding_words_recursive(&romaji, 0, 0, max_cost, &mut result);

    result
}

fn similar_sounding_words_recursive(
    s: &str,
    position: usize,
    previous_cost: i32,
    max_cost: i32,
    into: &mut Vec<(String, i32)>)
{
    let subs = &s[position..];
    if subs.is_empty() {
        return;
    }

    for (cost, sounds) in SIMILAR_SOUNDS {
        let new_cost = previous_cost + cost;
        if new_cost > max_cost {
            continue;
        }

        for sound in sounds.iter() {
            if subs.starts_with(sound) {
                let before_sound = &s[..position];
                let after_sound = &s[position + sound.len()..];

                for similar_sound in sounds.iter() {
                    if similar_sound != &subs {
                        let new_string = before_sound.to_string() + similar_sound + after_sound;

                        similar_sounding_words_recursive(
                            &new_string,
                            position + similar_sound.len(),
                            new_cost,
                            max_cost,
                            into
                        );

                        into.push((new_string, new_cost));
                    }
                }
            }
        }
    }

    if let Some((idx, _)) = subs.char_indices().nth(1) {
        similar_sounding_words_recursive(s, position + idx, previous_cost, max_cost, into);
    }
}
