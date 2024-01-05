use anyhow::Result;
use core::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, error::Error};

#[allow(dead_code)]
pub fn kana_to_romaji(word: &str) -> String {
    let transformed: String = word
        .chars()
        .map(|c| {
            KANA_TO_ENGLISH
                .get(&c)
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

fn could_be_romaji(word: &str) -> bool {
    let allowed_characters = Regex::new(r"^[a-pr-z\-']*$").unwrap();
    allowed_characters.captures(word).is_none()
}

#[allow(dead_code)]
fn convert_single_letter<'a>(letter: u8) -> Result<Option<&'a String>> {
    let single_letter_word = String::from_utf8(vec![letter])?;
    Ok(ONE_LETTER_TO_KATAKANA.get(&single_letter_word))
}

#[allow(dead_code)]
fn convert_double_letter<'a>(l1: u8, l2: u8) -> Result<Option<&'a String>> {
    let double_letter_word = String::from_utf8(vec![l1, l2])?;
    Ok(TWO_LETTER_TO_KATAKANA.get(&double_letter_word))
}

#[allow(dead_code)]
fn convert_triple_letter<'a>(l1: u8, l2: u8, l3: u8) -> Result<Option<&'a String>> {
    let triple_letter_word = String::from_utf8(vec![l1, l2, l3])?;
    Ok(THREE_LETTER_TO_KATAKANA.get(&triple_letter_word))
}

#[allow(dead_code)]
fn convert_quadruple_letter<'a>(l1: u8, l2: u8, l3: u8, l4: u8) -> Result<Option<&'a String>> {
    let quadruple_letter_word = String::from_utf8(vec![l1, l2, l3, l4])?;
    Ok(FOUR_LETTER_TO_KATAKANA.get(&quadruple_letter_word))
}

#[allow(dead_code)]
pub fn romaji_to_katakana(word: &str) -> Result<String> {
    if could_be_romaji(word) {
        return Err(NotConvertibleError::new(word).into());
    }
    let mut katakana = String::new();
    let chars: Vec<u8> = String::from(word).into_bytes();
    let mut index = 0;
    while index < chars.len() {
        let l1 = chars[index];
        let l2 = chars.get(index + 1).copied();
        let l3 = chars.get(index + 2).copied();
        let l4 = chars.get(index + 3).copied();
        katakana += {
            index += 2;
            l2.and_then(|l2| {
                if l2 == l1 && SOUKUN_CHARS.contains(&l1) {
                    index -= 1;
                    let small_tsu = String::from("xtsu");
                    FOUR_LETTER_TO_KATAKANA.get(&small_tsu)
                } else {
                    convert_double_letter(l1, l2).expect("Character should be valid utf8")
                }
            })
            .or_else(|| {
                index += 1;
                l3.and_then(|l3| {
                    convert_triple_letter(l1, l2.unwrap_or_default(), l3)
                        .expect("Character should be valid utf8")
                })
            })
            .or_else(|| {
                index -= 2;
                convert_single_letter(l1).expect("Character should be valid utf8")
            })
            .or_else(|| {
                index += 3;
                l4.and_then(|l4| {
                    convert_quadruple_letter(l1, l2.unwrap_or_default(), l3.unwrap_or_default(), l4)
                        .expect("Character should be valid utf8")
                })
            })
            .ok_or_else(|| NotConvertibleError::new(word))?
        }
    }
    Ok(katakana)
}

#[allow(dead_code)]
pub fn romaji_to_hiragana(word: &str) -> Result<String> {
    romaji_to_katakana(word)
        .map(|katakana| katakana_to_hiragana(&katakana).expect("Should be valid hiragana"))
}

#[allow(dead_code)]
pub fn katakana_to_hiragana(word: &str) -> Result<String> {
    if !word.chars().all(|c| HIRAGANA_CHARS.contains(&(c as u16))) {
        return Err(NotConvertibleError::new(word).into());
    }
    let as_u16: Vec<u16> = word.chars().map(|char| char as u16 - 0x60).collect();
    Ok(String::from_utf16(&as_u16).expect("Should be valid utf8"))
}

#[derive(Debug)]
struct NotConvertibleError {
    word: String,
}

impl NotConvertibleError {
    #[allow(unused)]
    fn new(word: &str) -> NotConvertibleError {
        NotConvertibleError {
            word: word.to_string(),
        }
    }
}

impl Error for NotConvertibleError {}

impl fmt::Display for NotConvertibleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not convertible", self.word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_romaji_to_hiragana() {
        assert_eq!(romaji_to_hiragana("").unwrap(), "");
        assert_eq!(romaji_to_hiragana("sayonara").unwrap(), "さよなら");
        assert_eq!(romaji_to_hiragana("tsukue").unwrap(), "つくえ");
        assert_eq!(romaji_to_hiragana("tukue").unwrap(), "つくえ");
        assert_eq!(romaji_to_hiragana("kinnyoubi").unwrap(), "きんようび");
        assert_eq!(romaji_to_hiragana("kin'youbi").unwrap(), "きんようび");
        assert_eq!(romaji_to_hiragana("konnya").unwrap(), "こんや");
        assert_eq!(romaji_to_hiragana("konnnichi").unwrap(), "こんにち");
        assert_eq!(romaji_to_hiragana("kaetta").unwrap(), "かえった");
        assert_eq!(romaji_to_hiragana("kon'nichiha").unwrap(), "こんにちは");
        assert_eq!(romaji_to_hiragana("arigatou").unwrap(), "ありがとう");
        assert_eq!(romaji_to_hiragana("ohayou").unwrap(), "おはよう");
        assert_eq!(romaji_to_hiragana("nihon").unwrap(), "にほん");
        assert_eq!(romaji_to_hiragana("kyou").unwrap(), "きょう");
        assert_eq!(romaji_to_hiragana("gakkou").unwrap(), "がっこう");
        assert_eq!(romaji_to_hiragana("ryokou").unwrap(), "りょこう");
        assert_eq!(romaji_to_hiragana("shashin").unwrap(), "しゃしん");
        assert_eq!(romaji_to_hiragana("chikatetsu").unwrap(), "ちかてつ");
        assert_eq!(romaji_to_hiragana("jyuusho").unwrap(), "じゅうしょ");
        assert_eq!(romaji_to_hiragana("fuji").unwrap(), "ふじ");
        assert_eq!(romaji_to_hiragana("boxtsuti").unwrap(), "ぼっち");
    }

    #[test]
    fn test_romaji_to_katakana() {
        assert_eq!(romaji_to_katakana("konnbann").unwrap(), "コンバン");
        assert_eq!(romaji_to_katakana("tsukue").unwrap(), "ツクエ");
        assert_eq!(romaji_to_katakana("tukue").unwrap(), "ツクエ");
        assert_eq!(romaji_to_katakana("kinnyoubi").unwrap(), "キンヨウビ");
        assert_eq!(romaji_to_katakana("kin'youbi").unwrap(), "キンヨウビ");
        assert_eq!(romaji_to_katakana("konnya").unwrap(), "コンヤ");
        assert_eq!(romaji_to_katakana("konnnichi").unwrap(), "コンニチ");
        assert_eq!(romaji_to_katakana("kaetta").unwrap(), "カエッタ");
        assert_eq!(romaji_to_katakana("arigatou").unwrap(), "アリガトウ");
        assert_eq!(romaji_to_katakana("ohayou").unwrap(), "オハヨウ");
        assert_eq!(romaji_to_katakana("nihon").unwrap(), "ニホン");
        assert_eq!(romaji_to_katakana("kyou").unwrap(), "キョウ");
        assert_eq!(romaji_to_katakana("gakkou").unwrap(), "ガッコウ");
        assert_eq!(romaji_to_katakana("ryokou").unwrap(), "リョコウ");
        assert_eq!(romaji_to_katakana("shashin").unwrap(), "シャシン");
        assert_eq!(romaji_to_katakana("chikatetsu").unwrap(), "チカテツ");
        assert_eq!(romaji_to_katakana("jyuusho").unwrap(), "ジュウショ");
        assert_eq!(romaji_to_katakana("fuji").unwrap(), "フジ");
    }

    #[test]
    fn test_katakana_to_hiragana() {
        assert_eq!(katakana_to_hiragana("サヨナラ").unwrap(), "さよなら");
        assert_eq!(katakana_to_hiragana("アリガトウ").unwrap(), "ありがとう");
        assert_eq!(katakana_to_hiragana("オハヨウ").unwrap(), "おはよう");
        assert_eq!(katakana_to_hiragana("ニホン").unwrap(), "にほん");
        assert_eq!(katakana_to_hiragana("キョウ").unwrap(), "きょう");
        assert_eq!(katakana_to_hiragana("ガッコウ").unwrap(), "がっこう");
        assert_eq!(katakana_to_hiragana("リョコウ").unwrap(), "りょこう");
        assert_eq!(katakana_to_hiragana("シャシン").unwrap(), "しゃしん");
        assert_eq!(katakana_to_hiragana("チカテツ").unwrap(), "ちかてつ");
        assert_eq!(katakana_to_hiragana("ジュウショ").unwrap(), "じゅうしょ");
        assert_eq!(katakana_to_hiragana("フジ").unwrap(), "ふじ");
    }
}

lazy_static! {
    static ref KANA_TO_ENGLISH: HashMap<char, String> = {
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
            ('ヱ', "e".to_string()),
            ('ー', "".to_string()),
            ('・', " ".to_string()),
            ('、', ",".to_string()),
            ('。', ".".to_string()),
            ('　', " ".to_string()),
        ]
        .into_iter()
        .collect()
    };
}

