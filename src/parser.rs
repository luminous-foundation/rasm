use std::{collections::HashMap, net::ToSocketAddrs};

use crate::{expr::{self, Expr}, instruction::Instruction, number::Number, tokenizer::Token};
use lazy_static::lazy_static;
use rainbow_wrapper::{ident, immediate, name, rainbow_wrapper::{functions::Arg, types::Value, wrapper::Wrapper}};

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

pub fn parse(tokens: Vec<Vec<Token>>, wrapper: &mut Wrapper) -> Vec<Expr> {
    let mut res: Vec<Expr> = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        let line = &tokens[i];

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
                                Token::STRING(s) => {
                                    wrapper.push_string(&s);

                                    Value::IDENT(s)
                                }
                                _ => panic!("unexpected token {arg:?}")
                            });
                        }

                        res.push(Expr::INSTR(INSTR_MAP.get(s.as_str()).unwrap().clone(), wrapped));
                    } else {
                        todo!("unhandled ident {:?}", line[0])
                    }
                }
                Token::TYPE(_) => {
                    if line.len() < 2 {
                        panic!("unexpected token {:?}", line[0]);
                    }

                    match &line[1] {
                        Token::IDENT(_) => {
                            let mut end = i;
                            while &tokens[end][0] != &Token::RCURLY {
                                end += 1;
                            }

                            res.push(parse_function(tokens[i..end].to_vec(), wrapper));

                            i = end;
                        }
                        _ => panic!("unexpected token {:?}", line[0])
                    }
                }
                Token::DOT => {}
                _ => {
                    todo!("unhandled token {:?}", line[0])
                }
            }
        }

        if res.len() > 0 {
            println!("{:?}", res[res.len()-1]);
        }

        i += 1;
    }

    wrapper.push(emit(&res));

    return res;
}

pub fn parse_function(tokens: Vec<Vec<Token>>, wrapper: &mut Wrapper) -> Expr {
    let ret_type = match &tokens[0][0] {
        Token::TYPE(t) => {
            let mut new_type = Vec::new();
            for typ in t {
                new_type.push(typ.to_rbtype());
            }

            new_type
        }
        _ => panic!("unexpected token {:?}", tokens[0][0])
    };

    let name = match &tokens[0][1] {
        Token::IDENT(n) => n,
        _ => panic!("unexpected token {:?}", tokens[0][0])
    }.clone();

    let mut args: Vec<Arg> = Vec::new();
    let mut i = 3;
    while tokens[0][i] != Token::RPAREN {
        let typ = match &tokens[0][i] {
            Token::TYPE(t) => {
                let mut new_type = Vec::new();
                for typ in t {
                    new_type.push(typ.to_rbtype());
                }

                new_type
            }
            _ => panic!("unexpected token {:?}", tokens[0][i])
        };

        let name = match &tokens[0][i + 1] {
            Token::IDENT(s) => s,
            _ => panic!("unexpected token {:?}", tokens[0][i + 1])
        }.clone();

        args.push(Arg { name, typ });

        i += 2;
    }

    let body = parse(tokens[1..].to_vec(), wrapper);

    return Expr::FUNCDEF(name, args, ret_type, body);
}

pub fn emit(exprs: &Vec<Expr>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for expr in exprs {
        res.append(&mut expr.to_bytes());
    }

    return res;
}