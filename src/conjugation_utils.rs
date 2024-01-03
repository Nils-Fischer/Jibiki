fn generate_godan_conjugations(verb: &str, category: &str) -> String {
    match category {
        // Ichidan
        "v1" => todo!(),   // ichidan verb
        "v1-s" => todo!(), // ichidan verb
        // Godan
        "v5r" => todo!(),   // ru-ending
        "v5r-i" => todo!(), // ru-ending (irregular)
        "v5k" => todo!(),   // ku-ending
        "v5k-s" => todo!(), // iku/yuku (special case)
        "v5n" => todo!(),   // nu-ending
        "v5m" => todo!(),   // mu-ending
        "v5aru" => todo!(), // aru-ending (special class)
        "v5b" => todo!(),   // bu-ending
        "v5s" => todo!(),   // su-ending
        "v5u" => todo!(),   // u-ending
        "v5u-s" => todo!(), // u-ending (special class)
        "v5g" => todo!(),   // gu-ending
        "v5t" => todo!(),   // tsu-ending
        // Irregular
        "vk" => todo!(),   // kuru verbs
        "vs-i" => todo!(), // suru nouns
        "vs-s" => todo!(), // suru nouns (specials class)
        "vz" => todo!(),   // zuru verbs
        _ => panic!("format '{}' unknown", category),
    }
}

