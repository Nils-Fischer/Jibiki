fn polite_imperative(verb: &str, category: &str) -> String {
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
