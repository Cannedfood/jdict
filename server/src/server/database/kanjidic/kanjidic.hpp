#pragma once

#include <string>
#include <utility>
#include <vector>
#include <bitset>
#include <map>

namespace jdict {

/// <!ELEMENT kanjidic2 (header,character*)>
struct kanjidic {
	/// <!ELEMENT header (file_version,database_version,date_of_creation)>
	struct header_t {
		std::string file_version; //!< <!ELEMENT file_version (#PCDATA)> The single header element will contain identification information about the version of the file
		std::string database_version; //!< <!ELEMENT database_version (#PCDATA)> This field denotes the version of kanjidic2 structure, as more than one version may exist.
		std::string date_of_creation; //!< <!ELEMENT date_of_creation (#PCDATA)> The version of the file, in the format YYYY-NN, where NN will be a number starting with 01 for the first version released in a calendar year, then increasing for each version in that year.
	};

	enum class variant_type_t {
		jis208, //!< In JIS X 0208 - kuten coding
		jis212, //!< In JIS X 0212 - kuten coding
		jis213, //!< In JIS X 0213 - kuten coding (most of the above relate to "shinjitai/kyuujitai" alternative character glyphs)
		deroo, //!< De Roo number - numeric
		njecd, //!< Halpern NJECD index number - numeric
		s_h, //!< The Kanji Dictionary (Spahn & Hadamitzky) - descriptor
		nelson_c, //!< "Classic" Nelson - numeric
		oneill, //!< Japanese Names (O'Neill) - numeric
		ucs, //!< Unicode codepoint- hex
	};

	struct variant_t {
		variant_type_t type = variant_type_t::deroo;
		std::string value;
	};

	/// <!ELEMENT misc (grade?, stroke_count+, variant*, freq?, rad_name*,jlpt?)>
	struct misc_t {
		uint8_t                  grade = 0; //!< <!ELEMENT grade (#PCDATA)> The kanji grade level. 1 through 6 indicates a Kyouiku kanji and the grade in which the kanji is taught in Japanese schools. 8 indicates it is one of the remaining Jouyou Kanji to be learned in junior high school. 9 indicates it is a Jinmeiyou (for use in names) kanji which in addition  to the Jouyou kanji are approved for use in family name registers and other official documents. 10 also indicates a Jinmeiyou kanji which is a variant of a Jouyou kanji. [G]
		std::vector<uint8_t>     stroke_count; //!< <!ELEMENT stroke_count (#PCDATA)> The stroke count of the kanji, including the radical. If more than one, the first is considered the accepted count, while subsequent ones are common miscounts. (See Appendix E. of the KANJIDIC documentation for some of the rules applied when counting strokes in some of the radicals.) [S]
		std::vector<variant_t>   variant; //!< <!ELEMENT variant (#PCDATA)> Either a cross-reference code to another kanji, usually regarded as a variant, or an alternative indexing code for the current kanji. The type of variant is given in the var_type attribute.
		uint32_t                 freq = 0; //!< <!ELEMENT freq (#PCDATA)> A frequency-of-use ranking. The 2,500 most-used characters have a ranking; those characters that lack this field are not ranked. The frequency is a number from 1 to 2,500 that expresses the relative frequency of occurrence of a character in modern Japanese. This is based on a survey in newspapers, so it is biassed towards kanji used in newspaper articles. The discrimination between the less frequently used kanji is not strong. (Actually there are 2,501 kanji ranked as there was a tie.)
		std::vector<std::string> rad_name; //!< <!ELEMENT rad_name (#PCDATA)> When the kanji is itself a radical and has a name, this element contains the name (in hiragana.) [T2]
		uint8_t                  jlpt = 0; //!< <!ELEMENT jlpt (#PCDATA)> The (former) Japanese Language Proficiency test level for this kanji. Values range from 1 (most advanced) to 4 (most elementary). This field does not appear for kanji that were not required for any JLPT level. Note that the JLPT test levels changed in 2010, with a new 5-level system (N1 to N5) being introduced. No official kanji lists are available for the new levels. The new levels are regarded as being similar to the old levels except that the old level 2 is now divided between N2 and N3.
	};

	/// <!ELEMENT codepoint (cp_value+)> The codepoint element states the code of the character in the various character set standards.
	struct codepoint_t {
		std::string jis208;
		std::string jis212;
		std::string jis213;
		std::string ucs;

		operator bool() const noexcept { return !(jis208.empty() && jis212.empty() && jis213.empty() && ucs.empty()); }
	};

