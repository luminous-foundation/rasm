use crate::tokenizer::Type;

#[derive(Debug)]
pub struct Data {
    pub name: String,
    pub _type: Vec<Type>,
    pub data: Vec<u8>
}