fn v1(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("ない", 1),
        "past" => ("た", 1),
        "past negative" => ("なかった", 1),
        "te form" => ("て", 1),
        "tai form" => ("たい", 1),
        "volitional" => ("よう", 1),
        "imperative" => ("ろ", 1),
        "passive" => ("られる", 1),
        "conditional" => ("たら", 1),
        "provisional conditional" => ("れば", 1),
        "causative" => ("させる", 1),
        "potential" => ("られる", 1),
        // polite
        "polite present" => ("ます", 1),
        "polite negative" => ("ません", 1),
        "polite past" => ("ました", 1),
        "polite past negative" => ("ませんでした", 1),
        "polite te form" => ("まして", 1),
        "polite volitional" => ("ましょう", 1),
        "polite imperative" => ("てください", 1),
        "polite passive" => ("られます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v1_s(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("ない", 1),
        "past" => ("た", 1),
        "past negative" => ("なかった", 1),
        "te form" => ("て", 1),
        "tai form" => ("たい", 1),
        "volitional" => ("よう", 1),
        "imperative" => ("", 0),
        "passive" => ("られる", 1),
        "conditional" => ("たら", 1),
        "provisional conditional" => ("れば", 1),
        "causative" => ("させる", 1),
        "potential" => ("られる", 1),
        // polite
        "polite present" => ("ます", 1),
        "polite negative" => ("ません", 1),
        "polite past" => ("ました", 1),
        "polite past negative" => ("ませんでした", 1),
        "polite te form" => ("まして", 1),
        "polite volitional" => ("ましょう", 1),
        "polite imperative" => ("てください", 1),
        "polite passive" => ("られます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5r(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("らない", 1),
        "past" => ("った", 1),
        "past negative" => ("らなかった", 1),
        "te form" => ("って", 1),
        "tai form" => ("りたい", 1),
        "volitional" => ("ろう", 1),
        "imperative" => ("れ", 1),
        "passive" => ("られる", 1),
        "conditional" => ("ったら", 1),
        "provisional conditional" => ("れば", 1),
        "causative" => ("らせる", 1),
        "potential" => ("れる", 1),
        // polite
        "polite present" => ("ります", 1),
        "polite negative" => ("りません", 1),
        "polite past" => ("りました", 1),
        "polite past negative" => ("りませんでした", 1),
        "polite te form" => ("りまして", 1),
        "polite volitional" => ("りましょう", 1),
        "polive imperative" => ("ってください", 1),
        "polite passive" => ("られます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5r_i(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("ない", 2),
        "past" => ("った", 1),
        "past negative" => ("なかった", 2),
        "te form" => ("って", 1),
        "tai form" => ("りたい", 1),
        "volitional" => ("ろう", 1),
        "imperative" => ("れ", 1),
        "passive" => ("られる", 1),
        "conditional" => ("ったら", 1),
        "provisional conditional" => ("れば", 1),
        "causative" => ("らせる", 1),
        "potential" => ("りえる", 1),
        // polite
        "polite present" => ("ります", 1),
        "polite negative" => ("りません", 1),
        "polite past" => ("りました", 1),
        "polite past negative" => ("りませんでした", 1),
        "polite te form" => ("りまして", 1),
        "polite volitional" => ("りましょう", 1),
        "polive imperative" => ("ってください", 1),
        "polite passive" => ("られます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5k(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("かない", 1),
        "past" => ("いた", 1),
        "past negative" => ("かなかった", 1),
        "te form" => ("いて", 1),
        "tai form" => ("きたい", 1),
        "volitional" => ("こう", 1),
        "imperative" => ("け", 1),
        "passive" => ("かれる", 1),
        "conditional" => ("いたら", 1),
        "provisional conditional" => ("けば", 1),
        "causative" => ("かせる", 1),
        "potential" => ("ける", 1),
        // polite
        "polite present" => ("きます", 1),
        "polite negative" => ("きません", 1),
        "polite past" => ("きました", 1),
        "polite past negative" => ("きませんでした", 1),
        "polite te form" => ("きまして", 1),
        "polite volitional" => ("きましょう", 1),
        "polive imperative" => ("いてください", 1),
        "polite passive" => ("かれます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5k_s(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("かない", 1),
        "past" => ("った", 1),
        "past negative" => ("かなかった", 1),
        "te form" => ("って", 1),
        "tai form" => ("きたい", 1),
        "volitional" => ("こう", 1),
        "imperative" => ("け", 1),
        "passive" => ("かれる", 1),
        "conditional" => ("ったら", 1),
        "provisional conditional" => ("けば", 1),
        "causative" => ("かせる", 1),
        "potential" => ("ける", 1),
        // polite
        "polite present" => ("きます", 1),
        "polite negative" => ("きません", 1),
        "polite past" => ("きました", 1),
        "polite past negative" => ("きませんでした", 1),
        "polite te form" => ("きまして", 1),
        "polite volitional" => ("きましょう", 1),
        "polive imperative" => ("ってください", 1),
        "polite passive" => ("かれます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5n(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("なない", 1),
        "past" => ("んだ", 1),
        "past negative" => ("ななかった", 1),
        "te form" => ("んで", 1),
        "tai form" => ("にたい", 1),
        "volitional" => ("のう", 1),
        "imperative" => ("ね", 1),
        "passive" => ("なれる", 1),
        "conditional" => ("んだら", 1),
        "provisional conditional" => ("ねば", 1),
        "causative" => ("なせる", 1),
        "potential" => ("ねる", 1),
        // polite
        "polite present" => ("にます", 1),
        "polite negative" => ("にません", 1),
        "polite past" => ("にました", 1),
        "polite past negative" => ("にませんでした", 1),
        "polite te form" => ("にまして", 1),
        "polite volitional" => ("にましょう", 1),
        "polite imperative" => ("んでくださ", 1),
        "polite passive" => ("なれます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5m(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("まない", 1),
        "past" => ("んだ", 1),
        "past negative" => ("まなかった", 1),
        "te form" => ("んで", 1),
        "tai form" => ("みたい", 1),
        "volitional" => ("もう", 1),
        "imperative" => ("め", 1),
        "passive" => ("まれる", 1),
        "conditional" => ("んだら", 1),
        "provisional conditional" => ("めば", 1),
        "causative" => ("ませる", 1),
        "potential" => ("める", 1),
        "present" => ("みます", 1),
        // polite
        "polite present" => ("みます", 1),
        "polite negative" => ("みません", 1),
        "polite past" => ("みました", 1),
        "polite past negative" => ("みませんでした", 1),
        "polite te form" => ("みまして", 1),
        "polite volitional" => ("みましょう", 1),
        "polite imperative" => ("んでください", 1),
        "polite passive" => ("まれます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5aru(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("らない", 1),
        "past" => ("った", 1),
        "past negative" => ("らなかった", 1),
        "te form" => ("って", 1),
        "tai form" => ("いたい", 1),
        "volitional" => ("ろう", 1),
        "imperative" => ("い", 1),
        "passive" => ("られる", 1),
        "conditional" => ("ったら", 1),
        "provisional conditional" => ("れば", 1),
        "causative" => ("らせる", 1),
        "potential" => ("り得る", 1),
        // polite
        "polite present" => ("います", 1),
        "polite negative" => ("いません", 1),
        "polite past" => ("いました", 1),
        "polite past negative" => ("いませんでした", 1),
        "polite te form" => ("いまして", 1),
        "polite volitional" => ("いましょう", 1),
        "polite imperative" => ("ってください", 1),
        "polite passive" => ("られます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5b(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("ばない", 1),
        "past" => ("んだ", 1),
        "past negative" => ("ばなかった", 1),
        "te form" => ("んで", 1),
        "tai form" => ("びたい", 1),
        "volitional" => ("ぼう", 1),
        "imperative" => ("べ", 1),
        "passive" => ("ばれる", 1),
        "conditional" => ("んだら", 1),
        "provisional conditional" => ("べば", 1),
        "causative" => ("ばせる", 1),
        "potential" => ("べる", 1),
        // polite
        "polite present" => ("びます", 1),
        "polite negative" => ("びません", 1),
        "polite past" => ("びました", 1),
        "polite past negative" => ("びませんでした", 1),
        "polite te form" => ("びまして", 1),
        "polite volitional" => ("びましょう", 1),
        "polite imperative" => ("んでください", 1),
        "polite passive" => ("ばれます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5s(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("さない", 1),
        "past" => ("した", 1),
        "past negative" => ("さなかった", 1),
        "te form" => ("して", 1),
        "tai form" => ("したい", 1),
        "volitional" => ("そう", 1),
        "imperative" => ("せ", 1),
        "passive" => ("される", 1),
        "conditional" => ("したら", 1),
        "provisional conditional" => ("せば", 1),
        "causative" => ("させる", 1),
        "potential" => ("せる", 1),
        // polite
        "polite present" => ("します", 1),
        "polite negative" => ("しません", 1),
        "polite past" => ("しました", 1),
        "polite past negative" => ("しませんでした", 1),
        "polite te form" => ("しまして", 1),
        "polite volitional" => ("しましょう", 1),
        "polite imperative" => ("してください", 1),
        "polite passive" => ("されます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5u(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("わない", 1),
        "past" => ("った", 1),
        "past negative" => ("わなかった", 1),
        "te form" => ("って", 1),
        "tai form" => ("いたい", 1),
        "volitional" => ("おう", 1),
        "imperative" => ("え", 1),
        "passive" => ("われる", 1),
        "conditional" => ("ったら", 1),
        "provisional conditional" => ("えば", 1),
        "causative" => ("わせる", 1),
        "potential" => ("える", 1),
        // polite
        "polite present" => ("います", 1),
        "polite negative" => ("いません", 1),
        "polite past" => ("いました", 1),
        "polite past negative" => ("いませんでした", 1),
        "polite te form" => ("いまして", 1),
        "polite volitional" => ("いましょう", 1),
        "polite imperative" => ("ってください", 1),
        "polite passive" => ("われます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5u_s(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("わない", 1),
        "past" => ("うた", 1),
        "past negative" => ("わなかった", 1),
        "te form" => ("うて", 1),
        "tai form" => ("いたい", 1),
        "volitional" => ("おう", 1),
        "imperative" => ("え", 1),
        "passive" => ("われる", 1),
        "conditional" => ("うたら", 1),
        "provisional conditional" => ("えば", 1),
        "causative" => ("わせる", 1),
        "potential" => ("える", 1),
        // polite
        "polite present" => ("います", 1),
        "polite negative" => ("いません", 1),
        "polite past" => ("いました", 1),
        "polite past negative" => ("いませんでした", 1),
        "polite te form" => ("いまして", 1),
        "polite volitional" => ("いましょう", 1),
        "polite imperative" => ("うてください", 1),
        "polite passive" => ("われます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5g(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("がない", 1),
        "past" => ("いだ", 1),
        "past negative" => ("がなかった", 1),
        "te form" => ("いで", 1),
        "tai form" => ("ぎたい", 1),
        "volitional" => ("ごう", 1),
        "imperative" => ("げ", 1),
        "passive" => ("がれる", 1),
        "conditional" => ("いだら", 1),
        "provisional conditional" => ("げば", 1),
        "causative" => ("がせる", 1),
        "potential" => ("げる", 1),
        // polite
        "polite present" => ("ぎます", 1),
        "polite negative" => ("ぎません", 1),
        "polite past" => ("ぎました", 1),
        "polite past negative" => ("ぎませんでした", 1),
        "polite te form" => ("ぎまして", 1),
        "polite volitional" => ("ぎましょう", 1),
        "polite imperative" => ("いでください", 1),
        "polite passive" => ("がれます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn v5t(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("たない", 1),
        "past" => ("った", 1),
        "past negative" => ("たなかった", 1),
        "te form" => ("って", 1),
        "tai form" => ("ちたい", 1),
        "volitional" => ("とう", 1),
        "imperative" => ("て", 1),
        "passive" => ("たれる", 1),
        "conditional" => ("ったら", 1),
        "provisional conditional" => ("てば", 1),
        "causative" => ("たせる", 1),
        "potential" => ("てる", 1),
        // polite
        "polite present" => ("ちます", 1),
        "polite negative" => ("ちません", 1),
        "polite past" => ("ちました", 1),
        "polite past negative" => ("ちませんでした", 1),
        "polite te form" => ("ちまして", 1),
        "polite volitional" => ("ちましょう", 1),
        "polite imperative" => ("ってください", 1),
        "polite passive" => ("たれます", 1),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn vk(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("ない", 1),
        "past" => ("きた", 2),
        "past negative" => ("なかった", 1),
        "te form" => ("きて", 2),
        "tai form" => ("きたい", 2),
        "volitional" => ("よう", 1),
        "imperative" => ("い", 1),
        "passive" => ("られる", 1),
        "conditional" => ("きたら", 2),
        "provisional conditional" => ("くれば", 2),
        "causative" => ("させる", 1),
        "potential" => ("られる", 1),
        // polite
        "polite present" => ("きます", 2),
        "polite negative" => ("きません", 2),
        "polite past" => ("きました", 2),
        "polite past negative" => ("きませんでした", 2),
        "polite te form" => ("きまして", 2),
        "polite volitional" => ("きましょう", 2),
        "polite imperative" => ("きてください", 2),
        "polite passive" => ("きられます", 2),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn vs_i(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("しない", 2),
        "past" => ("した", 2),
        "past negative" => ("しなかった", 2),
        "te form" => ("して", 2),
        "tai form" => ("したい", 2),
        "volitional" => ("しよう", 2),
        "imperative" => ("しろ", 2),
        "passive" => ("される", 2),
        "conditional" => ("したら", 2),
        "provisional conditional" => ("れば", 1),
        "causative" => ("させる", 2),
        "potential" => ("できる", 2),
        // polite
        "polite present" => ("します", 2),
        "polite negative" => ("しません", 2),
        "polite past" => ("しました", 2),
        "polite past negative" => ("しませんでした", 2),
        "polite te form" => ("しまして", 2),
        "polite volitional" => ("しましょう", 2),
        "polite imperative" => ("してください", 2),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn vs_s(verb: &str, conjugation: &str) -> String {
    vs_i(verb, conjugation)
}

fn vz(verb: &str, conjugation: &str) -> String {
    let (ending, suffix_len) = match conjugation {
        "negative" => ("じない", 2),
        "past" => ("じた", 2),
        "past negative" => ("じなかった", 2),
        "te form" => ("じて", 2),
        "tai form" => ("じたい", 2),
        "volitional" => ("じよう", 2),
        "imperative" => ("じろ", 2),
        "passive" => ("じられる", 2),
        "conditional" => ("じたら", 2),
        "provisional conditional" => ("れば", 1),
        "causative" => ("じさせる", 2),
        "potential" => ("じられる", 2),
        // polite
        "polite present" => ("じます", 2),
        "polite negative" => ("じません", 2),
        "polite past" => ("じました", 2),
        "polite past negative" => ("じませんでした", 2),
        "polite te form" => ("じまして", 2),
        "polite volitional" => ("じましょう", 2),
        "polite imperative" => ("じてください", 2),
        "polite passive" => ("じられます", 2),
        _ => panic!("conjugation '{}' unknown", conjugation),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn negative(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "vk" => ("ない", 1),
        "v5b" => ("ばない", 1),
        "v5g" => ("がない", 1),
        "v5k" | "v5k-s" => ("かない", 1),
        "v5m" => ("まない", 1),
        "v5n" => ("なない", 1),
        "v5r" | "v5aru" => ("らない", 1),
        "v5r-i" => ("ない", 2),
        "v5s" => ("さない", 1),
        "v5t" => ("たない", 1),
        "v5u" | "v5u-s" => ("わない", 1),
        "vs-i" => ("しない", 2),
        "vz" => ("じない", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn past(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("た", 1),
        "v5aru" | "v5k-s" | "v5r" | "v5r-i" | "v5t" | "v5u" => ("った", 1),
        "v5b" | "v5m" | "v5n" => ("んだ", 1),
        "v5g" => ("いだ", 1),
        "v5k" => ("いた", 1),
        "v5s" => ("した", 1),
        "v5u-s" => ("うた", 1),
        "vk" => ("きた", 2),
        "vs-i" => ("した", 2),
        "vz" => ("じた", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn past_negative(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "vk" => ("なかった", 1),
        "v5b" => ("ばなかった", 1),
        "v5g" => ("がなかった", 1),
        "v5k" | "v5k-s" => ("かなかった", 1),
        "v5m" => ("まなかった", 1),
        "v5n" => ("ななかった", 1),
        "v5r" | "v5aru" => ("らなかった", 1),
        "v5r-i" => ("なかった", 2),
        "v5s" => ("さなかった", 1),
        "v5t" => ("たなかった", 1),
        "v5u" | "v5u-s" => ("わなかった", 1),
        "vs-i" => ("しなかった", 2),
        "vz" => ("じなかった", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn te_form(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("て", 1),
        "v5aru" | "v5k-s" | "v5r" | "v5r_i" | "v5t" | "v5u" => ("って", 1),
        "v5b" | "v5m" | "v5n" => ("んで", 1),
        "v5g" => ("いで", 1),
        "v5k" => ("いて", 1),
        "v5s" => ("して", 1),
        "v5u-s" => ("うて", 1),
        "vk" => ("きて", 2),
        "vs-i" => ("して", 2),
        "vz" => ("じて", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn tai_form(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("たい", 1),
        "v5aru" | "v5u" | "v5u-s" => ("いたい", 1),
        "v5b" => ("びたい", 1),
        "v5g" => ("ぎたい", 1),
        "v5k" | "v5k-s" => ("きたい", 1),
        "v5m" => ("みたい", 1),
        "v5n" => ("にたい", 1),
        "v5r" | "v5r-i" => ("りたい", 1),
        "v5s" => ("したい", 1),
        "v5t" => ("ちたい", 1),
        "vk" => ("きたい", 2),
        "vs-i" => ("したい", 2),
        "vz" => ("じたい", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn volitional(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "vk" => ("よう", 1),
        "v5aru" | "v5r" | "v5r-i" => ("ろう", 1),
        "v5b" => ("ぼう", 1),
        "v5g" => ("ごう", 1),
        "v5k" | "v5k-s" => ("こう", 1),
        "v5m" => ("もう", 1),
        "v5n" => ("のう", 1),
        "v5s" => ("そう", 1),
        "v5t" => ("とう", 1),
        "v5u" | "v5u-s" => ("おう", 1),
        "vs-i" => ("しよう", 2),
        "vz" => ("じよう", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn imperative(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" => ("ろ", 1),
        "v1-s" => ("", 0),
        "v5aru" | "vk" => ("い", 1),
        "v5b" => ("べ", 1),
        "v5g" => ("げ", 1),
        "v5k" | "v5k-s" => ("け", 1),
        "v5m" => ("め", 1),
        "v5n" => ("ね", 1),
        "v5r" | "v5r-i" => ("れ", 1),
        "v5s" => ("せ", 1),
        "v5t" => ("て", 1),
        "v5u" | "v5u-s" => ("え", 1),
        "vs-i" => ("しろ", 2),
        "vz" => ("じろ", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn passive(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "v5aru" | "v5r" | "v5r-i" | "vk" => ("られる", 1),
        "v5b" => ("ばれる", 1),
        "v5g" => ("がれる", 1),
        "v5k" | "v5k-s" => ("かれる", 1),
        "v5m" => ("まれる", 1),
        "v5n" => ("なれる", 1),
        "v5s" => ("される", 1),
        "v5t" => ("たれる", 1),
        "v5u" | "v5u-s" => ("われる", 1),
        "vs-i" => ("される", 2),
        "vz" => ("じられる", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn conditional(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("たら", 1),
        "v5aru" | "v5k-s" | "v5r" | "v5r-i" | "v5t" | "v5u" => ("ったら", 1),
        "v5b" | "v5m" | "v5n" => ("んだら", 1),
        "v5g" => ("いだら", 1),
        "v5k" => ("いたら", 1),
        "v5s" => ("したら", 1),
        "v5u-s" => ("うたら", 1),
        "vk" => ("きたら", 2),
        "vs-i" => ("したら", 2),
        "vz" => ("じたら", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn provisional_conditional(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "v5aru" | "v5r" | "v5r-i" | "vs-i" | "vz" => ("れば", 1),
        "v5b" => ("べば", 1),
        "v5g" => ("げば", 1),
        "v5k" | "v5k-s" => ("けば", 1),
        "v5m" => ("めば", 1),
        "v5n" => ("ねば", 1),
        "v5s" => ("せば", 1),
        "v5t" => ("てば", 1),
        "v5u" | "v5u-s" => ("えば", 1),
        "vk" => ("くれば", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn causative(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "v5s" | "vk" => ("させる", 1),
        "v5aru" | "v5r" | "v5r-i" => ("らせる", 1),
        "v5b" => ("ばせる", 1),
        "v5g" => ("がせる", 1),
        "v5k" | "v5k-s" => ("かせる", 1),
        "v5m" => ("ませる", 1),
        "v5n" => ("なせる", 1),
        "v5t" => ("たせる", 1),
        "v5u" | "v5u-s" => ("わせる", 1),
        "vs-i" => ("させる", 2),
        "vz" => ("じさせる", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn potential(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "vk" => ("られる", 1),
        "v5aru" => ("り得る", 1),
        "v5b" => ("べる", 1),
        "v5g" => ("げる", 1),
        "v5k" | "v5k-s" => ("ける", 1),
        "v5m" => ("める", 1),
        "v5n" => ("ねる", 1),
        "v5r" => ("れる", 1),
        "v5r-i" => ("りえる", 1),
        "v5s" => ("せる", 1),
        "v5t" => ("てる", 1),
        "v5u" | "v5u-s" => ("える", 1),
        "vs-i" => ("できる", 2),
        "vz" => ("じられる", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn polite_present(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("ます", 1),
        "v5aru" | "v5u" | "v5u-s" => ("います", 1),
        "v5b" => ("びます", 1),
        "v5g" => ("ぎます", 1),
        "v5k" | "v5k-s" => ("きます", 1),
        "v5m" => ("みます", 1),
        "v5n" => ("にます", 1),
        "v5r" | "v5r-i" => ("ります", 1),
        "v5s" => ("します", 1),
        "v5t" => ("ちます", 1),
        "vk" => ("きます", 2),
        "vs-i" => ("します", 2),
        "vz" => ("じます", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn polite_negative(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("ません", 1),
        "v5aru" | "v5u" | "v5u-s" => ("いません", 1),
        "v5b" => ("びません", 1),
        "v5g" => ("ぎません", 1),
        "v5k" | "v5k-s" => ("きません", 1),
        "v5m" => ("みません", 1),
        "v5n" => ("にません", 1),
        "v5r" | "v5r-i" => ("りません", 1),
        "v5s" => ("しません", 1),
        "v5t" => ("ちません", 1),
        "vk" => ("きません", 2),
        "vs-i" => ("しません", 2),
        "vz" => ("じません", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn polite_past(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("ました", 1),
        "v5aru" | "v5u" | "v5u-s" => ("いました", 1),
        "v5b" => ("びました", 1),
        "v5g" => ("ぎました", 1),
        "v5k" | "v5k-s" => ("きました", 1),
        "v5m" => ("みました", 1),
        "v5n" => ("にました", 1),
        "v5r" | "v5r-i" => ("りました", 1),
        "v5s" => ("しました", 1),
        "v5t" => ("ちました", 1),
        "vk" => ("きました", 2),
        "vs-i" => ("しました", 2),
        "vz" => ("じました", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn polite_past_negative(verb: &str, category: &str) -> String {
    format!("{}でした", polite_past(verb, category))
}

fn polite_te_form(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("まして", 1),
        "v5aru" | "v5u" | "v5u-s" => ("いまして", 1),
        "v5b" => ("びまして", 1),
        "v5g" => ("ぎまして", 1),
        "v5k" | "v5k-s" => ("きまして", 1),
        "v5m" => ("みまして", 1),
        "v5n" => ("にまして", 1),
        "v5r" | "v5r-i" => ("りまして", 1),
        "v5s" => ("しまして", 1),
        "v5t" => ("ちまして", 1),
        "vk" => ("きまして", 2),
        "vs-i" => ("しまして", 2),
        "vz" => ("じまして", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn polite_volitional(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("ましょう", 1),
        "v5aru" | "v5u" | "v5u-s" => ("いましょう", 1),
        "v5b" => ("びましょう", 1),
        "v5g" => ("ぎましょう", 1),
        "v5k" | "v5k-s" => ("きましょう", 1),
        "v5m" => ("みましょう", 1),
        "v5n" => ("にましょう", 1),
        "v5r" | "v5r-i" => ("りましょう", 1),
        "v5s" => ("しましょう", 1),
        "v5t" => ("ちましょう", 1),
        "vk" => ("きましょう", 2),
        "vs-i" => ("しましょう", 2),
        "vz" => ("じましょう", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}

fn polite_imperative(verb: &str, category: &str) -> String {
    format!("{}ください", te_form(verb, category))
}

fn polite_passive(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "v5aru" | "v5r" | "v5r-i" => ("られます", 1),
        "v5b" => ("ばれます", 1),
        "v5g" => ("がれます", 1),
        "v5k" | "v5k-s" => ("かれます", 1),
        "v5m" => ("まれます", 1),
        "v5n" => ("なれます", 1),
        "v5s" => ("されます", 1),
        "v5t" => ("たれます", 1),
        "v5u" | "v5u-s" => ("われます", 1),
        "vk" => ("きられます", 2),
        "vs-i" => ("されます", 2),
        "vz" => ("じられます", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    format!("{}{}", &verb[..prefix_len], ending)
}