	/// <!ELEMENT rad_value (#PCDATA)> The radical number, in the range 1 to 214. The particular classification type is stated in the rad_type attribute.
	struct radical_t {
		uint16_t classical = 0;
		uint16_t nelson_c  = 0;
		operator bool() const noexcept { return classical != 0 || nelson_c != 0; }
	};

	enum class dic_ref_type_t {
		nelson_c, //!< "Modern Reader's Japanese-English Character Dictionary", edited by Andrew Nelson (now published as the "Classic"  Nelson).
		nelson_n, //!< "The New Nelson Japanese-English Character Dictionary", edited by John Haig.
		halpern_njecd, //!< "New Japanese-English Character Dictionary", edited by Jack Halpern.
		halpern_kkd, //!< "Kodansha Kanji Dictionary", (2nd Ed. of the NJECD) edited by Jack Halpern.
		halpern_kkld, //!< "Kanji Learners Dictionary" (Kodansha) edited by Jack Halpern.
		halpern_kkld_2ed, //!< "Kanji Learners Dictionary" (Kodansha), 2nd edition (2013) edited by Jack Halpern.
		heisig, //!< "Remembering The  Kanji"  by  James Heisig.
		heisig6, //!< "Remembering The  Kanji, Sixth Ed."  by  James Heisig.
		gakken, //!< "A  New Dictionary of Kanji Usage" (Gakken)
		oneill_names, //!< "Japanese Names", by P.G. O'Neill.
		oneill_kk, //!< "Essential Kanji" by P.G. O'Neill.
		moro, //!< "Daikanwajiten" compiled by Morohashi. For some kanji two additional attributes are used: m_vol:  the volume of the dictionary in which the kanji is found, and m_page: the page number in the volume.
		henshall, //!< "A Guide To Remembering Japanese Characters" by Kenneth G.  Henshall.
		sh_kk, //!< "Kanji and Kana" by Spahn and Hadamitzky.
		sh_kk2, //!< "Kanji and Kana" by Spahn and Hadamitzky (2011 edition).
		sakade, //!< "A Guide To Reading and Writing Japanese" edited by Florence Sakade.
		jf_cards, //!< Japanese Kanji Flashcards, by Max Hodges and Tomoko Okazaki. (Series 1)
		henshall3, //!< "A Guide To Reading and Writing Japanese" 3rd edition, edited by Henshall, Seeley and De Groot.
		tutt_cards, //!< Tuttle Kanji Cards, compiled by Alexander Kask.
		crowley, //!< "The Kanji Way to Japanese Language Power" by Dale Crowley.
		kanji_in_context, //!< "Kanji in Context" by Nishiguchi and Kono.
		busy_people, //!< "Japanese For Busy People" vols I-III, published by the AJLT. The codes are the volume.chapter.
		kodansha_compact, //!< The "Kodansha Compact Kanji Guide".
		maniette, //!< Codes from Yves Maniette's "Les Kanjis dans la tete" French adaptation of Heisig.
	};

	struct dic_ref_t {
		dic_ref_type_t type = dic_ref_type_t::busy_people; //!< <!ATTLIST dic_ref dr_type CDATA #REQUIRED> The dr_type defines the dictionary or reference book, etc. to which dic_ref element applies.
		std::string    index_number; //!< <!ELEMENT dic_ref (#PCDATA)> Each dic_ref contains an index number. The particular dictionary, etc. is defined by the dr_type attribute.
		uint16_t       moro_volume = 0;
		uint16_t       moro_page = 0;
	};

	enum class query_code_type_t {
		skip, //!< Halpern's SKIP (System  of  Kanji  Indexing  by  Patterns) code. The  format is n-nn-nn.  See the KANJIDIC  documentation  for  a description of the code and restrictions on  the  commercial  use  of this data. [P]  There are also a number of misclassification codes, indicated by the "skip_misclass" attribute.
		sh_desc, //!< The descriptor codes for The Kanji Dictionary (Tuttle  1996) by Spahn and Hadamitzky. They are in the form nxnn.n,   e.g.  3k11.2, where the  kanji has 3 strokes in the  identifying radical, it is radical "k" in the SH  classification system, there are 11 other strokes, and it is  the 2nd kanji in the 3k11 sequence. (I am very grateful to  Mark Spahn for providing the list of these descriptor codes  for the kanji in this file.) [I]
		four_corner, //!< The "Four Corner" code for the kanji. This is a code  invented by Wang Chen in 1928. See the KANJIDIC documentation  for  an overview of  the Four Corner System. [Q]
		deroo, //!< The codes developed by the late Father Joseph De Roo, and  published in  his book "2001 Kanji" (Bonjinsha). Fr De Roo  gave his permission for these codes to be included. [DR]
		misclass, //!< A possible misclassification of the kanji according to one of the code types. (See the "Z" codes in the KANJIDIC documentation for more details.)
	};

