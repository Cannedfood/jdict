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
        if snip_prefix("っ", &mut text_copy) {
            was_tsu = true;
            continue;
        }
        else if let Some(translation) = snip_and_translate_prefix_to_romaji(&mut text_copy) {
            if was_tsu {
                result.push_str(&translation[0..1]);
            }
            result.push_str(translation);
        }
        else {
            // Strip first char from text_copy and push it to result
            result.push(remove_first_char(&mut text_copy).unwrap());
        }
        was_tsu = false;
    }
    result
}

fn snip_and_translate_prefix_to_romaji(text: &mut &str) -> Option<&'static str> {
    if      snip_prefix("きゃ", text) { Some("kya") }
    else if snip_prefix("きゅ", text) { Some("kyu") }
    else if snip_prefix("きょ", text) { Some("kyo") }
    else if snip_prefix("しゃ", text) { Some("sha") }
    else if snip_prefix("しゅ", text) { Some("shu") }
    else if snip_prefix("しょ", text) { Some("sho") }
    else if snip_prefix("ちゃ", text) { Some("cha") }
    else if snip_prefix("ちゅ", text) { Some("chu") }
    else if snip_prefix("ちょ", text) { Some("cho") }
    else if snip_prefix("にゃ", text) { Some("nya") }
    else if snip_prefix("にゅ", text) { Some("nyu") }
    else if snip_prefix("にょ", text) { Some("nyo") }
    else if snip_prefix("ひゃ", text) { Some("hya") }
    else if snip_prefix("ひゅ", text) { Some("hyu") }
    else if snip_prefix("ひょ", text) { Some("hyo") }
    else if snip_prefix("みゃ", text) { Some("mya") }
    else if snip_prefix("みゅ", text) { Some("myu") }
    else if snip_prefix("みょ", text) { Some("myo") }
    else if snip_prefix("りゃ", text) { Some("rya") }
    else if snip_prefix("りゅ", text) { Some("ryu") }
    else if snip_prefix("りょ", text) { Some("ryo") }
    else if snip_prefix("ぎゃ", text) { Some("gya") }
    else if snip_prefix("ぎゅ", text) { Some("gyu") }
    else if snip_prefix("ぎょ", text) { Some("gyo") }
    else if snip_prefix("じゃ", text) { Some("ja") }
    else if snip_prefix("じゅ", text) { Some("ju") }
    else if snip_prefix("じょ", text) { Some("jo") }
    else if snip_prefix("ぢゃ", text) { Some("ja") }
    else if snip_prefix("ぢゅ", text) { Some("ju") }
    else if snip_prefix("ぢょ", text) { Some("jo") }
    else if snip_prefix("びゃ", text) { Some("bya") }
    else if snip_prefix("びゅ", text) { Some("byu") }
    else if snip_prefix("びょ", text) { Some("byo") }
    else if snip_prefix("ぴゃ", text) { Some("pya") }
    else if snip_prefix("ぴゅ", text) { Some("pyu") }
    else if snip_prefix("ぴょ", text) { Some("pyo") }
    else if snip_prefix("ふぁ", text) { Some("fa") }
    else if snip_prefix("ふぃ", text) { Some("fi") }
    else if snip_prefix("ふぇ", text) { Some("fe") }
    else if snip_prefix("ふぉ", text) { Some("fo") }
    else if snip_prefix("うぃ", text) { Some("wi") }
    else if snip_prefix("うぇ", text) { Some("we") }
    else if snip_prefix("うぉ", text) { Some("wo") }
    else if snip_prefix("きぇ", text) { Some("kye") }
    else if snip_prefix("しぇ", text) { Some("she") }
    else if snip_prefix("ちぇ", text) { Some("che") }
    else if snip_prefix("にぇ", text) { Some("nye") }
    else if snip_prefix("ひぇ", text) { Some("hye") }
    else if snip_prefix("みぇ", text) { Some("mye") }
    else if snip_prefix("りぇ", text) { Some("rye") }
    else if snip_prefix("ぎぇ", text) { Some("gye") }
    else if snip_prefix("じぇ", text) { Some("je") }
    else if snip_prefix("ぢぇ", text) { Some("je") }
    else if snip_prefix("びぇ", text) { Some("bye") }
    else if snip_prefix("ぴぇ", text) { Some("pye") }
    else if snip_prefix("ふぇ", text) { Some("fe") }
    else if snip_prefix("ふぉ", text) { Some("fo") }
    else if snip_prefix("きゃ", text) { Some("kya") }
    else if snip_prefix("きゅ", text) { Some("kyu") }
    else if snip_prefix("きょ", text) { Some("kyo") }
    else if snip_prefix("しゃ", text) { Some("sha") }
    else if snip_prefix("しゅ", text) { Some("shu") }
    else if snip_prefix("しょ", text) { Some("sho") }
    else if snip_prefix("ちゃ", text) { Some("cha") }
    else if snip_prefix("ちゅ", text) { Some("chu") }
    else if snip_prefix("ちょ", text) { Some("cho") }
    else if snip_prefix("にゃ", text) { Some("nya") }
    else if snip_prefix("にゅ", text) { Some("nyu") }
    else if snip_prefix("にょ", text) { Some("nyo") }
    else if snip_prefix("ひゃ", text) { Some("hya") }
    else if snip_prefix("ひゅ", text) { Some("hyu") }
    else if snip_prefix("ひょ", text) { Some("hyo") }
    else if snip_prefix("みゃ", text) { Some("mya") }
    else if snip_prefix("みゅ", text) { Some("myu") }
    else if snip_prefix("みょ", text) { Some("myo") }
    else if snip_prefix("りゃ", text) { Some("rya") }
    else if snip_prefix("りゅ", text) { Some("ryu") }
    else if snip_prefix("りょ", text) { Some("ryo") }
    else if snip_prefix("ぎゃ", text) { Some("gya") }
    else if snip_prefix("ぎゅ", text) { Some("gyu") }
    else if snip_prefix("ぎょ", text) { Some("gyo") }
    else if snip_prefix("じゃ", text) { Some("ja") }
    else if snip_prefix("じゅ", text) { Some("ju") }
    else if snip_prefix("じょ", text) { Some("jo") }
    else if snip_prefix("ぢゃ", text) { Some("ja") }
    else if snip_prefix("ぢゅ", text) { Some("ju") }
    else if snip_prefix("ぢょ", text) { Some("jo") }
    else if snip_prefix("びゃ", text) { Some("bya") }
    else if snip_prefix("びゅ", text) { Some("byu") }
    else if snip_prefix("びょ", text) { Some("byo") }
    else if snip_prefix("ぴゃ", text) { Some("pya") }
    else if snip_prefix("ぴゅ", text) { Some("pyu") }
    else if snip_prefix("ぴょ", text) { Some("pyo") }
    else if snip_prefix("あ", text) { Some("a") }
    else if snip_prefix("い", text) { Some("i") }
    else if snip_prefix("う", text) { Some("u") }
    else if snip_prefix("え", text) { Some("e") }
    else if snip_prefix("お", text) { Some("o") }
    else if snip_prefix("か", text) { Some("ka") }
    else if snip_prefix("き", text) { Some("ki") }
    else if snip_prefix("く", text) { Some("ku") }
    else if snip_prefix("け", text) { Some("ke") }
    else if snip_prefix("こ", text) { Some("ko") }
    else if snip_prefix("さ", text) { Some("sa") }
    else if snip_prefix("し", text) { Some("shi") }
    else if snip_prefix("す", text) { Some("su") }
    else if snip_prefix("せ", text) { Some("se") }
    else if snip_prefix("そ", text) { Some("so") }
    else if snip_prefix("た", text) { Some("ta") }
    else if snip_prefix("ち", text) { Some("chi") }
    else if snip_prefix("つ", text) { Some("tsu") }
    else if snip_prefix("て", text) { Some("te") }
    else if snip_prefix("と", text) { Some("to") }
    else if snip_prefix("な", text) { Some("na") }
    else if snip_prefix("に", text) { Some("ni") }
    else if snip_prefix("ぬ", text) { Some("nu") }
    else if snip_prefix("ね", text) { Some("ne") }
    else if snip_prefix("の", text) { Some("no") }
    else if snip_prefix("は", text) { Some("ha") }
    else if snip_prefix("ひ", text) { Some("hi") }
    else if snip_prefix("ふ", text) { Some("fu") }
    else if snip_prefix("へ", text) { Some("he") }
    else if snip_prefix("ほ", text) { Some("ho") }
    else if snip_prefix("ま", text) { Some("ma") }
    else if snip_prefix("み", text) { Some("mi") }
    else if snip_prefix("む", text) { Some("mu") }
    else if snip_prefix("め", text) { Some("me") }
    else if snip_prefix("も", text) { Some("mo") }
    else if snip_prefix("や", text) { Some("ya") }
    else if snip_prefix("ゆ", text) { Some("yu") }
    else if snip_prefix("よ", text) { Some("yo") }
    else if snip_prefix("ら", text) { Some("ra") }
    else if snip_prefix("り", text) { Some("ri") }
    else if snip_prefix("る", text) { Some("ru") }
    else if snip_prefix("れ", text) { Some("re") }
    else if snip_prefix("ろ", text) { Some("ro") }
    else if snip_prefix("わ", text) { Some("wa") }
    else if snip_prefix("を", text) { Some("wo") }
    else if snip_prefix("ん", text) { Some("n") }
    else if snip_prefix("が", text) { Some("ga") }
    else if snip_prefix("ぎ", text) { Some("gi") }
    else if snip_prefix("ぐ", text) { Some("gu") }
    else if snip_prefix("げ", text) { Some("ge") }
    else if snip_prefix("ご", text) { Some("go") }
    else if snip_prefix("ざ", text) { Some("za") }
    else if snip_prefix("じ", text) { Some("ji") }
    else if snip_prefix("ず", text) { Some("zu") }
    else if snip_prefix("ぜ", text) { Some("ze") }
    else if snip_prefix("ぞ", text) { Some("zo") }
    else if snip_prefix("だ", text) { Some("da") }
    else if snip_prefix("ぢ", text) { Some("ji") }
    else if snip_prefix("づ", text) { Some("zu") }
    else if snip_prefix("で", text) { Some("de") }
    else if snip_prefix("ど", text) { Some("do") }
    else if snip_prefix("ば", text) { Some("ba") }
    else if snip_prefix("び", text) { Some("bi") }
    else if snip_prefix("ぶ", text) { Some("bu") }
    else if snip_prefix("べ", text) { Some("be") }
    else if snip_prefix("ぼ", text) { Some("bo") }
    else if snip_prefix("ぱ", text) { Some("pa") }
    else if snip_prefix("ぴ", text) { Some("pi") }
    else if snip_prefix("ぷ", text) { Some("pu") }
    else if snip_prefix("ぺ", text) { Some("pe") }
    else if snip_prefix("ぽ", text) { Some("po") }
    else if snip_prefix("ヴァ", text) { Some("va") }
    else if snip_prefix("ヴィ", text) { Some("vi") }
    else if snip_prefix("ヴ", text) { Some("vu") }
    else if snip_prefix("ヴェ", text) { Some("ve") }
    else if snip_prefix("ヴォ", text) { Some("vo") }
    else if snip_prefix("う゛ぁ", text) { Some("va") }
    else if snip_prefix("う゛ぃ", text) { Some("vi") }
    else if snip_prefix("う゛", text) { Some("vu") }
    else if snip_prefix("う゛ぇ", text) { Some("ve") }
    else if snip_prefix("う゛ぉ", text) { Some("vo") }
    else if snip_prefix("キャ", text) { Some("kya") }
    else if snip_prefix("キュ", text) { Some("kyu") }
    else if snip_prefix("キョ", text) { Some("kyo") }
    else if snip_prefix("シャ", text) { Some("sha") }
    else if snip_prefix("シュ", text) { Some("shu") }
    else if snip_prefix("ショ", text) { Some("sho") }
    else if snip_prefix("チャ", text) { Some("cha") }
    else if snip_prefix("チュ", text) { Some("chu") }
    else if snip_prefix("チョ", text) { Some("cho") }
    else if snip_prefix("ニャ", text) { Some("nya") }
    else if snip_prefix("ニュ", text) { Some("nyu") }
    else if snip_prefix("ニョ", text) { Some("nyo") }
    else if snip_prefix("ヒャ", text) { Some("hya") }
    else if snip_prefix("ヒュ", text) { Some("hyu") }
    else if snip_prefix("ヒョ", text) { Some("hyo") }
    else if snip_prefix("ミャ", text) { Some("mya") }
    else if snip_prefix("ミュ", text) { Some("myu") }
    else if snip_prefix("ミョ", text) { Some("myo") }
    else if snip_prefix("リャ", text) { Some("rya") }
    else if snip_prefix("リュ", text) { Some("ryu") }
    else if snip_prefix("リョ", text) { Some("ryo") }
    else if snip_prefix("ギャ", text) { Some("gya") }
    else if snip_prefix("ギュ", text) { Some("gyu") }
    else if snip_prefix("ギョ", text) { Some("gyo") }
    else if snip_prefix("ジャ", text) { Some("ja") }
    else if snip_prefix("ジュ", text) { Some("ju") }
    else if snip_prefix("ジョ", text) { Some("jo") }
    else if snip_prefix("ヂャ", text) { Some("ja") }
    else if snip_prefix("ヂュ", text) { Some("ju") }
    else if snip_prefix("ヂョ", text) { Some("jo") }
    else if snip_prefix("ビャ", text) { Some("bya") }
    else if snip_prefix("ビュ", text) { Some("byu") }
    else if snip_prefix("ビョ", text) { Some("byo") }
    else if snip_prefix("ピャ", text) { Some("pya") }
    else if snip_prefix("ピュ", text) { Some("pyu") }
    else if snip_prefix("ピョ", text) { Some("pyo") }
    else if snip_prefix("ヴァ", text) { Some("va") }
    else if snip_prefix("ヴィ", text) { Some("vi") }
    else if snip_prefix("ヴ", text) { Some("vu") }
    else if snip_prefix("ヴェ", text) { Some("ve") }
    else if snip_prefix("ヴォ", text) { Some("vo") }
    else if snip_prefix("ウ゛ァ", text) { Some("va") }
    else if snip_prefix("ウ゛ィ", text) { Some("vi") }
    else if snip_prefix("ウ゛", text) { Some("vu") }
    else if snip_prefix("ウ゛ェ", text) { Some("ve") }
    else if snip_prefix("ウ゛ォ", text) { Some("vo") }
    else if snip_prefix("キャ", text) { Some("kya") }
    else if snip_prefix("キュ", text) { Some("kyu") }
    else if snip_prefix("キョ", text) { Some("kyo") }
    else if snip_prefix("シャ", text) { Some("sha") }
    else if snip_prefix("シュ", text) { Some("shu") }
    else if snip_prefix("ショ", text) { Some("sho") }
    else if snip_prefix("チャ", text) { Some("cha") }
    else if snip_prefix("チュ", text) { Some("chu") }
    else if snip_prefix("チョ", text) { Some("cho") }
    else if snip_prefix("ニャ", text) { Some("nya") }
    else if snip_prefix("ニュ", text) { Some("nyu") }
    else if snip_prefix("ニョ", text) { Some("nyo") }
    else if snip_prefix("ヒャ", text) { Some("hya") }
    else if snip_prefix("ヒュ", text) { Some("hyu") }
    else if snip_prefix("ヒョ", text) { Some("hyo") }
    else if snip_prefix("ミャ", text) { Some("mya") }
    else if snip_prefix("ミュ", text) { Some("myu") }
    else if snip_prefix("ミョ", text) { Some("myo") }
    else if snip_prefix("リャ", text) { Some("rya") }
    else if snip_prefix("リュ", text) { Some("ryu") }
    else if snip_prefix("リョ", text) { Some("ryo") }
    else if snip_prefix("ギャ", text) { Some("gya") }
    else if snip_prefix("ギュ", text) { Some("gyu") }
    else if snip_prefix("ギョ", text) { Some("gyo") }
    else if snip_prefix("ジャ", text) { Some("ja") }
    else if snip_prefix("ジュ", text) { Some("ju") }
    else if snip_prefix("ジョ", text) { Some("jo") }
    else if snip_prefix("ヂャ", text) { Some("ja") }
    else if snip_prefix("ヂュ", text) { Some("ju") }
    else if snip_prefix("ヂョ", text) { Some("jo") }
    else if snip_prefix("ビャ", text) { Some("bya") }
    else if snip_prefix("ビュ", text) { Some("byu") }
    else if snip_prefix("ビョ", text) { Some("byo") }
    else if snip_prefix("ピャ", text) { Some("pya") }
    else if snip_prefix("ピュ", text) { Some("pyu") }
    else if snip_prefix("ピョ", text) { Some("pyo") }
    else if snip_prefix("ヴァ", text) { Some("va") }
    else if snip_prefix("ヴィ", text) { Some("vi") }
    else if snip_prefix("ヴ", text) { Some("vu") }
    else if snip_prefix("ヴェ", text) { Some("ve") }
    else if snip_prefix("ヴォ", text) { Some("vo") }
    else if snip_prefix("ウ゛ァ", text) { Some("va") }
    else if snip_prefix("ウ゛ィ", text) { Some("vi") }
    else if snip_prefix("ウ゛", text) { Some("vu") }
    else if snip_prefix("ウ゛ェ", text) { Some("ve") }
    else if snip_prefix("ウ゛ォ", text) { Some("vo") }
    else if snip_prefix("キャ", text) { Some("kya") }
    else if snip_prefix("キュ", text) { Some("kyu") }
    else if snip_prefix("キョ", text) { Some("kyo") }
    else if snip_prefix("シャ", text) { Some("sha") }
    else if snip_prefix("シュ", text) { Some("shu") }
    else if snip_prefix("ショ", text) { Some("sho") }
    else if snip_prefix("チャ", text) { Some("cha") }
    else if snip_prefix("チュ", text) { Some("chu") }
    else if snip_prefix("チョ", text) { Some("cho") }
    else if snip_prefix("ニャ", text) { Some("nya") }
    else if snip_prefix("ニュ", text) { Some("nyu") }
    else if snip_prefix("ニョ", text) { Some("nyo") }
    else if snip_prefix("ヒャ", text) { Some("hya") }
    else if snip_prefix("ヒュ", text) { Some("hyu") }
    else if snip_prefix("ヒョ", text) { Some("hyo") }
    else if snip_prefix("ミャ", text) { Some("mya") }
    else if snip_prefix("ミュ", text) { Some("myu") }
    else if snip_prefix("ミョ", text) { Some("myo") }
    else if snip_prefix("リャ", text) { Some("rya") }
    else if snip_prefix("リュ", text) { Some("ryu") }
    else if snip_prefix("リョ", text) { Some("ryo") }
    else if snip_prefix("ギャ", text) { Some("gya") }
    else if snip_prefix("ギュ", text) { Some("gyu") }
    else if snip_prefix("ギョ", text) { Some("gyo") }
    else if snip_prefix("ジャ", text) { Some("ja") }
    else if snip_prefix("ジュ", text) { Some("ju") }
    else if snip_prefix("ジョ", text) { Some("jo") }
    else if snip_prefix("ヂャ", text) { Some("ja") }
    else if snip_prefix("ヂュ", text) { Some("ju") }
    else if snip_prefix("ヂョ", text) { Some("jo") }
    else if snip_prefix("ビャ", text) { Some("bya") }
    else if snip_prefix("ビュ", text) { Some("byu") }
    else if snip_prefix("ビョ", text) { Some("byo") }
    else if snip_prefix("ピャ", text) { Some("pya") }
    else if snip_prefix("ピュ", text) { Some("pyu") }
    else if snip_prefix("ピョ", text) { Some("pyo") }
    else if snip_prefix("ヴァ", text) { Some("va") }
    else if snip_prefix("ヴィ", text) { Some("vi") }
    else if snip_prefix("ヴ", text) { Some("vu") }
    else if snip_prefix("ヴェ", text) { Some("ve") }
    else if snip_prefix("ヴォ", text) { Some("vo") }
    else if snip_prefix("ウ゛ァ", text) { Some("va") }
    else if snip_prefix("ウ゛ィ", text) { Some("vi") }
    else if snip_prefix("ウ゛", text) { Some("vu") }
    else if snip_prefix("ウ゛ェ", text) { Some("ve") }
    else if snip_prefix("ウ゛ォ", text) { Some("vo") }
    else if snip_prefix("ア", text) { Some("a") }
    else if snip_prefix("イ", text) { Some("i") }
    else if snip_prefix("ウ", text) { Some("u") }
    else if snip_prefix("エ", text) { Some("e") }
    else if snip_prefix("オ", text) { Some("o") }
    else if snip_prefix("カ", text) { Some("ka") }
    else if snip_prefix("キ", text) { Some("ki") }
    else if snip_prefix("ク", text) { Some("ku") }
    else if snip_prefix("ケ", text) { Some("ke") }
    else if snip_prefix("コ", text) { Some("ko") }
    else if snip_prefix("サ", text) { Some("sa") }
    else if snip_prefix("シ", text) { Some("shi") }
    else if snip_prefix("ス", text) { Some("su") }
    else if snip_prefix("セ", text) { Some("se") }
    else if snip_prefix("ソ", text) { Some("so") }
    else if snip_prefix("タ", text) { Some("ta") }
    else if snip_prefix("チ", text) { Some("chi") }
    else if snip_prefix("ツ", text) { Some("tsu") }
    else if snip_prefix("テ", text) { Some("te") }
    else if snip_prefix("ト", text) { Some("to") }
    else if snip_prefix("ナ", text) { Some("na") }
    else if snip_prefix("ニ", text) { Some("ni") }
    else if snip_prefix("ヌ", text) { Some("nu") }
    else if snip_prefix("ネ", text) { Some("ne") }
    else if snip_prefix("ノ", text) { Some("no") }
    else if snip_prefix("ハ", text) { Some("ha") }
    else if snip_prefix("ヒ", text) { Some("hi") }
    else if snip_prefix("フ", text) { Some("fu") }
    else if snip_prefix("ヘ", text) { Some("he") }
    else if snip_prefix("ホ", text) { Some("ho") }
    else if snip_prefix("マ", text) { Some("ma") }
    else if snip_prefix("ミ", text) { Some("mi") }
    else if snip_prefix("ム", text) { Some("mu") }
    else if snip_prefix("メ", text) { Some("me") }
    else if snip_prefix("モ", text) { Some("mo") }
    else if snip_prefix("ヤ", text) { Some("ya") }
    else if snip_prefix("ユ", text) { Some("yu") }
    else if snip_prefix("ヨ", text) { Some("yo") }
    else if snip_prefix("ラ", text) { Some("ra") }
    else if snip_prefix("リ", text) { Some("ri") }
    else if snip_prefix("ル", text) { Some("ru") }
    else if snip_prefix("レ", text) { Some("re") }
    else if snip_prefix("ロ", text) { Some("ro") }
    else if snip_prefix("ワ", text) { Some("wa") }
    else if snip_prefix("ヲ", text) { Some("wo") }
    else if snip_prefix("ン", text) { Some("n") }
    else if snip_prefix("ガ", text) { Some("ga") }
    else if snip_prefix("ギ", text) { Some("gi") }
    else if snip_prefix("グ", text) { Some("gu") }
    else if snip_prefix("ゲ", text) { Some("ge") }
    else if snip_prefix("ゴ", text) { Some("go") }
    else if snip_prefix("ザ", text) { Some("za") }
    else if snip_prefix("ジ", text) { Some("ji") }
    else if snip_prefix("ズ", text) { Some("zu") }
    else if snip_prefix("ゼ", text) { Some("ze") }
    else if snip_prefix("ゾ", text) { Some("zo") }
    else if snip_prefix("ダ", text) { Some("da") }
    else if snip_prefix("ヂ", text) { Some("ji") }
    else if snip_prefix("ヅ", text) { Some("zu") }
    else if snip_prefix("デ", text) { Some("de") }
    else if snip_prefix("ド", text) { Some("do") }
    else if snip_prefix("バ", text) { Some("ba") }
    else if snip_prefix("ビ", text) { Some("bi") }
    else if snip_prefix("ブ", text) { Some("bu") }
    else if snip_prefix("ベ", text) { Some("be") }
    else if snip_prefix("ボ", text) { Some("bo") }
    else if snip_prefix("パ", text) { Some("pa") }
    else if snip_prefix("ピ", text) { Some("pi") }
    else if snip_prefix("プ", text) { Some("pu") }
    else if snip_prefix("ペ", text) { Some("pe") }
    else if snip_prefix("ポ", text) { Some("po") }
    else if snip_prefix("ァ", text) { Some("a") }
    else if snip_prefix("ィ", text) { Some("i") }
    else if snip_prefix("ゥ", text) { Some("u") }
    else if snip_prefix("ェ", text) { Some("e") }
    else if snip_prefix("ォ", text) { Some("o") }
    else if snip_prefix("ャ", text) { Some("ya") }
    else if snip_prefix("ュ", text) { Some("yu") }
    else if snip_prefix("ョ", text) { Some("yo") }
    else if snip_prefix("ッ", text) { Some("tu") }
    else if snip_prefix("ヴ", text) { Some("vu") }
    else if snip_prefix("ー", text) { Some("-") }
    else if snip_prefix("。", text) { Some(".") }
    else if snip_prefix("、", text) { Some(",") }
    else if snip_prefix("！", text) { Some("!") }
    else if snip_prefix("？", text) { Some("?") }
    else if snip_prefix("　", text) { Some(" ") }
    else {
        None
    }
}