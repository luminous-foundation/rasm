use crate::{error::Loc, tokenizer::{Token, Type}};

#[derive(Debug)]
pub struct Function {
    pub loc: Loc,

    pub name: String,
    pub return_type: Vec<Type>,
    pub arg_types: Vec<Vec<Type>>,
    pub arg_names: Vec<String>,

    pub body: Vec<Vec<Token>>,
    pub body_loc: Vec<Vec<Loc>>
}