pub mod parsing;
pub mod reading_meaning;

use compact_str::CompactString;
pub use parsing::parse_kanjidic2;
pub use reading_meaning::*;
use smallvec::SmallVec;

// header
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Header {
    pub file_version:     CompactString,
    pub database_version: CompactString,
    pub date_of_creation: CompactString,
}

// character
// We discard a lot of information here, including:
// - <literal> and all <cp_value> fields except <cp_value cp_type="ucs">.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Character {
    // cp_value cp_type="ucs"
    pub unicode: char,
    // radical
    pub radicals: SmallVec<[u8; 8]>,
    // radical rad_type=nelson_c
    pub radicals_nelson_c: Option<SmallVec<[u8; 8]>>,
    // misc
    pub misc: CharacterMetadata,
    // dic_number
    pub dic_number: (), // TODO
    // query_code
    pub query_code: (), // TODO
    // reading_meaning
    pub reading_meaning: SmallVec<[ReadingMeaning; 1]>,
}

// misc
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterMetadata {}
