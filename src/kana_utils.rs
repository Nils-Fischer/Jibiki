use regex::Regex;
use std::collections::HashMap;

fn translation_map() -> HashMap<char, String> {
    vec![
        ('あ', "a".to_string()),
        ('い', "i".to_string()),
        ('う', "u".to_string()),
        ('え', "e".to_string()),
        ('お', "o".to_string()),
        ('か', "ka".to_string()),
        ('が', "ga".to_string()),
        ('き', "ki".to_string()),
        ('ぎ', "gi".to_string()),
        ('く', "ku".to_string()),
        ('ぐ', "gu".to_string()),
        ('け', "ke".to_string()),
        ('げ', "ge".to_string()),
        ('こ', "ko".to_string()),
        ('ご', "go".to_string()),
        ('さ', "sa".to_string()),
        ('ざ', "za".to_string()),
        ('し', "shi".to_string()),
        ('じ', "ji".to_string()),
        ('す', "su".to_string()),
        ('ず', "zu".to_string()),
        ('せ', "se".to_string()),
        ('ぜ', "ze".to_string()),
        ('そ', "so".to_string()),
        ('ぞ', "zo".to_string()),
        ('た', "ta".to_string()),
        ('だ', "da".to_string()),
        ('ち', "chi".to_string()),
        ('ぢ', "ji".to_string()),
        ('つ', "tsu".to_string()),
        ('づ', "zu".to_string()),
        ('て', "te".to_string()),
        ('で', "de".to_string()),
        ('と', "to".to_string()),
        ('ど', "do".to_string()),
        ('な', "na".to_string()),
        ('に', "ni".to_string()),
        ('ぬ', "nu".to_string()),
        ('ね', "ne".to_string()),
        ('の', "no".to_string()),
        ('は', "ha".to_string()),
        ('ば', "ba".to_string()),
        ('ぱ', "pa".to_string()),
        ('ひ', "hi".to_string()),
        ('び', "bi".to_string()),
        ('ぴ', "pi".to_string()),
        ('ふ', "fu".to_string()),
        ('ぶ', "bu".to_string()),
        ('ぷ', "pu".to_string()),
        ('へ', "he".to_string()),
        ('べ', "be".to_string()),
        ('ぺ', "pe".to_string()),
        ('ほ', "ho".to_string()),
        ('ぼ', "bo".to_string()),
        ('ぽ', "po".to_string()),
        ('ま', "ma".to_string()),
        ('み', "mi".to_string()),
        ('む', "mu".to_string()),
        ('め', "me".to_string()),
        ('も', "mo".to_string()),
        ('や', "ya".to_string()),
        ('ゆ', "yu".to_string()),
        ('よ', "yo".to_string()),
        ('ら', "ra".to_string()),
        ('り', "ri".to_string()),
        ('る', "ru".to_string()),
        ('れ', "re".to_string()),
        ('ろ', "ro".to_string()),
        ('わ', "wa".to_string()),
        ('を', "wo".to_string()),
        ('ん', "n".to_string()),
        ('ゔ', "vu".to_string()),
        ('ゐ', "vi".to_string()),
        ('ゑ', "e".to_string()),
        ('ゃ', "-ya".to_string()),
        ('ゅ', "-yu".to_string()),
        ('ょ', "-yo".to_string()),
        ('ぁ', "a".to_string()),
        ('ぅ', "u".to_string()),
        ('ぇ', "e".to_string()),
        ('ぉ', "o".to_string()),
        ('ぃ', "i".to_string()),
        ('っ', "+".to_string()),
        ('ゖ', "ke".to_string()),
        ('ゕ', "ka".to_string()),
        ('ゎ', "wa".to_string()),
        ('ァ', "a".to_string()),
        ('ア', "a".to_string()),
        ('ィ', "i".to_string()),
        ('イ', "i".to_string()),
        ('ゥ', "u".to_string()),
        ('ウ', "u".to_string()),
        ('ェ', "e".to_string()),
        ('エ', "e".to_string()),
        ('ォ', "o".to_string()),
        ('オ', "o".to_string()),
        ('カ', "ka".to_string()),
        ('ガ', "ga".to_string()),
        ('キ', "ki".to_string()),
        ('ギ', "gi".to_string()),
        ('ク', "ku".to_string()),
        ('グ', "gu".to_string()),
        ('ケ', "ke".to_string()),
        ('ゲ', "ge".to_string()),
        ('コ', "ko".to_string()),
        ('ゴ', "go".to_string()),
        ('サ', "se".to_string()),
        ('ザ', "ze".to_string()),
        ('シ', "shi".to_string()),
        ('ジ', "ji".to_string()),
        ('ス', "su".to_string()),
        ('ズ', "zu".to_string()),
        ('セ', "se".to_string()),
        ('ゼ', "ze".to_string()),
        ('ソ', "so".to_string()),
        ('ゾ', "zo".to_string()),
        ('タ', "ta".to_string()),
        ('ダ', "da".to_string()),
        ('チ', "chi".to_string()),
        ('ヂ', "ji".to_string()),
        ('ッ', "+".to_string()),
        ('ツ', "tsu".to_string()),
        ('ヅ', "zu".to_string()),
        ('テ', "te".to_string()),
        ('デ', "de".to_string()),
        ('ト', "to".to_string()),
        ('ド', "do".to_string()),
        ('ナ', "na".to_string()),
        ('ニ', "ni".to_string()),
        ('ヌ', "nu".to_string()),
        ('ネ', "ne".to_string()),
        ('ノ', "no".to_string()),
        ('ハ', "ha".to_string()),
        ('バ', "ba".to_string()),
        ('パ', "pa".to_string()),
        ('ヒ', "hi".to_string()),
        ('ビ', "bi".to_string()),
        ('ピ', "pi".to_string()),
        ('フ', "fu".to_string()),
        ('ブ', "bu".to_string()),
        ('プ', "pu".to_string()),
        ('ヘ', "he".to_string()),
        ('ベ', "be".to_string()),
        ('ペ', "pe".to_string()),
        ('ホ', "ho".to_string()),
        ('ボ', "bo".to_string()),
        ('ポ', "po".to_string()),
        ('マ', "ma".to_string()),
        ('ミ', "mi".to_string()),
        ('ム', "mu".to_string()),
        ('メ', "me".to_string()),
        ('モ', "mo".to_string()),
        ('ャ', "-ya".to_string()),
        ('ヤ', "ya".to_string()),
        ('ュ', "-yu".to_string()),
        ('ユ', "yu".to_string()),
        ('ョ', "-yo".to_string()),
        ('ヨ', "yo".to_string()),
        ('ラ', "ra".to_string()),
        ('リ', "ri".to_string()),
        ('ル', "ru".to_string()),
        ('レ', "re".to_string()),
        ('ロ', "ro".to_string()),
        ('ヮ', "wa".to_string()),
        ('ワ', "wa".to_string()),
        ('ヰ', "wi".to_string()),
        ('ヲ', "wo".to_string()),
        ('ン', "n".to_string()),
        ('ヴ', "vu".to_string()),
        ('ヵ', "ka".to_string()),
        ('ヶ', "ke".to_string()),
        ('ー', "".to_string()),
        ('・', " ".to_string()),
        ('、', ",".to_string()),
        ('ヱ', "e".to_string()),
        ('。', ".".to_string()),
        ('　', " ".to_string()),
    ]
    .into_iter()
    .collect()
}

pub fn kana_to_romaji(word: &str) -> String {
    let map: HashMap<char, String> = translation_map();
    let transformed: String = word
        .chars()
        .map(|c| {
            map.get(&c)
                .unwrap_or_else(|| panic!("Unknown character in word: {}", word))
                .as_str()
        })
        .collect::<Vec<&str>>()
        .join("");
    let minus = Regex::new(r".\-").unwrap();
    let plus = Regex::new(r"\+(\w)").unwrap();
    let transformed = minus.replace_all(&transformed, "");
    let transformed = plus.replace_all(&transformed, "${1}${1}");
    transformed.to_string()
}
