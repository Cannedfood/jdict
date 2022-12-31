use std::str::FromStr;

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KanjiVG {
    pub kanji: Vec<Kanji>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub path: String,
    pub typ: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Kanji {
    pub kanji: String,
    pub original: Option<String>,
    pub phon: Option<String>,
    pub position: Option<Position>,
    pub partial: bool,
    pub number: Option<u8>,
    pub part: Option<u8>,
    pub radical_form: bool,
    pub radical: Option<Radical>,
    pub strokes: Vec<Stroke>,
    pub parts: Vec<Kanji>,
    pub trad_form: bool,
    pub variant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position {
    Bottom,
    Kamae,
    Left,
    Nyo,
    Nyoc,
    Right,
    Tare,
    Tarec,
    Top,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Radical {
    General,
    Jis,
    Nelson,
    Tradit,
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bottom" => Ok(Position::Bottom),
            "kamae" => Ok(Position::Kamae),
            "left" => Ok(Position::Left),
            "nyo" => Ok(Position::Nyo),
            "nyoc" => Ok(Position::Nyoc),
            "right" => Ok(Position::Right),
            "tare" => Ok(Position::Tare),
            "tarec" => Ok(Position::Tarec),
            "top" => Ok(Position::Top),
            _ => Err(()),
        }
    }
}

impl FromStr for Radical {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "general" => Ok(Radical::General),
            "jis" => Ok(Radical::Jis),
            "nelson" => Ok(Radical::Nelson),
            "tradit" => Ok(Radical::Tradit),
            _ => Err(()),
        }
    }
}
