#![allow(non_camel_case_types)]

use std::str::FromStr;

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum VariantType {
    jis208, // In JIS X 0208 - kuten coding
    jis212, // In JIS X 0212 - kuten coding
    jis213, // In JIS X 0213 - kuten coding (most of the above relate to "shinjitai/kyuujitai" alternative character glyphs)
    deroo, // De Roo number - numeric
    njecd, // Halpern NJECD index number - numeric
    s_h, // The Kanji Dictionary (Spahn & Hadamitzky) - descriptor
    nelson_c, // "Classic" Nelson - numeric
    oneill, // Japanese Names (O'Neill) - numeric
    ucs, // Unicode codepoint- hex
}

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Variant {
    pub typ: VariantType,
    pub value: String,
}

/// <!ELEMENT misc (grade?, stroke_count+, variant*, freq?, rad_name*,jlpt?)>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Misc {
    pub grade: u8, // <!ELEMENT grade (#PCDATA)> The kanji grade level. 1 through 6 indicates a Kyouiku kanji and the grade in which the kanji is taught in Japanese schools. 8 indicates it is one of the remaining Jouyou Kanji to be learned in junior high school. 9 indicates it is a Jinmeiyou (for use in names) kanji which in addition  to the Jouyou kanji are approved for use in family name registers and other official documents. 10 also indicates a Jinmeiyou kanji which is a variant of a Jouyou kanji. [G]
    pub stroke_count: Vec<u8>, // <!ELEMENT stroke_count (#PCDATA)> The stroke count of the kanji, including the radical. If more than one, the first is considered the accepted count, while subsequent ones are common miscounts. (See Appendix E. of the KANJIDIC documentation for some of the rules applied when counting strokes in some of the radicals.) [S]
    pub variant: Vec<Variant>, // <!ELEMENT variant (#PCDATA)> Either a cross-reference code to another kanji, usually regarded as a variant, or an alternative indexing code for the current kanji. The type of variant is given in the var_type attribute.
    pub freq: u32, // <!ELEMENT freq (#PCDATA)> A frequency-of-use ranking. The 2,500 most-used characters have a ranking; those characters that lack this field are not ranked. The frequency is a number from 1 to 2,500 that expresses the relative frequency of occurrence of a character in modern Japanese. This is based on a survey in newspapers, so it is biassed towards kanji used in newspaper articles. The discrimination between the less frequently used kanji is not strong. (Actually there are 2,501 kanji ranked as there was a tie.)
    pub rad_name: Vec<String>, // <!ELEMENT rad_name (#PCDATA)> When the kanji is itself a radical and has a name, this element contains the name (in hiragana.) [T2]
    pub jlpt: u8, // <!ELEMENT jlpt (#PCDATA)> The (former) Japanese Language Proficiency test level for this kanji. Values range from 1 (most advanced) to 4 (most elementary). This field does not appear for kanji that were not required for any JLPT level. Note that the JLPT test levels changed in 2010, with a new 5-level system (N1 to N5) being introduced. No official kanji lists are available for the new levels. The new levels are regarded as being similar to the old levels except that the old level 2 is now divided between N2 and N3.
}

/// <!ELEMENT codepoint (cp_value+)> The codepoint element states the code of the character in the various character set standards.
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Codepoint {
    pub jis208: Option<String>,
    pub jis212: Option<String>,
    pub jis213: Option<String>,
    pub ucs: Option<String>,
}

/// <!ELEMENT rad_value (#PCDATA)> The radical number, in the range 1 to 214. The particular classification type is stated in the rad_type attribute.
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Radical {
    pub classical: u16,
    pub nelson_c: u16,
}

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum DicRefType {
    nelson_c, // "Modern Reader's Japanese-English Character Dictionary", edited by Andrew Nelson (now published as the "Classic"  Nelson).
    nelson_n, // "The New Nelson Japanese-English Character Dictionary", edited by John Haig.
    halpern_njecd, // "New Japanese-English Character Dictionary", edited by Jack Halpern.
    halpern_kkd, // "Kodansha Kanji Dictionary", (2nd Ed. of the NJECD) edited by Jack Halpern.
    halpern_kkld, // "Kanji Learners Dictionary" (Kodansha) edited by Jack Halpern.
    halpern_kkld_2ed, // "Kanji Learners Dictionary" (Kodansha), 2nd edition (2013) edited by Jack Halpern.
    heisig, // "Remembering The  Kanji"  by  James Heisig.
    heisig6, // "Remembering The  Kanji, Sixth Ed."  by  James Heisig.
    gakken, // "A  New Dictionary of Kanji Usage" (Gakken)
    oneill_names, // "Japanese Names", by P.G. O'Neill.
    oneill_kk, // "Essential Kanji" by P.G. O'Neill.
    moro, // "Daikanwajiten" compiled by Morohashi. For some kanji two additional attributes are used: m_vol:  the volume of the dictionary in which the kanji is found, and m_page: the page number in the volume.
    henshall, // "A Guide To Remembering Japanese Characters" by Kenneth G.  Henshall.
    sh_kk, // "Kanji and Kana" by Spahn and Hadamitzky.
    sh_kk2, // "Kanji and Kana" by Spahn and Hadamitzky (2011 edition).
    sakade, // "A Guide To Reading and Writing Japanese" edited by Florence Sakade.
    jf_cards, // Japanese Kanji Flashcards, by Max Hodges and Tomoko Okazaki. (Series 1)
    henshall3, // "A Guide To Reading and Writing Japanese" 3rd edition, edited by Henshall, Seeley and De Groot.
    tutt_cards, // Tuttle Kanji Cards, compiled by Alexander Kask.
    crowley, // "The Kanji Way to Japanese Language Power" by Dale Crowley.
    kanji_in_context, // "Kanji in Context" by Nishiguchi and Kono.
    busy_people, // "Japanese For Busy People" vols I-III, published by the AJLT. The codes are the volume.chapter.
    kodansha_compact, // The "Kodansha Compact Kanji Guide".
    maniette, // Codes from Yves Maniette's "Les Kanjis dans la tete" French adaptation of Heisig.
}

