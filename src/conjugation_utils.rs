use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref VERB_TAGS: HashSet<String> = vec![
        String::from("v1"),
        String::from("v1-s"),
        String::from("vk"),
        String::from("v5b"),
        String::from("v5g"),
        String::from("v5k"),
        String::from("v5k-s"),
        String::from("v5m"),
        String::from("v5n"),
        String::from("v5r"),
        String::from("v5aru"),
        String::from("v5r-i"),
        String::from("v5s"),
        String::from("v5t"),
        String::from("v5u"),
        String::from("v5u-s"),
        String::from("vs-i"),
        String::from("vs-s"),
        String::from("vz"),
    ]
    .into_iter()
    .collect();
}
pub fn generate_conjugations(verb: &str, tags: Vec<&String>) -> Vec<String> {
    if !tags.iter().any(|s| s.starts_with('v')) {
        Vec::new()
    } else if let Some(category) = tags.into_iter().find(|&tag| VERB_TAGS.contains(tag)) {
        vec![
            negative(verb, category),
            past(verb, category),
            past_negative(verb, category),
            te_form(verb, category),
            tai_form(verb, category),
            volitional(verb, category),
            imperative(verb, category),
            passive(verb, category),
            conditional(verb, category),
            provisional_conditional(verb, category),
            causative(verb, category),
            potential(verb, category),
            polite_present(verb, category),
            polite_negative(verb, category),
            polite_past(verb, category),
            polite_past_negative(verb, category),
            polite_te_form(verb, category),
            polite_volitional(verb, category),
            polite_imperative(verb, category),
            polite_passive(verb, category),
        ]
    } else {
        Vec::new()
    }
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
        "vs-i" | "vs-s" => ("しない", 2),
        "vz" => ("じない", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
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
        "vs-i" | "vs-s" => ("した", 2),
        "vz" => ("じた", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
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
        "vs-i" | "vs-s" => ("しなかった", 2),
        "vz" => ("じなかった", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn te_form(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("て", 1),
        "v5aru" | "v5k-s" | "v5r" | "v5r-i" | "v5t" | "v5u" => ("って", 1),
        "v5b" | "v5m" | "v5n" => ("んで", 1),
        "v5g" => ("いで", 1),
        "v5k" => ("いて", 1),
        "v5s" => ("して", 1),
        "v5u-s" => ("うて", 1),
        "vk" => ("きて", 2),
        "vs-i" | "vs-s" => ("して", 2),
        "vz" => ("じて", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
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
        "vs-i" | "vs-s" => ("したい", 2),
        "vz" => ("じたい", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
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
        "vs-i" | "vs-s" => ("しよう", 2),
        "vz" => ("じよう", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
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
        "vs-i" | "vs-s" => ("しろ", 2),
        "vz" => ("じろ", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
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
        "vs-i" | "vs-s" => ("される", 2),
        "vz" => ("じられる", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
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
        "vs-i" | "vs-s" => ("したら", 2),
        "vz" => ("じたら", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn provisional_conditional(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "v5aru" | "v5r" | "v5r-i" | "vs-i" | "vs-s" | "vz" => ("れば", 1),
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
    [&verb[..prefix_len], ending].concat()
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
        "vs-i" | "vs-s" => ("させる", 2),
        "vz" => ("じさせる", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
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
        "vs-i" | "vs-s" => ("できる", 2),
        "vz" => ("じられる", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn polite_base_form(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("ま", 1),
        "v5aru" | "v5u" | "v5u-s" => ("いま", 1),
        "v5b" => ("びま", 1),
        "v5g" => ("ぎま", 1),
        "v5k" | "v5k-s" => ("きま", 1),
        "v5m" => ("みま", 1),
        "v5n" => ("にま", 1),
        "v5r" | "v5r-i" => ("りま", 1),
        "v5s" => ("しま", 1),
        "v5t" => ("ちま", 1),
        "vk" => ("きま", 2),
        "vs-i" | "vs-s" => ("しま", 2),
        "vz" => ("じま", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn polite_present(verb: &str, category: &str) -> String {
    polite_base_form(verb, category) + "す"
}

fn polite_negative(verb: &str, category: &str) -> String {
    polite_base_form(verb, category) + "せん"
}

fn polite_past(verb: &str, category: &str) -> String {
    polite_base_form(verb, category) + "した"
}

fn polite_past_negative(verb: &str, category: &str) -> String {
    polite_past(verb, category) + "でした"
}

fn polite_te_form(verb: &str, category: &str) -> String {
    polite_base_form(verb, category) + "して"
}

fn polite_volitional(verb: &str, category: &str) -> String {
    polite_base_form(verb, category) + "しょう"
}

fn polite_imperative(verb: &str, category: &str) -> String {
    polite_base_form(verb, category) + "してください"
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
        "vs-i" | "vs-s" => ("されます", 2),
        "vz" => ("じられます", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}
