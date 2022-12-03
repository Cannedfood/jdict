export interface Codepoint {
	jis208?: string;
	jis212?: string;
	jis213?: string;
	ucs?: string;
}

export const CodepointTypeValues = [
	"jis208",
	"jis212",
	"jis213",
	"ucs",
] as Array<keyof Codepoint>;

export interface Radical {
	classical?: number;
	nelson_c?: number;
}

export interface Misc {
	grade?: number;
	stroke_count?: number[];
	variant?: Variant[];
	freq?: number;
	rad_name?: string[];
	jlpt?: number;
}

export type QueryCodeType =
	"skip" |
	"sh_desc" |
	"four_corner" |
	"deroo" |
	"misclass";

export type SkipMisclassification =
	"posn" |
	"stroke_count" |
	"stroke_and_posn" |
	"stroke_diff";

export type ReadingType =
	"pinyin" |
	"korean_r" |
	"korean_h" |
	"vietnam" |
	"ja_on" |
	"ja_kun";

export type OnType =
	"none" |
	"kan" |
	"go" |
	"tou" |
	"kan'you";

export interface QueryCode {
	typ: QueryCodeType;
	value: string;
	skip_misclassification?: SkipMisclassification
}

export interface Reading {
	typ: ReadingType;
	value: string;
	approved_for_joyou_kanji?: true;
	on_type: OnType;
}

export interface Meaning {
	value: string;
	lang?: string;
}

export interface ReadingMeaningGroup {
	readings?: Reading[];
	meanings?: Meaning[];
	nanori?: string[];
}

export type VariantType =
	"jis208" |
	"jis212" |
	"jis213" |
	"deroo" |
	"njecd" |
	"s_h" |
	"nelson_c" |
	"oneill" |
	"ucs";

export interface Variant {
	typ: VariantType;
	value: string;
}

export type DicRefType =
	'nelson_c' |
	'nelson_n' |
	'halpern_njecd' |
	'halpern_kkd' |
	'halpern_kkld' |
	'halpern_kkld_2ed' |
	'heisig' |
	'heisig6' |
	'gakken' |
	'oneill_names' |
	'oneill_kk' |
	'moro' |
	'henshall' |
	'sh_kk' |
	'sh_kk2' |
	'sakade' |
	'jf_cards' |
	'henshall3' |
	'tutt_cards' |
	'crowley' |
	'kanji_in_context' |
	'busy_people' |
	'kodansha_compact' |
	'maniette';

export interface DicRef {
    typ: DicRefType, // <!ATTLIST dic_ref dr_type CDATA #REQUIRED> The dr_type defines the dictionary or reference book, etc. to which dic_ref element applies.
    index_number: string, // <!ELEMENT dic_ref (#PCDATA)> Each dic_ref contains an index number. The particular dictionary, etc. is defined by the dr_type attribute.
    moro_volume: number,
    moro_page: number,
}

export interface Character {
    literal: string, // <!ELEMENT literal (#PCDATA)> The literal element contains the actual kanji character.
    codepoint: Codepoint,
    radical: Radical,
    misc: Misc,

    dic_number: Array<DicRef>,
    query_code: Array<QueryCode>,
    reading_meaning_groups: Array<ReadingMeaningGroup>, // <!ELEMENT reading_meaning (rmgroup*, nanori*)> The readings for the kanji in several languages, and the meanings, also in several languages. The readings and meanings are grouped to enable the handling of the situation where the meaning is differentiated by reading. [T1]
}
