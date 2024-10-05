use std::collections::HashMap;

use crate::{expr::Expr, instruction::Instruction, number::Number, tokenizer::Token};
use lazy_static::lazy_static;
use rainbow_wrapper::{ident, immediate, name, rainbow_wrapper::types::Value};

lazy_static! {
    static ref INSTR_MAP: HashMap<&'static str, Instruction> = {
        let mut m = HashMap::new();
        m.insert("NOP", Instruction::NOP);
        m.insert("PUSH", Instruction::PUSH);
        m.insert("POP", Instruction::POP);
        m.insert("PEEK", Instruction::PEEK);
        m.insert("CALL", Instruction::CALL);
        m.insert("ADD", Instruction::ADD);
        m.insert("SUB", Instruction::SUB);
        m.insert("MUL", Instruction::MUL);
        m.insert("DIV", Instruction::DIV);
        m.insert("JMP", Instruction::JMP);
        m.insert("JNE", Instruction::JNE);
        m.insert("JE", Instruction::JE);
        m.insert("JGE", Instruction::JGE);
        m.insert("JG", Instruction::JG);
        m.insert("JLE", Instruction::JLE);
        m.insert("JL", Instruction::JL);
        m.insert("MOV", Instruction::MOV);
        m.insert("AND", Instruction::AND);
        m.insert("OR", Instruction::OR);
        m.insert("XOR", Instruction::XOR);
        m.insert("NOT", Instruction::NOT);
        m.insert("LSH", Instruction::LSH);
        m.insert("RSH", Instruction::RSH);
        m.insert("VAR", Instruction::VAR);
        m.insert("RET", Instruction::RET);
        m.insert("DEREF", Instruction::DEREF);
        m.insert("REF", Instruction::REF);
        m.insert("INST", Instruction::INST);
        m.insert("MOD", Instruction::MOD);
        m.insert("PMOV", Instruction::PMOV);
        m.insert("ALLOC", Instruction::ALLOC);
        m.insert("FREE", Instruction::FREE);
        m.insert("CALLC", Instruction::CALLC);
        m
    };
}

pub fn parse(tokens: Vec<Vec<Token>>) -> Vec<Expr> {
    let mut res: Vec<Expr> = Vec::new();

    for line in tokens {
        println!("{line:?}");

        if line.len() > 0 {
            match &line[0] {
                Token::IDENT(s) => {
                    if INSTR_MAP.contains_key(s.as_str()) {
                        let args = line[1..].to_vec();
                        let mut wrapped: Vec<Value> = Vec::new();

                        for arg in args {
                            wrapped.push(match arg {
                                Token::IDENT(s) => name!(s),
                                Token::VAR(s) => ident!(s),
                                Token::NUMBER(n) => {
                                    match n {
                                        Number::SIGNED(n) => immediate!(SIGNED(n)),
                                        Number::UNSIGNED(n) => immediate!(UNSIGNED(n)),
                                        Number::DECIMAL(n) => immediate!(DECIMAL(n)),
                                    }
                                }
                                Token::TYPE(t) => {
                                    let mut new_type = Vec::new();
                                    for typ in t {
                                        new_type.push(typ.to_rbtype());
                                    }

                                    Value::TYPE(new_type)
                                }
                                _ => panic!("unexpected token {arg:?}")
                            });
                        }

                        res.push(Expr::INSTR(INSTR_MAP.get(s.as_str()).unwrap().clone(), wrapped));
                    } else {
                        todo!("unhandled ident {:?}", line[0])
                    }
                }
                _ => {
                    todo!("unhandled token {:?}", line[0])
                }
            }
        }

        println!("{:?}", res[res.len()-1]);
    }

    return res;
}

pub fn emit(exprs: Vec<Expr>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for expr in exprs {
        res.append(&mut expr.to_bytes());
    }

    return res;
}