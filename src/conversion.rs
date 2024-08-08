use std::collections::HashMap;

use crate::{number::Number, tokenizer::Type};

use half::f16;
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
pub fn convert_number(n: Number) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    let _type = get_type(&n);

    let mut bytes: Vec<u8>;

    match _type {
        0x01 => bytes = (Into::<i64>::into(n.clone()) as i8).to_be_bytes().to_vec(),
        0x02 => bytes = (Into::<i64>::into(n.clone()) as i16).to_be_bytes().to_vec(),
        0x03 => bytes = (Into::<i64>::into(n.clone()) as i32).to_be_bytes().to_vec(),
        0x04 => bytes = (Into::<i64>::into(n.clone()) as i64).to_be_bytes().to_vec(),
        0x05 => bytes = (Into::<u64>::into(n.clone()) as u8).to_be_bytes().to_vec(),
        0x06 => bytes = (Into::<u64>::into(n.clone()) as u16).to_be_bytes().to_vec(),
        0x07 => bytes = (Into::<u64>::into(n.clone()) as u32).to_be_bytes().to_vec(),
        0x08 => bytes = (Into::<u64>::into(n.clone()) as u64).to_be_bytes().to_vec(),
        0x09 => bytes = (f16::from_f64(Into::<f64>::into(n.clone()))).to_be_bytes().to_vec(),
        0x0A => bytes = (Into::<f64>::into(n.clone()) as f32).to_be_bytes().to_vec(),
        0x0B => bytes = (Into::<f64>::into(n.clone()) as f64).to_be_bytes().to_vec(),
        _ => panic!("unreachable")
    }

    res.push(_type);

    res.append(&mut bytes);

    return res;
}

pub fn convert_type(_type: &Vec<Type>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for t in _type {
        res.push(*TYPE_MAP.get(t).expect("unreachable"));
    }

    return res;
}

// pub fn get_bytes_needed(n: Number) -> u8 {
//     return ((Into::<f64>::into(n) + 1.).log2() / 8.0).ceil() as u8;
// }

// TODO: squeeze into smallest type
pub fn get_type(n: &Number) -> u8 {
    match n {
        Number::SIGNED(_) => 0x04,
        Number::UNSIGNED(_) => 0x08,
        Number::FLOAT(_) => 0x0B,
    }
}