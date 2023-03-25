fn snip_prefix(prefix: &str, text: &mut &str) -> bool {
    if let Some(s) = text.strip_prefix(prefix) {
        *text = s;
        true
    } else {
        false
    }
}

fn remove_first_char(text: &mut &str) -> Option<char> {
    let mut chars = text.chars();
    let c = chars.next()?;
    *text = chars.as_str();
    Some(c)
}

pub fn to_romaji(text: &str) -> String {
    let mut result = String::new();
    let mut text_copy = text;

    let mut was_tsu = false;

    while !text_copy.is_empty() {
		// Small tsu
        if snip_prefix("っ", &mut text_copy) || snip_prefix("ッ", &mut text_copy) {
            was_tsu = true;
            continue;
        }
		// Prolonged sound mark
		else if result.ends_with(is_vowel) && snip_prefix("ー", &mut text_copy) {
			result.push(result.chars().last().unwrap());
		}
		// Kana
        else if let Some(translation) = snip_and_translate_prefix_to_romaji(&mut text_copy) {
            if was_tsu {
                result.push_str(&translation[0..1]);
            }
            result.push_str(translation);
        }
		// Undo full-width chars
		else if let Some(c) = snip_full_width_char(&mut text_copy) {
			result.push(c);
		}
		// Fall back to just copying the first char
        else {
            result.push(remove_first_char(&mut text_copy).unwrap());
        }
        was_tsu = false;
    }
    result
}

fn snip_full_width_char(text: &mut &str) -> Option<char> {
	let c = text.chars().next()?;

	let result = match c {
		'０'..='９' => char::from_u32(c as u32 - '０' as u32 + '0' as u32),
		'Ａ'..='Ｚ' => char::from_u32(c as u32 - 'Ａ' as u32 + 'A' as u32),
		'ａ'..='ｚ' => char::from_u32(c as u32 - 'ａ' as u32 + 'a' as u32),
		_ => None,
	};

	if result.is_some() {
		*text = &text[1..];
	}

	result
}

fn is_vowel(c: char) -> bool {
	matches!(c,
		'a' | 'e' | 'i' | 'o' | 'u' |
		'A' | 'E' | 'I' | 'O' | 'U'
	)
}

#[cfg(test)]
mod test {
	#[test]
	fn test_kana() {
		// Hiragana
		assert_eq!(super::to_romaji("れいぞうこ"), "reizouko"); // Basic
		assert_eq!(super::to_romaji("かって"), "katte"); // tsu
		assert_eq!(super::to_romaji("ぴょこん"), "pyokon");

		// Katakana
		assert_eq!(super::to_romaji("ハンカチ"), "hankachi"); // Basic
		assert_eq!(super::to_romaji("ポット"), "potto"); // tsu
		assert_eq!(super::to_romaji("ハンガリー"), "hangarii"); // prolonged sound mark

	}
}

fn snip_and_translate_prefix_to_romaji(text: &mut &str) -> Option<&'static str> {
	for kana in KANA_TABLE.iter() {
		if text.starts_with(kana.kana) {
			*text = &text[kana.kana.len()..];
			return Some(kana.romaji);
		}
	}
	None
}

pub struct Kana {
	kana: &'static str,
	romaji: &'static str,
}
impl Kana {
	const fn hiragana(kana: &'static str, romaji: &'static str) -> Self {
		Self { kana, romaji }
	}
	const fn katakana(kana: &'static str, romaji: &'static str) -> Self {
		Self { kana, romaji }
	}
	const fn symbol(kana: &'static str, romaji: &'static str) -> Self {
		Self { kana, romaji }
	}
}

