import { Option, bool, u8, Vec } from "./rust_compat";

export interface KanjiVG {
    kanji: Kanji[];
}

export interface Kanji {
    kanji: string,
    original: Option<string>,
    phon: Option<string>,
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
    path: string,
    typ: string,
}

export type Position = string;
export type Radical = string;

export function allPaths(kanji: Kanji): Stroke[] {
    return [
        ...(kanji.strokes ?? []),
        ...(kanji.parts?.flatMap(p => allPaths(p)) ?? []),
    ];
}
