use crate::enums::raw_arg_types::RawArgTypes;

pub struct Arg {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub example: String,
    pub regexes: Vec<regex::Regex>,
    pub expect: RawArgTypes,
    pub min_len: usize,
    pub max_len: usize,
}
