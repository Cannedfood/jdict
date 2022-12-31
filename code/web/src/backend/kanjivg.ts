import { Option, bool, u8, Vec } from "./rust_compat";

export interface KanjiVG {
    kanji: Kanji[];
}

export interface Kanji {
    kanji: String,
    original: Option<String>,
    phon: Option<String>,
    position: Option<Position>,
    partial: bool,
    number: Option<u8>,
    part: Option<u8>,
    radical_form: bool,
    radical: Option<Radical>,
    strokes: Vec<Stroke>,
    parts: Vec<Kanji>,
    trad_form: bool,
    variant: bool,
}

export interface Stroke {
    path: String,
    typ: String,
}

export type Position = string;
export type Radical = string;
