#include "./jmdict_index.hpp"

#include "./timer.hpp"
#include <set>
#include <stdio.h>

namespace jdict {

template<class T>
static bool contains(std::vector<T> const& vec, T v) {
    for(auto& vv : vec) {
        if(vv == v) return true;
    }
    return false;
}

static std::string_view snip(std::string_view& s, int n) {
    auto result = s.substr(0, n);
    s.remove_prefix(n);
    return result;
}

static bool try_snip(std::string_view& s, std::string_view c) {
    if(s.starts_with(c)) {
        s.remove_prefix(c.size());
        return true;
    }
    return false;
}

static bool try_snip(std::string_view& s, char8_t const* c) {
    return try_snip(s, std::string_view((const char*)c, strlen((const char*)c)));
}

static std::string_view snip_romaji(std::string_view& s) {
    // Hiragana
    if(try_snip(s, u8"きゃ")) return "kya";
    if(try_snip(s, u8"きゅ")) return "kyu";
    if(try_snip(s, u8"きょ")) return "kyo";
    if(try_snip(s, u8"しゃ")) return "sha";
    if(try_snip(s, u8"しゅ")) return "shu";
    if(try_snip(s, u8"しょ")) return "sho";
    if(try_snip(s, u8"ちゃ")) return "cha";
    if(try_snip(s, u8"ちゅ")) return "chu";
    if(try_snip(s, u8"ちょ")) return "cho";
    if(try_snip(s, u8"にゃ")) return "nya";
    if(try_snip(s, u8"にゅ")) return "nyu";
    if(try_snip(s, u8"にょ")) return "nyo";
    if(try_snip(s, u8"ひゃ")) return "hya";
    if(try_snip(s, u8"ひゅ")) return "hyu";
    if(try_snip(s, u8"ひょ")) return "hyo";
    if(try_snip(s, u8"みゃ")) return "mya";
    if(try_snip(s, u8"みゅ")) return "myu";
    if(try_snip(s, u8"みょ")) return "myo";
    if(try_snip(s, u8"りゃ")) return "rya";
    if(try_snip(s, u8"りゅ")) return "ryu";
    if(try_snip(s, u8"りょ")) return "ryo";
    if(try_snip(s, u8"ぎゃ")) return "gya";
    if(try_snip(s, u8"ぎゅ")) return "gyu";
    if(try_snip(s, u8"ぎょ")) return "gyo";
    if(try_snip(s, u8"じゃ")) return "ja";
    if(try_snip(s, u8"じゅ")) return "ju";
    if(try_snip(s, u8"じょ")) return "jo";
    if(try_snip(s, u8"ぢゃ")) return "ja";
    if(try_snip(s, u8"ぢゅ")) return "ju";
    if(try_snip(s, u8"ぢょ")) return "jo";
    if(try_snip(s, u8"びゃ")) return "bya";
    if(try_snip(s, u8"びゅ")) return "byu";
    if(try_snip(s, u8"びょ")) return "byo";
    if(try_snip(s, u8"ぴゃ")) return "pya";
    if(try_snip(s, u8"ぴゅ")) return "pyu";
    if(try_snip(s, u8"ぴょ")) return "pyo";
    if(try_snip(s, u8"あ")) return "a";
    if(try_snip(s, u8"い")) return "i";
    if(try_snip(s, u8"う")) return "u";
    if(try_snip(s, u8"え")) return "e";
    if(try_snip(s, u8"お")) return "o";
    if(try_snip(s, u8"か")) return "ka";
    if(try_snip(s, u8"き")) return "ki";
    if(try_snip(s, u8"く")) return "ku";
    if(try_snip(s, u8"け")) return "ke";
    if(try_snip(s, u8"こ")) return "ko";
    if(try_snip(s, u8"さ")) return "sa";
    if(try_snip(s, u8"し")) return "shi";
    if(try_snip(s, u8"す")) return "su";
    if(try_snip(s, u8"せ")) return "se";
    if(try_snip(s, u8"そ")) return "so";
    if(try_snip(s, u8"た")) return "ta";
    if(try_snip(s, u8"ち")) return "chi";
    if(try_snip(s, u8"つ")) return "tsu";
    if(try_snip(s, u8"て")) return "te";
    if(try_snip(s, u8"と")) return "to";
    if(try_snip(s, u8"な")) return "na";
    if(try_snip(s, u8"に")) return "ni";
    if(try_snip(s, u8"ぬ")) return "nu";
    if(try_snip(s, u8"ね")) return "ne";
    if(try_snip(s, u8"の")) return "no";
    if(try_snip(s, u8"は")) return "ha";
    if(try_snip(s, u8"ひ")) return "hi";
    if(try_snip(s, u8"ふ")) return "hu";
    if(try_snip(s, u8"へ")) return "he";
    if(try_snip(s, u8"ほ")) return "ho";
    if(try_snip(s, u8"ま")) return "ma";
    if(try_snip(s, u8"み")) return "mi";
    if(try_snip(s, u8"む")) return "mu";
    if(try_snip(s, u8"め")) return "me";
    if(try_snip(s, u8"も")) return "mo";
    if(try_snip(s, u8"や")) return "ya";
    if(try_snip(s, u8"ゆ")) return "yu";
    if(try_snip(s, u8"よ")) return "yo";
    if(try_snip(s, u8"ら")) return "ra";
    if(try_snip(s, u8"り")) return "ri";
    if(try_snip(s, u8"る")) return "ru";
    if(try_snip(s, u8"れ")) return "re";
    if(try_snip(s, u8"ろ")) return "ro";
    if(try_snip(s, u8"わ")) return "wa";
    if(try_snip(s, u8"ゐ")) return "wi";
    if(try_snip(s, u8"ゑ")) return "we";
    if(try_snip(s, u8"を")) return "o";
    if(try_snip(s, u8"が")) return "ga";
    if(try_snip(s, u8"ぎ")) return "gi";
    if(try_snip(s, u8"ぐ")) return "gu";
    if(try_snip(s, u8"げ")) return "ge";
    if(try_snip(s, u8"ご")) return "go";
    if(try_snip(s, u8"ざ")) return "za";
    if(try_snip(s, u8"じ")) return "ji";
    if(try_snip(s, u8"ず")) return "zu";
    if(try_snip(s, u8"ぜ")) return "ze";
    if(try_snip(s, u8"ぞ")) return "zo";
    if(try_snip(s, u8"だ")) return "da";
    if(try_snip(s, u8"ぢ")) return "ji";
    if(try_snip(s, u8"づ")) return "zu";
    if(try_snip(s, u8"で")) return "de";
    if(try_snip(s, u8"ど")) return "do";
    if(try_snip(s, u8"ば")) return "ba";
    if(try_snip(s, u8"び")) return "bi";
    if(try_snip(s, u8"ぶ")) return "bu";
    if(try_snip(s, u8"べ")) return "be";
    if(try_snip(s, u8"ぼ")) return "bo";
    if(try_snip(s, u8"ぱ")) return "pa";
    if(try_snip(s, u8"ぴ")) return "pi";
    if(try_snip(s, u8"ぷ")) return "pu";
    if(try_snip(s, u8"ぺ")) return "pe";
    if(try_snip(s, u8"ぽ")) return "po";
    if(try_snip(s, u8"ん")) return "n";

    // Katakana
    if(try_snip(s, u8"キャ")) return "kya";
    if(try_snip(s, u8"キュ")) return "kyu";
    if(try_snip(s, u8"キョ")) return "kyo";
    if(try_snip(s, u8"シャ")) return "sha";
    if(try_snip(s, u8"シュ")) return "shu";
    if(try_snip(s, u8"ショ")) return "sho";
    if(try_snip(s, u8"チャ")) return "cha";
    if(try_snip(s, u8"チュ")) return "chu";
    if(try_snip(s, u8"チョ")) return "cho";
    if(try_snip(s, u8"ニャ")) return "nya";
    if(try_snip(s, u8"ニュ")) return "nyu";
    if(try_snip(s, u8"ニョ")) return "nyo";
    if(try_snip(s, u8"ヒャ")) return "hya";
    if(try_snip(s, u8"ヒュ")) return "hyu";
    if(try_snip(s, u8"ヒョ")) return "hyo";
    if(try_snip(s, u8"ミャ")) return "mya";
    if(try_snip(s, u8"ミュ")) return "myu";
    if(try_snip(s, u8"ミョ")) return "myo";
    if(try_snip(s, u8"リャ")) return "rya";
    if(try_snip(s, u8"リュ")) return "ryu";
    if(try_snip(s, u8"リョ")) return "ryo";
    if(try_snip(s, u8"ギャ")) return "gya";
    if(try_snip(s, u8"ギュ")) return "gyu";
    if(try_snip(s, u8"ギョ")) return "gyo";
    if(try_snip(s, u8"ジャ")) return "ja";
    if(try_snip(s, u8"ジュ")) return "ju";
    if(try_snip(s, u8"ジョ")) return "jo";
    if(try_snip(s, u8"ヂャ")) return "ja";
    if(try_snip(s, u8"ヂュ")) return "ju";
    if(try_snip(s, u8"ヂョ")) return "jo";
    if(try_snip(s, u8"ビゃ")) return "bya";
    if(try_snip(s, u8"ビュ")) return "byu";
    if(try_snip(s, u8"ビョ")) return "byo";
    if(try_snip(s, u8"ピゃ")) return "pya";
    if(try_snip(s, u8"ピュ")) return "pyu";
    if(try_snip(s, u8"ピョ")) return "pyo";

    if(try_snip(s, u8"ア")) return "a";
    if(try_snip(s, u8"イ")) return "i";
    if(try_snip(s, u8"ウ")) return "u";
    if(try_snip(s, u8"エ")) return "e";
    if(try_snip(s, u8"オ")) return "o";
    if(try_snip(s, u8"カ")) return "ka";
    if(try_snip(s, u8"キ")) return "ki";
    if(try_snip(s, u8"ク")) return "ku";
    if(try_snip(s, u8"ケ")) return "ke";
    if(try_snip(s, u8"コ")) return "ko";
    if(try_snip(s, u8"サ")) return "sa";
    if(try_snip(s, u8"シ")) return "shi";
    if(try_snip(s, u8"ス")) return "su";
    if(try_snip(s, u8"セ")) return "se";
    if(try_snip(s, u8"ソ")) return "so";
    if(try_snip(s, u8"タ")) return "ta";
    if(try_snip(s, u8"チ")) return "chi";
    if(try_snip(s, u8"ツ")) return "tsu";
    if(try_snip(s, u8"テ")) return "te";
    if(try_snip(s, u8"ト")) return "to";
    if(try_snip(s, u8"ナ")) return "na";
    if(try_snip(s, u8"ニ")) return "ni";
    if(try_snip(s, u8"ヌ")) return "nu";
    if(try_snip(s, u8"ネ")) return "ne";
    if(try_snip(s, u8"ノ")) return "no";
    if(try_snip(s, u8"ハ")) return "ha";
    if(try_snip(s, u8"ヒ")) return "hi";
    if(try_snip(s, u8"フ")) return "fu";
    if(try_snip(s, u8"ヘ")) return "he";
    if(try_snip(s, u8"ホ")) return "ho";
    if(try_snip(s, u8"マ")) return "ma";
    if(try_snip(s, u8"ミ")) return "mi";
    if(try_snip(s, u8"ム")) return "mu";
    if(try_snip(s, u8"メ")) return "me";
    if(try_snip(s, u8"モ")) return "mo";
    if(try_snip(s, u8"ヤ")) return "ya";
    if(try_snip(s, u8"ユ")) return "yu";
    if(try_snip(s, u8"エ")) return "ye"; // Same as "e"
    if(try_snip(s, u8"ヨ")) return "yo";
    if(try_snip(s, u8"ラ")) return "ra";
    if(try_snip(s, u8"リ")) return "ri";
    if(try_snip(s, u8"ル")) return "ru";
    if(try_snip(s, u8"レ")) return "re";
    if(try_snip(s, u8"ロ")) return "ro";
    if(try_snip(s, u8"ワ")) return "wa";
    if(try_snip(s, u8"ヰ")) return "wi";
    if(try_snip(s, u8"ヱ")) return "we";
    if(try_snip(s, u8"ヲ")) return "wo";
    if(try_snip(s, u8"ガ")) return "ga";
    if(try_snip(s, u8"ギ")) return "gi";
    if(try_snip(s, u8"グ")) return "gu";
    if(try_snip(s, u8"ゲ")) return "ge";
    if(try_snip(s, u8"ゴ")) return "go";
    if(try_snip(s, u8"ザ")) return "za";
    if(try_snip(s, u8"ジ")) return "ji";
    if(try_snip(s, u8"ズ")) return "zu";
    if(try_snip(s, u8"ゼ")) return "ze";
    if(try_snip(s, u8"ゾ")) return "zo";
    if(try_snip(s, u8"ダ")) return "da";
    if(try_snip(s, u8"ヂ")) return "ji";
    if(try_snip(s, u8"ヅ")) return "zu";
    if(try_snip(s, u8"デ")) return "de";
    if(try_snip(s, u8"ド")) return "do";
    if(try_snip(s, u8"バ")) return "ba";
    if(try_snip(s, u8"ビ")) return "bi";
    if(try_snip(s, u8"ブ")) return "bu";
    if(try_snip(s, u8"ベ")) return "be";
    if(try_snip(s, u8"ボ")) return "bo";
    if(try_snip(s, u8"パ")) return "pa";
    if(try_snip(s, u8"ピ")) return "pi";
    if(try_snip(s, u8"プ")) return "pu";
    if(try_snip(s, u8"ペ")) return "pe";
    if(try_snip(s, u8"ポ")) return "po";
    if(try_snip(s, u8"ン")) return "n";

    return snip(s, 1);
}

static std::string to_romaji(std::string_view s) {
    std::string result;
    while(!s.empty()) {
        bool tsuPrefix = try_snip(s, u8"っ") || try_snip(s, u8"ッ");

        auto romaji = snip_romaji(s);
        if(tsuPrefix && !romaji.empty())
            result.push_back(romaji.front());
        result.append(romaji);
    }
    return result;
}

std::vector<jmdict::entry const*> jmdict_index::search(std::string_view query) const {
    debug::timer _("search");

    std::vector<jmdict::entry const*> result;
    for(auto& [reading, entry] : byReading) {
        if(reading.find(query) != std::string::npos && !contains(result, entry)) {
            result.push_back(entry);
        }
        Next:;
    }

    return result;
}

jmdict_index::jmdict_index(jmdict const& dict) :
    dict(&dict)
{
    debug::timer _("building index");
    for(auto& entry : dict.entries) {
        bySequenceNumber.emplace(entry.sequence, &entry);
        for(auto& r : entry.readings) {
            byReading.emplace(to_romaji(r.value), &entry);
        }
    }
}

} // namespace jdict