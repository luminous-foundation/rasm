use crate::tokenizer::Type;

#[derive(Debug)]
pub struct Struct { // lol
    pub name: String,
    pub var_names: Vec<String>,
    pub var_types: Vec<Vec<Type>>
}
