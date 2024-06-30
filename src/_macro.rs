use crate::{error::Loc, tokenizer::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Macro {
    pub loc: Loc,

    pub name: String,
    pub args: Vec<String>,
    pub content: Vec<Vec<Token>>
}