lazy_static! {
    static ref ONE_LETTER_TO_KATAKANA: HashMap<String, String> = {
        vec![
            ("a".to_string(), "ア".to_string()),
            ("i".to_string(), "イ".to_string()),
            ("u".to_string(), "ウ".to_string()),
            ("e".to_string(), "エ".to_string()),
            ("o".to_string(), "オ".to_string()),
            ("n".to_string(), "ン".to_string()),
            ("-".to_string(), "ー".to_string()),
            ("'".to_string(), "".to_string()),
        ]
        .into_iter()
        .collect()
    };
}

lazy_static! {
    static ref TWO_LETTER_TO_KATAKANA: HashMap<String, String> = {
        vec![
            ("xa".to_string(), "ァ".to_string()),
            ("xi".to_string(), "ィ".to_string()),
            ("xu".to_string(), "ゥ".to_string()),
            ("xe".to_string(), "ェ".to_string()),
            ("xo".to_string(), "ォ".to_string()),
            ("ka".to_string(), "カ".to_string()),
            ("ki".to_string(), "キ".to_string()),
            ("ku".to_string(), "ク".to_string()),
            ("ke".to_string(), "ケ".to_string()),
            ("ko".to_string(), "コ".to_string()),
            ("ca".to_string(), "カ".to_string()),
            ("cu".to_string(), "ク".to_string()),
            ("co".to_string(), "コ".to_string()),
            ("ga".to_string(), "ガ".to_string()),
            ("gi".to_string(), "ギ".to_string()),
            ("gu".to_string(), "グ".to_string()),
            ("ge".to_string(), "ゲ".to_string()),
            ("go".to_string(), "ゴ".to_string()),
            ("sa".to_string(), "サ".to_string()),
            ("si".to_string(), "シ".to_string()),
            ("su".to_string(), "ス".to_string()),
            ("se".to_string(), "セ".to_string()),
            ("so".to_string(), "ソ".to_string()),
            ("za".to_string(), "ザ".to_string()),
            ("zi".to_string(), "ジ".to_string()),
            ("zu".to_string(), "ズ".to_string()),
            ("ze".to_string(), "ゼ".to_string()),
            ("zo".to_string(), "ゾ".to_string()),
            ("ja".to_string(), "ジャ".to_string()),
            ("ji".to_string(), "ジ".to_string()),
            ("ju".to_string(), "ジュ".to_string()),
            ("je".to_string(), "ジェ".to_string()),
            ("jo".to_string(), "ジョ".to_string()),
            ("ta".to_string(), "タ".to_string()),
            ("ti".to_string(), "チ".to_string()),
            ("tu".to_string(), "ツ".to_string()),
            ("te".to_string(), "テ".to_string()),
            ("to".to_string(), "ト".to_string()),
            ("da".to_string(), "ダ".to_string()),
            ("di".to_string(), "ヂ".to_string()),
            ("du".to_string(), "ヅ".to_string()),
            ("de".to_string(), "デ".to_string()),
            ("do".to_string(), "ド".to_string()),
            ("na".to_string(), "ナ".to_string()),
            ("ni".to_string(), "ニ".to_string()),
            ("nu".to_string(), "ヌ".to_string()),
            ("ne".to_string(), "ネ".to_string()),
            ("no".to_string(), "ノ".to_string()),
            ("ha".to_string(), "ハ".to_string()),
            ("hi".to_string(), "ヒ".to_string()),
            ("hu".to_string(), "フ".to_string()),
            ("he".to_string(), "ヘ".to_string()),
            ("ho".to_string(), "ホ".to_string()),
            ("ba".to_string(), "バ".to_string()),
            ("bi".to_string(), "ビ".to_string()),
            ("bu".to_string(), "ブ".to_string()),
            ("be".to_string(), "ベ".to_string()),
            ("bo".to_string(), "ボ".to_string()),
            ("pa".to_string(), "パ".to_string()),
            ("pi".to_string(), "ピ".to_string()),
            ("pu".to_string(), "プ".to_string()),
            ("pe".to_string(), "ペ".to_string()),
            ("po".to_string(), "ポ".to_string()),
            ("va".to_string(), "ヴァ".to_string()),
            ("vi".to_string(), "ヴィ".to_string()),
            ("vu".to_string(), "ヴ".to_string()),
            ("ve".to_string(), "ヴェ".to_string()),
            ("vo".to_string(), "ヴォ".to_string()),
            ("fa".to_string(), "ファ".to_string()),
            ("fi".to_string(), "フィ".to_string()),
            ("fu".to_string(), "フ".to_string()),
            ("fe".to_string(), "フェ".to_string()),
            ("fo".to_string(), "フォ".to_string()),
            ("ma".to_string(), "マ".to_string()),
            ("mi".to_string(), "ミ".to_string()),
            ("mu".to_string(), "ム".to_string()),
            ("me".to_string(), "メ".to_string()),
            ("mo".to_string(), "モ".to_string()),
            ("ya".to_string(), "ヤ".to_string()),
            ("yi".to_string(), "イ".to_string()),
            ("yu".to_string(), "ユ".to_string()),
            ("ye".to_string(), "イェ".to_string()),
            ("yo".to_string(), "ヨ".to_string()),
            ("ra".to_string(), "ラ".to_string()),
            ("ri".to_string(), "リ".to_string()),
            ("ru".to_string(), "ル".to_string()),
            ("re".to_string(), "レ".to_string()),
            ("ro".to_string(), "ロ".to_string()),
            ("la".to_string(), "ラ".to_string()),
            ("li".to_string(), "リ".to_string()),
            ("lu".to_string(), "ル".to_string()),
            ("le".to_string(), "レ".to_string()),
            ("lo".to_string(), "ロ".to_string()),
            ("wa".to_string(), "ワ".to_string()),
            ("wi".to_string(), "ヰ".to_string()),
            ("wu".to_string(), "ウ".to_string()),
            ("we".to_string(), "ヱ".to_string()),
            ("wo".to_string(), "ヲ".to_string()),
            ("nn".to_string(), "ン".to_string()),
        ]
        .into_iter()
        .collect()
    };
}

