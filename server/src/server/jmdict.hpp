#pragma once

#include <string>
#include <vector>

namespace jdict {

/// <!ELEMENT JMdict (entry*)>
struct jmdict {
    /// <!ELEMENT k_ele (keb, ke_inf*, ke_pri*)> The kanji element, or in its absence, the reading element, is the defining component of each entry. The overwhelming majority of entries will have a single kanji element associated with a word in Japanese. Where there are multiple kanji elements within an entry, they will be orthographical variants of the same word, either using variations in  , or alternative and equivalent kanji. Common "mis-spellings" may be included, provided they are associated with appropriate information fields. Synonyms are not included; they may be indicated in the cross-reference field associated with the sense element.
    struct kanji {
        std::string value; //!< <!ELEMENT keb (#PCDATA)> This element will contain a word or short phrase in Japanese which is written using at least one non-kana character (usually kanji, but can be other characters). The valid characters are kanji, kana, related characters such as chouon and kurikaeshi, and in exceptional cases, letters from other alphabets.
        std::vector<std::string> infos; //!< <!ELEMENT ke_inf (#PCDATA)> This is a coded information field related specifically to the orthography of the keb, and will typically indicate some unusual aspect, such as okurigana irregularity.

        /// <!ELEMENT ke_pri (#PCDATA)>
        ///     This and the equivalent re_pri field are provided to record
        ///     information about the relative priority of the entry,  and consist
        ///     of codes indicating the word appears in various references which
        ///     can be taken as an indication of the frequency with which the word
        ///     is used. This field is intended for use either by applications which
        ///     want to concentrate on entries of  a particular priority, or to
        ///     generate subset files.
        ///     The current values in this field are:
        ///     - news1/2: appears in the "wordfreq" file compiled by Alexandre Girardi
        ///     from the Mainichi Shimbun. (See the Monash ftp archive for a copy.)
        ///     Words in the first 12,000 in that file are marked "news1" and words
        ///     in the second 12,000 are marked "news2".
        ///     - ichi1/2: appears in the "Ichimango goi bunruishuu", Senmon Kyouiku
        ///     Publishing, Tokyo, 1998.  (The entries marked "ichi2" were
        ///     demoted from ichi1 because they were observed to have low
        ///     frequencies in the WWW and newspapers.)
        ///     - spec1 and spec2: a small number of words use this marker when they
        ///     are detected as being common, but are not included in other lists.
        ///     - gai1/2: common loanwords, based on the wordfreq file.
        ///     - nfxx: this is an indicator of frequency-of-use ranking in the
        ///     wordfreq file. "xx" is the number of the set of 500 words in which
        ///     the entry can be found, with "01" assigned to the first 500, "02"
        ///     to the second, and so on. (The entries with news1, ichi1, spec1, spec2
        ///     and gai1 values are marked with a "(P)" in the EDICT and EDICT2
        ///     files.)
        ///
        ///     The reason both the kanji and reading elements are tagged is because
        ///     on occasions a priority is only associated with a particular
        ///     kanji/reading pair.
        std::vector<std::string> priorities;
    };

    /// <!ELEMENT r_ele (reb, re_nokanji?, re_restr*, re_inf*, re_pri*)> The reading element typically contains the valid readings of the word(s) in the kanji element using modern kanadzukai. Where there are multiple reading elements, they will typically be alternative readings of the kanji element. In the absence of a kanji element, i.e. in the case of a word or phrase written entirely in kana, these elements will define the entry.
    struct reading {
        std::string value; //!< <!ELEMENT reb (#PCDATA)> This element content is restricted to kana and related Tharacters such as chouon and kurikaeshi. Kana usage will be Tonsistent between the keb and reb elements; e.g. if the keb Tontains katakana, so too will the reb.
        bool not_actual_reading = false; //!< <!ELEMENT re_nokanji (#PCDATA)>This element, which will usually have a null value, indicates that the reb, while associated with the keb, cannot be regarded as a true reading of the kanji. It is typically used for words such as foreign place names, gairaigo which can be in kanji or katakana, etc.
        std::vector<std::string> restrict_kanji; //!< <!ELEMENT re_restr (#PCDATA)> This element is used to indicate when the reading only applies to a subset of the keb elements in the entry. In its absence, all readings apply to all kanji elements. The contents of this element must exactly match those of one of the keb elements.
        std::vector<std::string> infos; //!< <!ELEMENT re_inf (#PCDATA)> General coded information pertaining to the specific reading. Typically it will be used to indicate some unusual aspect of the reading.
        std::vector<std::string> priorities; //!< <!ELEMENT re_inf (#PCDATA)> General coded information pertaining to the specific reading. Typically it will be used to indicate some unusual aspect of the reading.
    };

