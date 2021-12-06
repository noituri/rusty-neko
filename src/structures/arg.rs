use crate::enums::raw_arg_types::raw_arg_types;

pub struct Arg {
    pub name: String,
    pub description: String, 
    pub required: bool, 
    pub example: String,
    pub regexes: Vec<regex::Regex>,
    pub expect: raw_arg_types,
    pub min_len: usize,
    pub max_len: usize
}