	enum class skip_misclass_t {
		none,
		posn,
		stroke_count,
		stroke_and_posn,
		stroke_diff
	};

	/// <!ELEMENT query_code (q_code+)> These codes contain information relating to the glyph, and can be used for finding a required kanji. The type of code is defined by the qc_type attribute.
	struct query_code_t {
		query_code_type_t type = query_code_type_t::deroo;
		std::string       value;
		skip_misclass_t   skip_misclass;
	};

	enum class reading_type_t {
		pinyin, //!< The modern PinYin romanization of the Chinese reading of the kanji. The tones are represented by a concluding  digit. [Y]
		korean_r, //!< The romanized form of the Korean reading(s) of the  kanji.  The readings are in the (Republic of Korea) Ministry  of Education style of romanization. [W]
		korean_h, //!< The Korean reading(s) of the kanji in hangul.
		vietnam, //!< The Vietnamese readings supplied by Minh Chau Pham.
		ja_on, //!< The "on" Japanese reading of the kanji, in katakana.  Another attribute r_status, if present, will indicate with a value of "jy" whether the reading is approved for a "Jouyou kanji". (The r_status attribute is not currently used.) A further attribute on_type, if present,  will indicate with  a value of kan, go, tou or kan'you the type of on-reading. (The on_type attribute is not currently used.)
		ja_kun, //!< The "kun" Japanese reading of the kanji, usually in  hiragana.  Where relevant the okurigana is also included separated by a  ".". Readings associated with prefixes and suffixes are  marked with a "-". A second attribute r_status, if present,  will indicate with a value of "jy" whether the reading is  approved for a "Jouyou kanji". (The r_status attribute is  not currently used.)
	};

	enum class on_type_t {
		none,
		kan,
		go,
		tou,
		kanyou,
	};

	/// <!ELEMENT reading (#PCDATA)> The reading element contains the reading or pronunciation of the kanji.
	struct reading_t {
		std::string    value;
		reading_type_t type = reading_type_t::ja_kun; //!< <!ATTLIST reading r_type CDATA #REQUIRED> The r_type attribute defines the type of reading in the reading element.
		bool           approved_for_joyou_kanji = false; //!< <!ATTLIST reading r_status CDATA #IMPLIED> See under ja_on and ja_kun above.
		on_type_t      on_type = on_type_t::none; //!< <!ATTLIST reading r_status CDATA #IMPLIED> See under ja_on and ja_kun above.
	};

	struct meaning_t {
		std::string value; //!< <!ELEMENT meaning (#PCDATA)> The meaning associated with the kanji.
		std::string lang; //!< <!ATTLIST meaning m_lang CDATA #IMPLIED> The m_lang attribute defines the target language of the meaning. It will be coded using the two-letter language code from the ISO 639-1 standard. When absent, the value "en" (i.e. English) is implied. [{}]
	};
	/// <!ELEMENT rmgroup (reading*, meaning*)>
	struct rm_group_t {
		std::vector<reading_t> readings;
		std::vector<meaning_t> meanings;
	};

	/// <!ELEMENT character (literal,codepoint, radical, misc, dic_number?, query_code?, reading_meaning?)*>
	struct character_t {
		std::string literal; //!< <!ELEMENT literal (#PCDATA)> The character itself in UTF8 coding.
		codepoint_t codepoint;
		radical_t   radical;
		misc_t      misc;

		std::vector<dic_ref_t>    dic_number;
		std::vector<query_code_t> query_code;
		std::vector<rm_group_t>   reading_meaning_groups; /// <!ELEMENT reading_meaning (rmgroup*, nanori*)> The readings for the kanji in several languages, and the meanings, also in several languages. The readings and meanings are grouped to enable the handling of the situation where the meaning is differentiated by reading. [T1]
		std::vector<std::string>  nanori; //!< <!ELEMENT nanori (#PCDATA)> Japanese readings that are now only associated with names.
	};
	header_t                 header;
	std::vector<character_t> characters;

	static kanjidic parse_file(const char* path);
};

const char* to_string(kanjidic::variant_type_t) noexcept;
const char* to_string(kanjidic::dic_ref_type_t) noexcept;
const char* to_string(kanjidic::query_code_type_t) noexcept;
const char* to_string(kanjidic::skip_misclass_t) noexcept;
const char* to_string(kanjidic::reading_type_t) noexcept;
const char* to_string(kanjidic::on_type_t) noexcept;

} // namespace jdict
