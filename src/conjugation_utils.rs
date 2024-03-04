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
    if !tags.iter().any(|s| s.starts_with('v')) || verb.is_empty() {
        Vec::new()
    } else if let Some(category) = tags.into_iter().find(|&tag| VERB_TAGS.contains(tag)) {
        vec![
            // plain forms
            causative(verb, category),
            causative_passive(verb, category),
            causative_short(verb, category),
            conditional(verb, category),
            desire(verb, category),
            imperative(verb, category),
            passive(verb, category),
            potential(verb, category),
            provisional_conditional(verb, category),
            volitional(verb, category),
            // negative forms
            negative(verb, category),
            causative_negative(verb, category),
            causative_passive_negative(verb, category),
            conditional_negative(verb, category),
            desire_negative(verb, category),
            imperative_negative(verb, category),
            passive_negative(verb, category),
            potential_negative(verb, category),
            // past forms
            past(verb, category),
            causative_past(verb, category),
            causative_passive_past(verb, category),
            desire_past(verb, category),
            passive_past(verb, category),
            potential_past(verb, category),
            // past negative
            past_negative(verb, category),
            causative_past_negative(verb, category),
            causative_passive_past_negative(verb, category),
            desire_past_negative(verb, category),
            passive_past_negative(verb, category),
            potential_past_negative(verb, category),
            // te form
            te_form(verb, category),
            causative_te(verb, category),
            causative_passive_te(verb, category),
            passive_te(verb, category),
            potential_te(verb, category),
            // polite
            polite(verb, category),
            causative_polite(verb, category),
            causative_passive_polite(verb, category),
            passive_polite(verb, category),
            potential_polite(verb, category),
            volitional_polite(verb, category),
            // polite negative
            polite_negative(verb, category),
            causative_polite_negative(verb, category),
            causative_passive_polite_negative(verb, category),
            passive_polite_negative(verb, category),
            potential_polite_negative(verb, category),
            // polite past
            polite_past(verb, category),
            causative_polite_past(verb, category),
            causative_passive_polite_past(verb, category),
            passive_polite_past(verb, category),
            potential_polite_past(verb, category),
            // polite te-form
            polite_te_form(verb, category),
        ]
    } else {
        Vec::new()
    }
}

fn masu_stem(verb: &str, category: &str) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("", 1),
        "v5aru" | "v5u" | "v5u-s" => ("い", 1),
        "v5b" => ("び", 1),
        "v5g" => ("ぎ", 1),
        "v5k" | "v5k-s" => ("き", 1),
        "v5m" => ("み", 1),
        "v5n" => ("に", 1),
        "v5r" | "v5r-i" => ("り", 1),
        "v5s" => ("し", 1),
        "v5t" => ("ち", 1),
        "vk" => ("き", 2),
        "vs-i" | "vs-s" => ("し", 2),
        "vz" => ("じ", 2),
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

fn causative_passive(verb: &str, category: &str) -> String {
    let causative = causative(verb, category);
    [&causative[..causative.len() - 3], "られる"].concat()
}

fn causative_short(verb: &str, category: &str) -> String {
    let causative = causative(verb, category);
    [&causative[..causative.len() - 6], "す"].concat()
}

fn conditional(verb: &str, category: &str) -> String {
    past(verb, category) + "ら"
}