impl FromStr for DicRefType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nelson_c"         => Ok(Self::nelson_c),
            "nelson_n"         => Ok(Self::nelson_n),
            "halpern_njecd"    => Ok(Self::halpern_njecd),
            "halpern_kkd"      => Ok(Self::halpern_kkd),
            "halpern_kkld"     => Ok(Self::halpern_kkld),
            "halpern_kkld_2ed" => Ok(Self::halpern_kkld_2ed),
            "heisig"           => Ok(Self::heisig),
            "heisig6"          => Ok(Self::heisig6),
            "gakken"           => Ok(Self::gakken),
            "oneill_names"     => Ok(Self::oneill_names),
            "oneill_kk"        => Ok(Self::oneill_kk),
            "moro"             => Ok(Self::moro),
            "henshall"         => Ok(Self::henshall),
            "sh_kk"            => Ok(Self::sh_kk),
            "sh_kk2"           => Ok(Self::sh_kk2),
            "sakade"           => Ok(Self::sakade),
            "jf_cards"         => Ok(Self::jf_cards),
            "henshall3"        => Ok(Self::henshall3),
            "tutt_cards"       => Ok(Self::tutt_cards),
            "crowley"          => Ok(Self::crowley),
            "kanji_in_context" => Ok(Self::kanji_in_context),
            "busy_people"      => Ok(Self::busy_people),
            "kodansha_compact" => Ok(Self::kodansha_compact),
            "maniette"         => Ok(Self::maniette),
            _ => Err(()),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct DicRef {
    pub typ: DicRefType, // <!ATTLIST dic_ref dr_type CDATA #REQUIRED> The dr_type defines the dictionary or reference book, etc. to which dic_ref element applies.
    pub index_number: String, // <!ELEMENT dic_ref (#PCDATA)> Each dic_ref contains an index number. The particular dictionary, etc. is defined by the dr_type attribute.
    pub moro_volume: u16,
    pub moro_page: u16,
}

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum QueryCodeType {
    skip, // Halpern's SKIP (System  of  Kanji  Indexing  by  Patterns) code. The  format is n-nn-nn.  See the KANJIDIC  documentation  for  a description of the code and restrictions on  the  commercial  use  of this data. [P]  There are also a number of misclassification codes, indicated by the "skip_misclass" attribute.
    sh_desc, // The descriptor codes for The Kanji Dictionary (Tuttle  1996) by Spahn and Hadamitzky. They are in the form nxnn.n,   e.g.  3k11.2, where the  kanji has 3 strokes in the  identifying radical, it is radical "k" in the SH  classification system, there are 11 other strokes, and it is  the 2nd kanji in the 3k11 sequence. (I am very grateful to  Mark Spahn for providing the list of these descriptor codes  for the kanji in this file.) [I]
    four_corner, // The "Four Corner" code for the kanji. This is a code  invented by Wang Chen in 1928. See the KANJIDIC documentation  for  an overview of  the Four Corner System. [Q]
    deroo, // The codes developed by the late Father Joseph De Roo, and  published in  his book "2001 Kanji" (Bonjinsha). Fr De Roo  gave his permission for these codes to be included. [DR]
    misclass, // A possible misclassification of the kanji according to one of the code types. (See the "Z" codes in the KANJIDIC documentation for more details.)
}

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum SkipMisclass {
    none,
    posn,
    stroke_count,
    stroke_and_posn,
    stroke_diff
}
impl Default for SkipMisclass {
    fn default() -> Self { SkipMisclass::none }
}

