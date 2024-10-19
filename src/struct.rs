use crate::tokenizer::Type;

#[derive(Debug)]
pub struct Struct {
    pub name: String,

    pub types: Vec<Vec<Type>>,
    pub names: Vec<String>,
}