fn desire(verb: &str, category: &str) -> String {
    let stem = masu_stem(verb, category);
    masu_stem(verb, category) + "たい"
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
        "v1" | "v1-s" | "v5aru" | "v5r" | "v5r-i" => ("られる", 1),
        "v5b" => ("ばれる", 1),
        "v5g" => ("がれる", 1),
        "v5k" | "v5k-s" => ("かれる", 1),
        "v5m" => ("まれる", 1),
        "v5n" => ("なれる", 1),
        "v5s" => ("される", 1),
        "v5t" => ("たれる", 1),
        "v5u" | "v5u-s" => ("われる", 1),
        "vk" => ("きれる", 2),
        "vs-i" | "vs-s" => ("される", 2),
        "vz" => ("じられる", 2),
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

fn causative_negative(verb: &str, category: &str) -> String {
    negative(&causative(verb, category), "v1")
}

fn causative_passive_negative(verb: &str, category: &str) -> String {
    negative(&causative(verb, category), "v1")
}

fn conditional_negative(verb: &str, category: &str) -> String {
    past_negative(verb, category) + "ら"
}

fn desire_negative(verb: &str, category: &str) -> String {
    let tai = desire(verb, category);
    [&tai[..tai.len() - 3], "くない"].concat()
}

fn imperative_negative(verb: &str, category: &str) -> String {
    imperative(verb, category) + "な"
}

fn passive_negative(verb: &str, category: &str) -> String {
    negative(&passive(verb, category), "v1")
}

fn potential_negative(verb: &str, category: &str) -> String {
    negative(&potential(verb, category), "v1")
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

fn causative_past(verb: &str, category: &str) -> String {
    past(&causative(verb, category), "v1")
}

fn causative_passive_past(verb: &str, category: &str) -> String {
    past(&causative_passive(verb, category), "v1")
}

fn desire_past(verb: &str, category: &str) -> String {
    let tai = desire(verb, category);
    [&tai[..tai.len() - 3], "かった"].concat()
}

fn passive_past(verb: &str, category: &str) -> String {
    past(&passive(verb, category), "v1")
}

fn potential_past(verb: &str, category: &str) -> String {
    past(&potential(verb, category), "v1")
}

fn past_negative(verb: &str, category: &str) -> String {
    let negative = negative(verb, category);
    [&negative[..negative.len() - 3], "かった"].concat()
}

fn causative_past_negative(verb: &str, category: &str) -> String {
    past_negative(&causative(verb, category), "v1")
}

fn causative_passive_past_negative(verb: &str, category: &str) -> String {
    past_negative(&&causative_passive(verb, category), "v1")
}

fn desire_past_negative(verb: &str, category: &str) -> String {
    let tai = desire(verb, category);
    [&tai[..tai.len() - 3], "くなかった"].concat()
}

fn passive_past_negative(verb: &str, category: &str) -> String {
    past_negative(&passive(verb, category), "v1")
}

fn potential_past_negative(verb: &str, category: &str) -> String {
    past_negative(&potential(verb, category), "v1")
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

fn causative_te(verb: &str, category: &str) -> String {
    te_form(&causative(verb, category), "v1")
}

fn causative_passive_te(verb: &str, category: &str) -> String {
    te_form(&causative_passive(verb, category), "v1")
}

fn passive_te(verb: &str, category: &str) -> String {
    te_form(&passive(verb, category), "v1")
}

fn potential_te(verb: &str, category: &str) -> String {
    te_form(&potential(verb, category), "v1")
}

fn polite(verb: &str, category: &str) -> String {
    masu_stem(verb, category) + "ます"
}

fn causative_polite(verb: &str, category: &str) -> String {
    let causative = causative(verb, category);
    [&causative[..causative.len() - 3], "ます"].concat()
}

fn causative_passive_polite(verb: &str, category: &str) -> String {
    let causative_passive = causative_passive(verb, category);
    [&causative_passive[..causative_passive.len() - 3], "ます"].concat()
}

fn passive_polite(verb: &str, category: &str) -> String {
    let passive = passive(verb, category);
    [&passive[..passive.len() - 3], "ます"].concat()
}

fn potential_polite(verb: &str, category: &str) -> String {
    let potential = potential(verb, category);
    [&potential[..potential.len() - 3], "ます"].concat()
}

fn volitional_polite(verb: &str, category: &str) -> String {
    masu_stem(verb, category) + "ましょう"
}

fn polite_negative(verb: &str, category: &str) -> String {
    masu_stem(verb, category) + "ません"
}

fn causative_polite_negative(verb: &str, category: &str) -> String {
    let causative = causative(verb, category);
    [&causative[..causative.len() - 3], "ません"].concat()
}

fn causative_passive_polite_negative(verb: &str, category: &str) -> String {
    let causative_passive = causative_passive(verb, category);
    [&causative_passive[..causative_passive.len() - 3], "ません"].concat()
}

fn passive_polite_negative(verb: &str, category: &str) -> String {
    let passive = passive(verb, category);
    [&passive[..passive.len() - 3], "ません"].concat()
}

fn potential_polite_negative(verb: &str, category: &str) -> String {
    let potential = potential(verb, category);
    [&potential[..potential.len() - 3], "ません"].concat()
}

fn polite_past(verb: &str, category: &str) -> String {
    masu_stem(verb, category) + "ました"
}

fn causative_polite_past(verb: &str, category: &str) -> String {
    let causative = causative(verb, category);
    [&causative[..causative.len() - 3], "ました"].concat()
}

fn causative_passive_polite_past(verb: &str, category: &str) -> String {
    let causative_passive = causative_passive(verb, category);
    [&causative_passive[..causative_passive.len() - 3], "ました"].concat()
}

fn passive_polite_past(verb: &str, category: &str) -> String {
    let passive = passive(verb, category);
    [&passive[..passive.len() - 3], "ました"].concat()
}

fn potential_polite_past(verb: &str, category: &str) -> String {
    let potential = potential(verb, category);
    [&potential[..potential.len() - 3], "ました"].concat()
}

fn polite_te_form(verb: &str, category: &str) -> String {
    masu_stem(verb, category) + "まして"
}
