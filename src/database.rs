use egui::ahash::HashMap;

use crate::{jmdict, kanjidic2, kanjivg};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Database {
    pub dictionary: Vec<jmdict::Entry>,
    pub kanji_dictionary: HashMap<char, kanjidic2::Character>,
    pub kanji_strokes: HashMap<char, kanjivg::StrokeGroup>,
}
