use std::collections::HashMap;

pub trait Query {
    fn queries(&self) -> Vec<&str>;
}

pub fn to_queriable_dict<'a, D: Query>(dicts: &'a [D]) -> HashMap<&'a str, Vec<&'a D>> {
    let mut map: HashMap<&'a str, Vec<&'a D>> = HashMap::new();

    for dict in dicts {
        for key in dict.queries() {
            map.entry(key).or_default().push(dict);
        }
    }
    map
}
