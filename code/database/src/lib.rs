#![feature(generators, generator_trait)]

pub mod util;
pub mod jmdict;
pub mod jmdict_parsing;
pub mod jmdict_result_rating;
pub mod kanjidic;
pub mod kanjidic_parsing;
pub mod kanjivg;
pub mod kanjivg_parsing;
pub mod fulltext_index;
pub mod kana;
pub mod database;
pub mod shared_api;

pub use fulltext_index::FullTextIndex;
