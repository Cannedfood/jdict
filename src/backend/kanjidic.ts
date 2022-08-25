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
	type: QueryCodeType;
	value: string;
	skip_misclassification?: SkipMisclassification
}

export interface Reading {
	type: ReadingType;
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
	type: VariantType;
	value: string;
}

export interface Character extends Misc {
	literal: string;
	codepoint: Codepoint;
	radical: Radical;
	query_code?: QueryCode[];
	reading_meaning_groups?: ReadingMeaningGroup[];
	nanori?: string[];
}
