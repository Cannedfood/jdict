pub struct Entry {
    pub romaji:   &'static str,
    pub hiragana: &'static str,
    pub katakana: &'static str,
}
const fn entry(romaji: &'static str, hiragana: &'static str, katakana: &'static str) -> Entry {
    Entry { romaji, hiragana, katakana }
}

const BASIC_ENTRIES: [Entry; _] = [
    entry("a", "あ", "ア"),
    entry("i", "い", "イ"),
    entry("u", "う", "ウ"),
    entry("e", "え", "エ"),
    entry("o", "お", "オ"),
    entry("ka", "か", "カ"),
    entry("ki", "き", "キ"),
    entry("ku", "く", "ク"),
    entry("ke", "け", "ケ"),
    entry("ko", "こ", "コ"),
    entry("sa", "さ", "サ"),
    entry("si", "し", "シ"),
    entry("su", "す", "ス"),
    entry("se", "せ", "セ"),
    entry("so", "そ", "ソ"),
    entry("ta", "た", "タ"),
    entry("ti", "ち", "チ"),
    entry("tu", "つ", "ツ"),
    entry("te", "て", "テ"),
    entry("to", "と", "ト"),
    entry("na", "な", "ナ"),
    entry("ni", "に", "ニ"),
    entry("nu", "ぬ", "ヌ"),
    entry("ne", "ね", "ネ"),
    entry("no", "の", "ノ"),
    entry("ha", "は", "ハ"),
    entry("hi", "ひ", "ヒ"),
    entry("hu", "ふ", "フ"),
    entry("he", "へ", "ヘ"),
    entry("ho", "ほ", "ホ"),
    entry("ma", "ま", "マ"),
    entry("mi", "み", "ミ"),
    entry("mu", "む", "ム"),
    entry("me", "め", "メ"),
    entry("mo", "も", "モ"),
    entry("ra", "ら", "ラ"),
    entry("ri", "り", "リ"),
    entry("ru", "る", "ル"),
    entry("re", "れ", "レ"),
    entry("ro", "ろ", "ロ"),
    entry("wa", "わ", "ワ"),
    entry("wi", "ゐ", "ヰ"),
    entry("we", "ゑ", "ヱ"),
    entry("wo", "を", "ヲ"),
    entry("n",  "ん", "ン"),
    entry("ga", "が", "ガ"),
    entry("gi", "ぎ", "ギ"),
    entry("gu", "ぐ", "グ"),
    entry("ge", "げ", "ゲ"),
    entry("go", "ご", "ゴ"),
    entry("za", "ざ", "ザ"),
    entry("zi", "じ", "ジ"),
    entry("zu", "ず", "ズ"),
    entry("ze", "ぜ", "ゼ"),
    entry("zo", "ぞ", "ゾ"),
    entry("da", "だ", "ダ"),
    entry("di", "ぢ", "ヂ"),
    entry("du", "づ", "ヅ"),
    entry("de", "で", "デ"),
    entry("do", "ど", "ド"),
    entry("ba", "ば", "バ"),
    entry("bi", "び", "ビ"),
    entry("bu", "ぶ", "ブ"),
    entry("be", "べ", "ベ"),
    entry("bo", "ぼ", "ボ"),
    entry("pa", "ぱ", "パ"),
    entry("pi", "ぴ", "ピ"),
    entry("pu", "ぷ", "プ"),
    entry("pe", "ぺ", "ペ"),
    entry("po", "ぽ", "ポ"),
];