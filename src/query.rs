pub trait Query {
    fn possible_queries(&self) -> Vec<&String>;
}
