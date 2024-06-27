use crate::tokenizer::Token;

#[derive(Debug)]
pub struct Macro {
    pub name: String,
    pub content: Vec<Vec<Token>>
}