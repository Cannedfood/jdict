#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gloss {
    pub text: compact_str::CompactString,

    // attribute xml:lang
    pub lang: isolang::Language,
    // pri
    pub highlight: bool,
    // attribute g_type
    pub typ: GlossType,
    // attribute g_gend
    pub gender: Option<compact_str::CompactString>, // Unused
}

pub enum GlossTags {}

pub struct Priority {}

// #[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
// pub struct Lang([u8; 3]);
// impl Default for Lang {
//     fn default() -> Self {
//         Lang(*b"eng")
//     }
// }
// impl From<&[u8; 3]> for Lang {
//     fn from(bytes: &[u8; 3]) -> Self {
//         Lang(*bytes)
//     }
// }
// impl FromStr for Lang {
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, ()> {
//         if s.len() != 3 {
//             return Err(());
//         }

//         let bytes = s.as_bytes();

//         Ok(Lang([bytes[0], bytes[1], bytes[2]]))
//     }
// }
// impl AsRef<str> for Lang {
//     fn as_ref(&self) -> &str {
//         unsafe { std::str::from_utf8_unchecked(&self.0) }
//     }
// }
// impl Display for Lang {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(self.as_ref())
//     }
// }

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::EnumString,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum GlossType {
    #[default]
    Regular,
    #[strum(serialize = "expl")]
    Explanation,
    #[strum(serialize = "lit")]
    LiteralTranslation,
    #[strum(serialize = "tm")]
    Trademark,
    #[strum(serialize = "fig")]
    Figurative,
}