lazy_static! {
    static ref THREE_LETTER_TO_KATAKANA: HashMap<String, String> = {
        vec![
            ("tsu".to_string(), "ツ".to_string()),
            ("xka".to_string(), "ヵ".to_string()),
            ("xke".to_string(), "ヶ".to_string()),
            ("xwa".to_string(), "ヮ".to_string()),
            ("xya".to_string(), "ャ".to_string()),
            ("xyu".to_string(), "ュ".to_string()),
            ("xyo".to_string(), "ョ".to_string()),
            ("kya".to_string(), "キャ".to_string()),
            ("kyi".to_string(), "キィ".to_string()),
            ("kyu".to_string(), "キュ".to_string()),
            ("kye".to_string(), "キェ".to_string()),
            ("kyo".to_string(), "キョ".to_string()),
            ("gya".to_string(), "ギャ".to_string()),
            ("gyi".to_string(), "ギィ".to_string()),
            ("gyu".to_string(), "ギュ".to_string()),
            ("gye".to_string(), "ギェ".to_string()),
            ("gyo".to_string(), "ギョ".to_string()),
            ("sya".to_string(), "シャ".to_string()),
            ("syi".to_string(), "シィ".to_string()),
            ("syu".to_string(), "シュ".to_string()),
            ("sye".to_string(), "シェ".to_string()),
            ("syo".to_string(), "ショ".to_string()),
            ("sha".to_string(), "シャ".to_string()),
            ("shi".to_string(), "シ".to_string()),
            ("shu".to_string(), "シュ".to_string()),
            ("she".to_string(), "シェ".to_string()),
            ("sho".to_string(), "ショ".to_string()),
            ("zya".to_string(), "ジャ".to_string()),
            ("zyi".to_string(), "ジィ".to_string()),
            ("zyu".to_string(), "ジュ".to_string()),
            ("zye".to_string(), "ジェ".to_string()),
            ("zyo".to_string(), "ジョ".to_string()),
            ("jya".to_string(), "ジャ".to_string()),
            ("jyi".to_string(), "ジィ".to_string()),
            ("jyu".to_string(), "ジュ".to_string()),
            ("jye".to_string(), "ジェ".to_string()),
            ("jyo".to_string(), "ジョ".to_string()),
            ("tya".to_string(), "チャ".to_string()),
            ("tyi".to_string(), "チィ".to_string()),
            ("tyu".to_string(), "チュ".to_string()),
            ("tye".to_string(), "チェ".to_string()),
            ("tyo".to_string(), "チョ".to_string()),
            ("cya".to_string(), "チャ".to_string()),
            ("cyi".to_string(), "チィ".to_string()),
            ("cyu".to_string(), "チュ".to_string()),
            ("cye".to_string(), "チェ".to_string()),
            ("cyo".to_string(), "チョ".to_string()),
            ("cha".to_string(), "チャ".to_string()),
            ("chi".to_string(), "チ".to_string()),
            ("chu".to_string(), "チュ".to_string()),
            ("che".to_string(), "チェ".to_string()),
            ("cho".to_string(), "チョ".to_string()),
            ("tha".to_string(), "テャ".to_string()),
            ("thi".to_string(), "ティ".to_string()),
            ("thu".to_string(), "テュ".to_string()),
            ("the".to_string(), "テェ".to_string()),
            ("tho".to_string(), "テョ".to_string()),
            ("dya".to_string(), "ヂャ".to_string()),
            ("dyi".to_string(), "ヂィ".to_string()),
            ("dyu".to_string(), "ヂュ".to_string()),
            ("dye".to_string(), "ヂェ".to_string()),
            ("dyo".to_string(), "ヂョ".to_string()),
            ("dha".to_string(), "デャ".to_string()),
            ("dhi".to_string(), "ディ".to_string()),
            ("dhu".to_string(), "デュ".to_string()),
            ("dhe".to_string(), "デェ".to_string()),
            ("dho".to_string(), "デョ".to_string()),
            ("nya".to_string(), "ニャ".to_string()),
            ("nyi".to_string(), "ニィ".to_string()),
            ("nyu".to_string(), "ニュ".to_string()),
            ("nye".to_string(), "ニェ".to_string()),
            ("nyo".to_string(), "ニョ".to_string()),
            ("hya".to_string(), "ヒャ".to_string()),
            ("hyi".to_string(), "ヒィ".to_string()),
            ("hyu".to_string(), "ヒュ".to_string()),
            ("hye".to_string(), "ヒェ".to_string()),
            ("hyo".to_string(), "ヒョ".to_string()),
            ("bya".to_string(), "ビャ".to_string()),
            ("byi".to_string(), "ビィ".to_string()),
            ("byu".to_string(), "ビュ".to_string()),
            ("bye".to_string(), "ビェ".to_string()),
            ("byo".to_string(), "ビョ".to_string()),
            ("pya".to_string(), "ピャ".to_string()),
            ("pyi".to_string(), "ピィ".to_string()),
            ("pyu".to_string(), "ピュ".to_string()),
            ("pye".to_string(), "ピェ".to_string()),
            ("pyo".to_string(), "ピョ".to_string()),
            ("mya".to_string(), "ミャ".to_string()),
            ("myi".to_string(), "ミィ".to_string()),
            ("myu".to_string(), "ミュ".to_string()),
            ("mye".to_string(), "ミェ".to_string()),
            ("myo".to_string(), "ミョ".to_string()),
            ("rya".to_string(), "リャ".to_string()),
            ("ryi".to_string(), "リィ".to_string()),
            ("ryu".to_string(), "リュ".to_string()),
            ("rye".to_string(), "リェ".to_string()),
            ("ryo".to_string(), "リョ".to_string()),
            ("lya".to_string(), "リャ".to_string()),
            ("lyi".to_string(), "リィ".to_string()),
            ("lyu".to_string(), "リュ".to_string()),
            ("lye".to_string(), "リェ".to_string()),
            ("lyo".to_string(), "リョ".to_string()),
        ]
        .into_iter()
        .collect()
    };
}

lazy_static! {
    static ref FOUR_LETTER_TO_KATAKANA: HashMap<String, String> = {
        vec![("xtsu".to_string(), "ッ".to_string())]
            .into_iter()
            .collect()
    };
}

const SOUKUN_CHARS: [u8; 19] = [
    98, 99, 100, 102, 103, 104, 106, 107, 108, 109, 112, 114, 115, 116, 118, 119, 120, 121, 122,
];

const HIRAGANA_CHARS: std::ops::RangeInclusive<u16> = 0x3041..=0x30fe;

#[allow(dead_code)]
const KATAKANA_CHARS: std::ops::RangeInclusive<u16> = 0x30A0..=0x30FF;

lazy_static! {
    pub static ref KANJI_CHARS: Regex = Regex::new(r"\p{Han}").unwrap();
}

#[allow(dead_code)]
const KANJI_RADICALS: std::ops::RangeInclusive<u16> = 0x2E80..=0x2FD5;
