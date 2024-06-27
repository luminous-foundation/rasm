use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub args: Vec<String>,
    pub content: Vec<Vec<Token>>
}