/// <!ELEMENT query_code (q_code+)> These codes contain information relating to the glyph, and can be used for finding a required kanji. The type of code is defined by the qc_type attribute.
#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct QueryCode {
    pub typ: QueryCodeType,
    pub value: String,
    pub skip_misclass: SkipMisclass,
}

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum ReadingType {
    pinyin, // The modern PinYin romanization of the Chinese reading of the kanji. The tones are represented by a concluding  digit. [Y]
    korean_r, // The romanized form of the Korean reading(s) of the  kanji.  The readings are in the (Republic of Korea) Ministry  of Education style of romanization. [W]
    korean_h, // The Korean reading(s) of the kanji in hangul.
    vietnam, // The Vietnamese readings supplied by Minh Chau Pham.
    ja_on, // The "on" Japanese reading of the kanji, in katakana.  Another attribute r_status, if present, will indicate with a value of "jy" whether the reading is approved for a "Jouyou kanji". (The r_status attribute is not currently used.) A further attribute on_type, if present,  will indicate with  a value of kan, go, tou or kan'you the type of on-reading. (The on_type attribute is not currently used.)
    ja_kun, // The "kun" Japanese reading of the kanji, usually in  hiragana.  Where relevant the okurigana is also included separated by a  ".". Readings associated with prefixes and suffixes are  marked with a "-". A second attribute r_status, if present,  will indicate with a value of "jy" whether the reading is  approved for a "Jouyou kanji". (The r_status attribute is  not currently used.)
}
impl FromStr for ReadingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pinyin"   => Ok(ReadingType::pinyin),
            "korean_r" => Ok(ReadingType::korean_r),
            "korean_h" => Ok(ReadingType::korean_h),
            "ja_on"    => Ok(ReadingType::ja_on),
            "ja_kun"   => Ok(ReadingType::ja_kun),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum OnType {
    none,
    kan,
    go,
    tou,
    kanyou,
}
impl Default for OnType {
    fn default() -> Self { OnType::none }
}
impl FromStr for OnType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kan"     => Ok(OnType::kan),
            "go"      => Ok(OnType::go),
            "tou"     => Ok(OnType::tou),
            "kan'you" => Ok(OnType::kanyou),
            _ => Err(()),
        }
    }
}

/// <!ELEMENT reading (#PCDATA)> The reading element contains the reading or pronunciation of the kanji.
#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Reading {
    pub value: String,
    pub typ: ReadingType, // <!ATTLIST reading r_type CDATA #REQUIRED> The r_type attribute defines the type of reading in the reading element.
    pub approved_for_joyou_kanji: bool, // <!ATTLIST reading r_status CDATA #IMPLIED> See under ja_on and ja_kun above.
    pub on_type: OnType, // <!ATTLIST reading r_status CDATA #IMPLIED> See under ja_on and ja_kun above.
}

/// <!ELEMENT meaning (#PCDATA)> The meaning associated with the kanji.
#[derive(Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Meaning {
    pub value: String, //< <!ELEMENT meaning (#PCDATA)> The meaning associated with the kanji.
    pub lang: String, //< <!ATTLIST meaning m_lang CDATA #IMPLIED> The m_lang attribute defines the target language of the meaning. It will be coded using the two-letter language code from the ISO 639-1 standard. When absent, the value "en" (i.e. English) is implied. [{}]
}

/// <!ELEMENT rmgroup (reading*, meaning*)>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct ReadingMeaningGroup {
    pub readings: Vec<Reading>,
    pub meanings: Vec<Meaning>,
    pub nanori: Vec<String>, // <!ELEMENT nanori (#PCDATA)> The nanori element contains the nanori readings of the kanji. These are readings that are not normally used in the Japanese language, but are used in names and place names. [N]
}

/// <!ELEMENT character (literal,codepoint, radical, misc, dic_number?, query_code?, reading_meaning?)*>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Character {
    pub literal: String, // <!ELEMENT literal (#PCDATA)> The literal element contains the actual kanji character.
    pub codepoint: Codepoint,
    pub radical: Radical,
    pub misc: Misc,

    pub dic_number: Vec<DicRef>,
    pub query_code: Vec<QueryCode>,
    pub reading_meaning_groups: Vec<ReadingMeaningGroup>, // <!ELEMENT reading_meaning (rmgroup*, nanori*)> The readings for the kanji in several languages, and the meanings, also in several languages. The readings and meanings are grouped to enable the handling of the situation where the meaning is differentiated by reading. [T1]
}

/// <!ELEMENT header (file_version,database_version,date_of_creation)>
#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Header {
    pub file_version: String,
    pub database_version: String,
    pub date_of_creation: String,
}

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct Kanjidic {
    /// <!ELEMENT header (file_version,database_version,date_of_creation)>
    pub header: Header,
    /// <!ELEMENT character (literal,codepoint, radical, misc, dic_number?, query_code?, reading_meaning?)*>
    pub characters: Vec<Character>,
}
