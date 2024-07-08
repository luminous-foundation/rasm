use std::{collections::HashMap, str::FromStr};
use lazy_static::lazy_static;

use crate::{error::Loc, number::Number};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    VOID,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F16,
    F32,
    F64,
    POINTER,
    TYPE,
    STRUCT,
    NAME,
}

lazy_static! {
    static ref TYPE_MAP: HashMap<&'static str, Type> = {
        let mut m = HashMap::new();
        m.insert("VOID", Type::VOID);
        m.insert("I8", Type::I8);
        m.insert("I16", Type::I16);
        m.insert("I32", Type::I32);
        m.insert("I64", Type::I64);
        m.insert("U8", Type::U8);
        m.insert("CHAR", Type::U8);
        m.insert("U16", Type::U16);
        m.insert("U32", Type::U32);
        m.insert("U64", Type::U64);
        m.insert("F16", Type::F16);
        m.insert("F32", Type::F32);
        m.insert("F64", Type::F64);
        m.insert("*", Type::POINTER);
        m.insert("TYPE", Type::TYPE);
        m.insert("STRUCT", Type::STRUCT);
        m.insert("NAME", Type::NAME);
        m.insert("FUNCPTR", Type::NAME);
        m
    };
}

fn is_type(input: &str) -> bool {
    TYPE_MAP.contains_key(&input.to_uppercase()[..])
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    NUMBER(Number),
    TYPE(Vec<Type>),
    LPAREN,
    RPAREN,
    LCURLY,
    RCURLY,
    LSQUARE,
    RSQUARE,
    STRING(String),
    DOT,
    COMMA,
    IDENT(String),
}

macro_rules! push {
    ($tokens:expr, $locs:expr, $cur_token:expr, $in_num:expr, $loc:expr) => {
        if $cur_token.len() > 0 {
            $locs.push($loc.clone());
            $loc.col = $loc.col + $cur_token.len();

            if($in_num) {
                $tokens.push(Token::NUMBER(Number::from_str(&$cur_token).unwrap()));
                $in_num = false;
            } else {
                $tokens.push(Token::IDENT($cur_token));
            }

            $cur_token = String::from("");
        }
    };
}

// TODO: character literals
pub fn tokenize(line: String, loc: &mut Loc) -> (Vec<Token>, Vec<Loc>) {
    let mut tokens: Vec<Token> = Vec::new();

    let mut locs: Vec<Loc> = Vec::new();

    let mut cur_token: String = String::from("");
    let mut in_str = false;
    let mut in_num = false;

    let mut in_type = false;
    let mut temp_type: Vec<Type> = Vec::new();

    for c in line.chars() {
        if is_type(&cur_token) {
            locs.push(loc.clone());
            loc.col = loc.col + cur_token.len();

            temp_type.push(*TYPE_MAP.get(&cur_token.to_uppercase()[..]).unwrap());
            cur_token = String::from("");
            in_type = true;
        } else if in_type {
            in_type = false;
            temp_type.reverse();
            tokens.push(Token::TYPE(temp_type.clone()));
            temp_type.clear();
        }

        if !in_str {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { // TODO: float support
                    if cur_token.len() == 0 {
                        in_num = true;
                    }
                    cur_token.push(c);
                }
                '(' => {
                    push!(tokens, locs, cur_token, in_num, loc);
                    tokens.push(Token::LPAREN);

                    locs.push(loc.clone());
                    loc.col = loc.col + 1;
                }
                ')' => {
                    push!(tokens, locs, cur_token, in_num, loc);
                    tokens.push(Token::RPAREN);

                    locs.push(loc.clone());
                    loc.col = loc.col + 1;
                }
                '{' => {
                    push!(tokens, locs, cur_token, in_num, loc);
                    tokens.push(Token::LCURLY);

                    locs.push(loc.clone());
                    loc.col = loc.col + 1;
                }
                '}' => {
                    push!(tokens, locs, cur_token, in_num, loc);
                    tokens.push(Token::RCURLY);

                    locs.push(loc.clone());
                    loc.col = loc.col + 1;
                }
                '[' => {
                    push!(tokens, locs, cur_token, in_num, loc);
                    tokens.push(Token::LSQUARE);

                    locs.push(loc.clone());
                    loc.col = loc.col + 1;
                }
                ']' => {
                    push!(tokens, locs, cur_token, in_num, loc);
                    tokens.push(Token::RSQUARE);

                    locs.push(loc.clone());
                    loc.col = loc.col + 1;
                }
                '"' => {
                    in_str = true;
                }
                '.' => {
                    if in_num {
                        cur_token.push(c);
                    } else {
                        push!(tokens, locs, cur_token, in_num, loc);
                        tokens.push(Token::DOT);

                        locs.push(loc.clone());
                        loc.col = loc.col + 1;
                    }
                }
                ',' => {
                    push!(tokens, locs, cur_token, in_num, loc);
                    tokens.push(Token::COMMA);

                    locs.push(loc.clone());
                    loc.col = loc.col + 1;
                }
                ' ' | '\r' | '\n' => {
                    push!(tokens, locs, cur_token, in_num, loc);
                }
                ';' => {
                    break;
                }
                _ => cur_token.push(c)
            }
        } else {
            // yes im using match for this
            match c {
                '"' => {
                    locs.push(loc.clone());
                    loc.col = loc.col + cur_token.len();

                    tokens.push(Token::STRING(cur_token));

                    cur_token = String::from("");
                }
                _ => cur_token.push(c)
            }
        }
    }

    return (tokens, locs);
}