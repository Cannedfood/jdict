export interface Kanji {
    value: string;
    infos?: string[];
    priorities?: string[];
}

export interface Reading {
    value: string;
    not_actual_reading?: boolean;
    restrict_kanji?: string[];
    infos?: string[];
    priorities?: string[];
}

export interface SourceLanguage {
    word: string;
    lang?: string;
    partial?: boolean;
    waseieigo?: boolean;
}

export interface Gloss {
    content: string;
    lang?: string;
    gender?: string;
    type?: string;
    highlight?: boolean;
}

export interface Sentence {
    value: string;
    lang?: string;
}

export interface Example {
    source: string;
    form_in_example: string;
    sentences: Sentence[];
}

export interface Sense {
    restrict_kanji?: string[];
    restrict_reading?: string[];
    part_of_speech_tags?: string[];
    cross_references?: string[];
    antonyms?: string[];
    fields?: string[];
    misc_info?: string[];
    sense_info?: string[];

    lang_origin?: SourceLanguage[];
    dialects?: string[];

    glosses: Gloss[];
    examples?: Example[];
}

export interface Entry {
    id: string;
    kanji: Kanji[];
    readings: Reading[];
    senses: Sense[];
}