pub const KANA_TABLE: [Kana; 399] = sort_by_kana_length([
	Kana::hiragana("きゃ", "kya"),
    Kana::hiragana("きゅ", "kyu"),
    Kana::hiragana("きょ", "kyo"),
    Kana::hiragana("しゃ", "sha"),
    Kana::hiragana("しゅ", "shu"),
    Kana::hiragana("しょ", "sho"),
    Kana::hiragana("ちゃ", "cha"),
    Kana::hiragana("ちゅ", "chu"),
    Kana::hiragana("ちょ", "cho"),
    Kana::hiragana("にゃ", "nya"),
    Kana::hiragana("にゅ", "nyu"),
    Kana::hiragana("にょ", "nyo"),
    Kana::hiragana("ひゃ", "hya"),
    Kana::hiragana("ひゅ", "hyu"),
    Kana::hiragana("ひょ", "hyo"),
    Kana::hiragana("みゃ", "mya"),
    Kana::hiragana("みゅ", "myu"),
    Kana::hiragana("みょ", "myo"),
    Kana::hiragana("りゃ", "rya"),
    Kana::hiragana("りゅ", "ryu"),
    Kana::hiragana("りょ", "ryo"),
    Kana::hiragana("ぎゃ", "gya"),
    Kana::hiragana("ぎゅ", "gyu"),
    Kana::hiragana("ぎょ", "gyo"),
    Kana::hiragana("じゃ", "ja"),
    Kana::hiragana("じゅ", "ju"),
    Kana::hiragana("じょ", "jo"),
    Kana::hiragana("ぢゃ", "ja"),
    Kana::hiragana("ぢゅ", "ju"),
    Kana::hiragana("ぢょ", "jo"),
    Kana::hiragana("びゃ", "bya"),
    Kana::hiragana("びゅ", "byu"),
    Kana::hiragana("びょ", "byo"),
    Kana::hiragana("ぴゃ", "pya"),
    Kana::hiragana("ぴゅ", "pyu"),
    Kana::hiragana("ぴょ", "pyo"),
    Kana::hiragana("ふぁ", "fa"),
    Kana::hiragana("ふぃ", "fi"),
    Kana::hiragana("ふぇ", "fe"),
    Kana::hiragana("ふぉ", "fo"),
    Kana::hiragana("うぃ", "wi"),
    Kana::hiragana("うぇ", "we"),
    Kana::hiragana("うぉ", "wo"),
    Kana::hiragana("きぇ", "kye"),
    Kana::hiragana("しぇ", "she"),
    Kana::hiragana("ちぇ", "che"),
    Kana::hiragana("にぇ", "nye"),
    Kana::hiragana("ひぇ", "hye"),
    Kana::hiragana("みぇ", "mye"),
    Kana::hiragana("りぇ", "rye"),
    Kana::hiragana("ぎぇ", "gye"),
    Kana::hiragana("じぇ", "je"),
    Kana::hiragana("ぢぇ", "je"),
    Kana::hiragana("びぇ", "bye"),
    Kana::hiragana("ぴぇ", "pye"),
    Kana::hiragana("ふぇ", "fe"),
    Kana::hiragana("ふぉ", "fo"),
    Kana::hiragana("きゃ", "kya"),
    Kana::hiragana("きゅ", "kyu"),
    Kana::hiragana("きょ", "kyo"),
    Kana::hiragana("しゃ", "sha"),
    Kana::hiragana("しゅ", "shu"),
    Kana::hiragana("しょ", "sho"),
    Kana::hiragana("ちゃ", "cha"),
    Kana::hiragana("ちゅ", "chu"),
    Kana::hiragana("ちょ", "cho"),
    Kana::hiragana("にゃ", "nya"),
    Kana::hiragana("にゅ", "nyu"),
    Kana::hiragana("にょ", "nyo"),
    Kana::hiragana("ひゃ", "hya"),
    Kana::hiragana("ひゅ", "hyu"),
    Kana::hiragana("ひょ", "hyo"),
    Kana::hiragana("みゃ", "mya"),
    Kana::hiragana("みゅ", "myu"),
    Kana::hiragana("みょ", "myo"),
    Kana::hiragana("りゃ", "rya"),
    Kana::hiragana("りゅ", "ryu"),
    Kana::hiragana("りょ", "ryo"),
    Kana::hiragana("ぎゃ", "gya"),
    Kana::hiragana("ぎゅ", "gyu"),
    Kana::hiragana("ぎょ", "gyo"),
    Kana::hiragana("じゃ", "ja"),
    Kana::hiragana("じゅ", "ju"),
    Kana::hiragana("じょ", "jo"),
    Kana::hiragana("ぢゃ", "ja"),
    Kana::hiragana("ぢゅ", "ju"),
    Kana::hiragana("ぢょ", "jo"),
    Kana::hiragana("びゃ", "bya"),
    Kana::hiragana("びゅ", "byu"),
    Kana::hiragana("びょ", "byo"),
    Kana::hiragana("ぴゃ", "pya"),
    Kana::hiragana("ぴゅ", "pyu"),
    Kana::hiragana("ぴょ", "pyo"),
    Kana::hiragana("あ", "a"),
    Kana::hiragana("い", "i"),
    Kana::hiragana("う", "u"),
    Kana::hiragana("え", "e"),
    Kana::hiragana("お", "o"),
    Kana::hiragana("か", "ka"),
    Kana::hiragana("き", "ki"),
    Kana::hiragana("く", "ku"),
    Kana::hiragana("け", "ke"),
    Kana::hiragana("こ", "ko"),
    Kana::hiragana("さ", "sa"),
    Kana::hiragana("し", "shi"),
    Kana::hiragana("す", "su"),
    Kana::hiragana("せ", "se"),
    Kana::hiragana("そ", "so"),
    Kana::hiragana("た", "ta"),
    Kana::hiragana("ち", "chi"),
    Kana::hiragana("つ", "tsu"),
    Kana::hiragana("て", "te"),
    Kana::hiragana("と", "to"),
    Kana::hiragana("な", "na"),
    Kana::hiragana("に", "ni"),
    Kana::hiragana("ぬ", "nu"),
    Kana::hiragana("ね", "ne"),
    Kana::hiragana("の", "no"),
    Kana::hiragana("は", "ha"),
    Kana::hiragana("ひ", "hi"),
    Kana::hiragana("ふ", "fu"),
    Kana::hiragana("へ", "he"),
    Kana::hiragana("ほ", "ho"),
    Kana::hiragana("ま", "ma"),
    Kana::hiragana("み", "mi"),
    Kana::hiragana("む", "mu"),
    Kana::hiragana("め", "me"),
    Kana::hiragana("も", "mo"),
    Kana::hiragana("や", "ya"),
    Kana::hiragana("ゆ", "yu"),
    Kana::hiragana("よ", "yo"),
    Kana::hiragana("ら", "ra"),
    Kana::hiragana("り", "ri"),
    Kana::hiragana("る", "ru"),
    Kana::hiragana("れ", "re"),
    Kana::hiragana("ろ", "ro"),
    Kana::hiragana("わ", "wa"),
    Kana::hiragana("を", "wo"),
    Kana::hiragana("ん", "n"),
    Kana::hiragana("が", "ga"),
    Kana::hiragana("ぎ", "gi"),
    Kana::hiragana("ぐ", "gu"),
    Kana::hiragana("げ", "ge"),
    Kana::hiragana("ご", "go"),
    Kana::hiragana("ざ", "za"),
    Kana::hiragana("じ", "ji"),
    Kana::hiragana("ず", "zu"),
    Kana::hiragana("ぜ", "ze"),
    Kana::hiragana("ぞ", "zo"),
    Kana::hiragana("だ", "da"),
    Kana::hiragana("ぢ", "ji"),
    Kana::hiragana("づ", "zu"),
    Kana::hiragana("で", "de"),
    Kana::hiragana("ど", "do"),
    Kana::hiragana("ば", "ba"),
    Kana::hiragana("び", "bi"),
    Kana::hiragana("ぶ", "bu"),
    Kana::hiragana("べ", "be"),
    Kana::hiragana("ぼ", "bo"),
    Kana::hiragana("ぱ", "pa"),
    Kana::hiragana("ぴ", "pi"),
    Kana::hiragana("ぷ", "pu"),
    Kana::hiragana("ぺ", "pe"),
    Kana::hiragana("ぽ", "po"),
    Kana::katakana("ヴァ", "va"),
    Kana::katakana("ヴィ", "vi"),
    Kana::katakana("ヴ", "vu"),
    Kana::katakana("ヴェ", "ve"),
    Kana::katakana("ヴォ", "vo"),
    Kana::katakana("う゛ぁ", "va"),
    Kana::katakana("う゛ぃ", "vi"),
    Kana::katakana("う゛", "vu"),
    Kana::katakana("う゛ぇ", "ve"),
    Kana::katakana("う゛ぉ", "vo"),
    Kana::katakana("キャ", "kya"),
    Kana::katakana("キュ", "kyu"),
    Kana::katakana("キョ", "kyo"),
    Kana::katakana("シャ", "sha"),
    Kana::katakana("シュ", "shu"),
    Kana::katakana("ショ", "sho"),
    Kana::katakana("チャ", "cha"),
    Kana::katakana("チュ", "chu"),
    Kana::katakana("チョ", "cho"),
    Kana::katakana("ニャ", "nya"),
    Kana::katakana("ニュ", "nyu"),
    Kana::katakana("ニョ", "nyo"),
    Kana::katakana("ヒャ", "hya"),
    Kana::katakana("ヒュ", "hyu"),
    Kana::katakana("ヒョ", "hyo"),
    Kana::katakana("ミャ", "mya"),
    Kana::katakana("ミュ", "myu"),
    Kana::katakana("ミョ", "myo"),
    Kana::katakana("リャ", "rya"),
    Kana::katakana("リュ", "ryu"),
    Kana::katakana("リョ", "ryo"),
    Kana::katakana("ギャ", "gya"),
    Kana::katakana("ギュ", "gyu"),
    Kana::katakana("ギョ", "gyo"),
    Kana::katakana("ジャ", "ja"),
    Kana::katakana("ジュ", "ju"),
    Kana::katakana("ジョ", "jo"),
    Kana::katakana("ヂャ", "ja"),
    Kana::katakana("ヂュ", "ju"),
    Kana::katakana("ヂョ", "jo"),
    Kana::katakana("ビャ", "bya"),
    Kana::katakana("ビュ", "byu"),
    Kana::katakana("ビョ", "byo"),
    Kana::katakana("ピャ", "pya"),
    Kana::katakana("ピュ", "pyu"),
    Kana::katakana("ピョ", "pyo"),
    Kana::katakana("ヴァ", "va"),
    Kana::katakana("ヴィ", "vi"),
    Kana::katakana("ヴ", "vu"),
    Kana::katakana("ヴェ", "ve"),
    Kana::katakana("ヴォ", "vo"),
    Kana::katakana("ウ゛ァ", "va"),
    Kana::katakana("ウ゛ィ", "vi"),
    Kana::katakana("ウ゛", "vu"),
    Kana::katakana("ウ゛ェ", "ve"),
    Kana::katakana("ウ゛ォ", "vo"),
    Kana::katakana("キャ", "kya"),
    Kana::katakana("キュ", "kyu"),
    Kana::katakana("キョ", "kyo"),
    Kana::katakana("シャ", "sha"),
    Kana::katakana("シュ", "shu"),
    Kana::katakana("ショ", "sho"),
    Kana::katakana("チャ", "cha"),
    Kana::katakana("チュ", "chu"),
    Kana::katakana("チョ", "cho"),
    Kana::katakana("ニャ", "nya"),
    Kana::katakana("ニュ", "nyu"),
    Kana::katakana("ニョ", "nyo"),
    Kana::katakana("ヒャ", "hya"),
    Kana::katakana("ヒュ", "hyu"),
    Kana::katakana("ヒョ", "hyo"),
    Kana::katakana("ミャ", "mya"),
    Kana::katakana("ミュ", "myu"),
    Kana::katakana("ミョ", "myo"),
    Kana::katakana("リャ", "rya"),
    Kana::katakana("リュ", "ryu"),
    Kana::katakana("リョ", "ryo"),
    Kana::katakana("ギャ", "gya"),
    Kana::katakana("ギュ", "gyu"),
    Kana::katakana("ギョ", "gyo"),
    Kana::katakana("ジャ", "ja"),
    Kana::katakana("ジュ", "ju"),
    Kana::katakana("ジョ", "jo"),
    Kana::katakana("ヂャ", "ja"),
    Kana::katakana("ヂュ", "ju"),
    Kana::katakana("ヂョ", "jo"),
    Kana::katakana("ビャ", "bya"),
    Kana::katakana("ビュ", "byu"),
    Kana::katakana("ビョ", "byo"),
    Kana::katakana("ピャ", "pya"),
    Kana::katakana("ピュ", "pyu"),
    Kana::katakana("ピョ", "pyo"),
    Kana::katakana("ヴァ", "va"),
    Kana::katakana("ヴィ", "vi"),
    Kana::katakana("ヴ", "vu"),
    Kana::katakana("ヴェ", "ve"),
    Kana::katakana("ヴォ", "vo"),
    Kana::katakana("ウ゛ァ", "va"),
    Kana::katakana("ウ゛ィ", "vi"),
    Kana::katakana("ウ゛", "vu"),
    Kana::katakana("ウ゛ェ", "ve"),
    Kana::katakana("ウ゛ォ", "vo"),
    Kana::katakana("キャ", "kya"),
    Kana::katakana("キュ", "kyu"),
    Kana::katakana("キョ", "kyo"),
    Kana::katakana("シャ", "sha"),
    Kana::katakana("シュ", "shu"),
    Kana::katakana("ショ", "sho"),
    Kana::katakana("チャ", "cha"),
    Kana::katakana("チュ", "chu"),
    Kana::katakana("チョ", "cho"),
    Kana::katakana("ニャ", "nya"),
    Kana::katakana("ニュ", "nyu"),
    Kana::katakana("ニョ", "nyo"),
    Kana::katakana("ヒャ", "hya"),
    Kana::katakana("ヒュ", "hyu"),
    Kana::katakana("ヒョ", "hyo"),
    Kana::katakana("ミャ", "mya"),
    Kana::katakana("ミュ", "myu"),
    Kana::katakana("ミョ", "myo"),
    Kana::katakana("リャ", "rya"),
    Kana::katakana("リュ", "ryu"),
    Kana::katakana("リョ", "ryo"),
    Kana::katakana("ギャ", "gya"),
    Kana::katakana("ギュ", "gyu"),
    Kana::katakana("ギョ", "gyo"),
    Kana::katakana("ジャ", "ja"),
    Kana::katakana("ジュ", "ju"),
    Kana::katakana("ジョ", "jo"),
    Kana::katakana("ヂャ", "ja"),
    Kana::katakana("ヂュ", "ju"),
    Kana::katakana("ヂョ", "jo"),
    Kana::katakana("ビャ", "bya"),
    Kana::katakana("ビュ", "byu"),
    Kana::katakana("ビョ", "byo"),
    Kana::katakana("ピャ", "pya"),
    Kana::katakana("ピュ", "pyu"),
    Kana::katakana("ピョ", "pyo"),
    Kana::katakana("ヴァ", "va"),
    Kana::katakana("ヴィ", "vi"),
    Kana::katakana("ヴ", "vu"),
    Kana::katakana("ヴェ", "ve"),
    Kana::katakana("ヴォ", "vo"),
    Kana::katakana("ウ゛ァ", "va"),
    Kana::katakana("ウ゛ィ", "vi"),
    Kana::katakana("ウ゛", "vu"),
    Kana::katakana("ウ゛ェ", "ve"),
    Kana::katakana("ウ゛ォ", "vo"),
    Kana::katakana("ア", "a"),
    Kana::katakana("イ", "i"),
    Kana::katakana("ウ", "u"),
    Kana::katakana("エ", "e"),
    Kana::katakana("オ", "o"),
    Kana::katakana("カ", "ka"),
    Kana::katakana("キ", "ki"),
    Kana::katakana("ク", "ku"),
    Kana::katakana("ケ", "ke"),
    Kana::katakana("コ", "ko"),
    Kana::katakana("サ", "sa"),
    Kana::katakana("シ", "shi"),
    Kana::katakana("ス", "su"),
    Kana::katakana("セ", "se"),
    Kana::katakana("ソ", "so"),
    Kana::katakana("タ", "ta"),
    Kana::katakana("チ", "chi"),
    Kana::katakana("ツ", "tsu"),
    Kana::katakana("テ", "te"),
    Kana::katakana("ト", "to"),
    Kana::katakana("ナ", "na"),
    Kana::katakana("ニ", "ni"),
    Kana::katakana("ヌ", "nu"),
    Kana::katakana("ネ", "ne"),
    Kana::katakana("ノ", "no"),
    Kana::katakana("ハ", "ha"),
    Kana::katakana("ヒ", "hi"),
    Kana::katakana("フ", "fu"),
    Kana::katakana("ヘ", "he"),
    Kana::katakana("ホ", "ho"),
    Kana::katakana("マ", "ma"),
    Kana::katakana("ミ", "mi"),
    Kana::katakana("ム", "mu"),
    Kana::katakana("メ", "me"),
    Kana::katakana("モ", "mo"),
    Kana::katakana("ヤ", "ya"),
    Kana::katakana("ユ", "yu"),
    Kana::katakana("ヨ", "yo"),
    Kana::katakana("ラ", "ra"),
    Kana::katakana("リ", "ri"),
    Kana::katakana("ル", "ru"),
    Kana::katakana("レ", "re"),
    Kana::katakana("ロ", "ro"),
    Kana::katakana("ワ", "wa"),
    Kana::katakana("ヲ", "wo"),
    Kana::katakana("ン", "n"),
    Kana::katakana("ガ", "ga"),
    Kana::katakana("ギ", "gi"),
    Kana::katakana("グ", "gu"),
    Kana::katakana("ゲ", "ge"),
    Kana::katakana("ゴ", "go"),
    Kana::katakana("ザ", "za"),
    Kana::katakana("ジ", "ji"),
    Kana::katakana("ズ", "zu"),
    Kana::katakana("ゼ", "ze"),
    Kana::katakana("ゾ", "zo"),
    Kana::katakana("ダ", "da"),
    Kana::katakana("ヂ", "ji"),
    Kana::katakana("ヅ", "zu"),
    Kana::katakana("デ", "de"),
    Kana::katakana("ド", "do"),
    Kana::katakana("バ", "ba"),
    Kana::katakana("ビ", "bi"),
    Kana::katakana("ブ", "bu"),
    Kana::katakana("ベ", "be"),
    Kana::katakana("ボ", "bo"),
    Kana::katakana("パ", "pa"),
    Kana::katakana("ピ", "pi"),
    Kana::katakana("プ", "pu"),
    Kana::katakana("ペ", "pe"),
    Kana::katakana("ポ", "po"),
    Kana::katakana("ァ", "a"),
    Kana::katakana("ィ", "i"),
    Kana::katakana("ゥ", "u"),
    Kana::katakana("ェ", "e"),
    Kana::katakana("ォ", "o"),
    Kana::katakana("ャ", "ya"),
    Kana::katakana("ュ", "yu"),
    Kana::katakana("ョ", "yo"),
    Kana::katakana("ッ", "tu"),
    Kana::katakana("ヴ", "vu"),
    Kana::symbol("ー", "-"),
    Kana::symbol("。", "."),
    Kana::symbol("、", ","),
    Kana::symbol("！", "!"),
    Kana::symbol("？", "?"),
    Kana::symbol("　", " "),
]);

const fn sort_by_kana_length<const N: usize>(kana: [Kana; N]) -> [Kana; N] {
    let mut kana = kana;

    // Insertion sort by descending kana length; Can't use for loop because of const
    let mut i = 1;
    while i < N {
        let mut j = i;
        while j > 0 && kana[j - 1].kana.len() < kana[j].kana.len() {
            kana.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }

    kana
}
