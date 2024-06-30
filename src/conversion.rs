use std::collections::HashMap;

use crate::tokenizer::Type;

use lazy_static::lazy_static;

lazy_static! {
    static ref TYPE_MAP: HashMap<Type, u8> = {
        let mut m = HashMap::new();
        m.insert(Type::VOID,    0x00);
        m.insert(Type::I8,      0x01);
        m.insert(Type::I16,     0x02);
        m.insert(Type::I32,     0x03);
        m.insert(Type::I64,     0x04);
        m.insert(Type::U8,      0x05);
        m.insert(Type::U16,     0x06);
        m.insert(Type::U32,     0x07);
        m.insert(Type::U64,     0x08);
        m.insert(Type::F16,     0x09);
        m.insert(Type::F32,     0x0A);
        m.insert(Type::F64,     0x0B);
        m.insert(Type::POINTER, 0x0C);
        m.insert(Type::TYPE,    0x0D);
        m.insert(Type::STRUCT,  0x0E);
        m.insert(Type::NAME,    0x0F);
        m
    };
}

pub fn convert_string(text: &String) -> Vec<u8> {
    return text.clone().into_bytes();
}

pub fn convert_bytecode_string(text: &String) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    res.push(text.len() as u8);
    res.append(&mut text.clone().into_bytes());

    return res;
}

// TODO: implement
// TODO: squeeze number into smaller value
pub fn convert_number(n: u128) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    let bytes = n.to_ne_bytes();
    for i in 0..get_bytes_needed(n) {
        res.push(bytes[i as usize]);
    }

    return res;
}

pub fn convert_type(_type: &Vec<Type>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for t in _type {
        res.push(*TYPE_MAP.get(t).expect("unreachable"));
    }

    return res;
}

pub fn get_bytes_needed(n: u128) -> u8 {
    return ((n as f32 + 1.).log2() / 8.0).ceil() as u8;
}