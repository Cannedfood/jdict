use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::util::is_default;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KanjiVG {
    pub kanji: Vec<Kanji>,
}

#[derive(Debug, Clone, Copy)]
pub struct CubicBezier {
    pub start: (f32, f32),
    pub control1: (f32, f32),
    pub control2: (f32, f32),
    pub end: (f32, f32),
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stroke {
    /// The stroke path in SVG format. Usually only contains move-to and cubic beziers.
    pub path: String,
    pub typ: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Kanji {
    pub kanji: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub original: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub phon: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub position: Option<Position>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub partial: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub number: Option<u8>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub part: Option<u8>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub radical_form: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub radical: Option<Radical>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub strokes: Vec<Stroke>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub parts: Vec<Kanji>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub trad_form: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub variant: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
