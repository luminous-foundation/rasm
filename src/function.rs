use crate::tokenizer::{Token, Type};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub arg_types: Vec<Type>,
    pub arg_names: Vec<String>,
    pub body: Vec<Vec<Token>>
}