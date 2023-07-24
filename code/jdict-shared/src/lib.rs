#![feature(btree_cursors)]
#![feature(let_chains)]
#![feature(const_swap)]
#![feature(const_mut_refs)]

pub mod database;
pub mod fulltext_index;
pub mod jmdict;
pub mod jmdict_parsing;
pub mod kana;
pub mod kanjidic;
pub mod kanjidic_parsing;
pub mod kanjivg;
pub mod kanjivg_parsing;
pub mod phonetic;
pub mod shared_api;
pub mod util;

pub use fulltext_index::FullTextIndex;