    /// <!ELEMENT sense (stagk*, stagr*, pos*, xref*, ant*, field*, misc*, s_inf*, lsource*, dial*, gloss*, example*)> The sense element will record the translational equivalent of the Japanese word, plus other related information. Where there are several distinctly different meanings of the word, multiple sense elements will be employed.
    struct sense {
        std::vector<std::string> restrict_kanji;   //!< <!ELEMENT stagk (#PCDATA)> These elements, if present, indicate that the sense is restricted to the lexeme represented by the keb and/or reb.
        std::vector<std::string> restrict_reading; //!< <!ELEMENT stagr (#PCDATA)> These elements, if present, indicate that the sense is restricted to the lexeme represented by the keb and/or reb.
        std::vector<std::string> part_of_speech_tags; //!< <!ELEMENT pos (#PCDATA)> Part-of-speech information about the entry/sense. Should use appropriate entity codes. In general where there are multiple senses in an entry, the part-of-speech of an earlier sense will apply to later senses unless there is a new part-of-speech indicated.
        std::vector<std::string> cross_references; //!< <!ELEMENT xref (#PCDATA)*> This element is used to indicate a cross-reference to another entry with a similar or related meaning or sense. The content of this element is typically a keb or reb element in another entry. In some cases a keb will be followed by a reb and/or a sense number to provide a precise target for the cross-reference. Where this happens, a JIS "centre-dot" (0x2126) is placed between the components of the cross-reference. The target keb or reb must not contain a centre-dot.
        std::vector<std::string> antonyms; //!< <!ELEMENT ant (#PCDATA)*> This element is used to indicate another entry which is an antonym of the current entry/sense. The content of this element must exactly match that of a keb or reb element in another entry.
        std::vector<std::string> fields; //!< <!ELEMENT field (#PCDATA)> Information about the field of application of the entry/sense. When absent, general application is implied. Entity coding for specific fields of application.
        std::vector<std::string> misc_info; //!< <!ELEMENT misc (#PCDATA)> This element is used for other relevant information about the entry/sense. As with part-of-speech, information will usually apply to several senses.
        std::vector<std::string> sense_info; //!< <!ELEMENT s_inf (#PCDATA)> The sense-information elements provided for additional information to be recorded about a sense. Typical usage would be to indicate such things as level of currency of a sense, the regional variations, etc.

        /// <!ELEMENT lsource (#PCDATA)> This element records the information about the source language(s) of a loan-word/gairaigo. If the source language is other than English, the language is indicated by the xml:lang attribute. The element value (if any) is the source word or phrase.
        struct source_language {
            std::string word;
            std::string language; //!< <!ATTLIST lsource xml:lang CDATA "eng"> The xml:lang attribute defines the language(s) from which a loanword is drawn.  It will be coded using the three-letter language code from the ISO 639-2 standard. When absent, the value "eng" (i.e. English) is the default value. The bibliographic (B) codes are used.
            bool partial = false; //!< <!ATTLIST lsource ls_type CDATA #IMPLIED> The ls_type attribute indicates whether the lsource element fully or partially describes the source word or phrase of the loanword. If absent, it will have the implied value of "full". Otherwise it will contain "part".
            bool waseieigo = false; //!< <!ATTLIST lsource ls_wasei CDATA #IMPLIED> The ls_wasei attribute indicates that the Japanese word has been constructed from words in the source language, and not from an actual phrase in that language. Most commonly used to indicate "waseieigo".
        };
        std::vector<source_language> lang_origin;
        std::vector<std::string> dialects;

        /// <!ELEMENT gloss (#PCDATA | pri)*> Within each sense will be one or more "glosses", i.e. target-language words or phrases which are equivalents to the Japanese word. This element would normally be present, however it may be omitted in entries which are purely for a cross-reference.
        struct gloss {
            std::string content;
            std::string language; //!< <!ATTLIST gloss xml:lang CDATA "eng"> The xml:lang attribute defines the target language of the gloss. It will be coded using the three-letter language code from the ISO 639 standard. When absent, the value "eng" (i.e. English) is the default value.
            std::string gender; //!< <!ATTLIST gloss g_gend CDATA #IMPLIED> The g_gend attribute defines the gender of the gloss (typically a noun in the target language. When absent, the gender is either not relevant or has yet to be provided.
            std::string type; //!< <!ATTLIST gloss g_type CDATA #IMPLIED> The g_type attribute specifies that the gloss is of a particular type, e.g. "lit" (literal), "fig" (figurative), "expl" (explanation).
            bool highlight; //!< <!ELEMENT pri (#PCDATA)> These elements highlight particular target-language words which are strongly associated with the Japanese word. The purpose is to establish a set of target-language words which can effectively be used as head-words in a reverse target-language/Japanese relationship.
        };
        std::vector<gloss> glosses;

        /// <!ELEMENT example (ex_srce,ex_text,ex_sent+)> The example elements contain a Japanese sentence using the term associated with the entry, and one or more translations of that sentence. Within the element, the ex_srce element will indicate the source of the sentences (typically the sequence number in the Tatoeba Project), the ex_text element will contain the form of the term in the Japanese sentence, and the ex_sent elements contain the example sentences.
        struct example {
            std::string source; //!< <!ELEMENT ex_srce (#PCDATA)> typically the sequence number in the Tatoeba Project
            std::string form_in_example; //!< <!ELEMENT ex_text (#PCDATA)> the ex_text element will contain the form of the term in the Japanese sentence (so you can highlight it, probably)
            /// <!ELEMENT ex_sent (#PCDATA)>
            struct sentence {
                std::string language;
                std::string value;
            };
            std::vector<sentence> sentences;
        };
        std::vector<example> examples;
    };

    /// <!ELEMENT entry (ent_seq, k_ele*, r_ele+, sense+)> Entries consist of kanji elements, reading elements, general information and sense elements. Each entry must have at least one reading element and one sense element. Others are optional.
    struct entry {
        std::string          sequence; //!< <!ELEMENT ent_seq (#PCDATA)> A unique numeric sequence number for each entry
        std::vector<kanji>   kanji;
        std::vector<reading> readings;
        std::vector<sense>   senses;
    };

    std::vector<entry> entries;

    static jmdict parse_file(const char* path);
};


} // namespace jdict