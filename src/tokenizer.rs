use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    NUMBER(u128),
    TYPE(Type),
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
    ($tokens:expr, $cur_token:expr, $in_num:expr) => {
        if $cur_token.len() > 0 {
            if($in_num) {
                $tokens.push(Token::NUMBER($cur_token.parse::<u128>().unwrap()));
                $in_num = false;
            } else {
                $tokens.push(Token::IDENT($cur_token));
            }
            $cur_token = String::from("");
        }
    };
}

pub fn tokenize(line: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut cur_token: String = String::from("");
    let mut in_str = false;
    let mut in_num = false;
    for c in line.chars() {
        if is_type(&cur_token) {
            tokens.push(Token::TYPE(*TYPE_MAP.get(&cur_token.to_uppercase()[..]).unwrap()));
            cur_token = String::from("");
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
                    push!(tokens, cur_token, in_num);
                    tokens.push(Token::LPAREN);
                }
                ')' => {
                    push!(tokens, cur_token, in_num);
                    tokens.push(Token::RPAREN);
                }
                '{' => {
                    push!(tokens, cur_token, in_num);
                    tokens.push(Token::LCURLY);
                }
                '}' => {
                    push!(tokens, cur_token, in_num);
                    tokens.push(Token::RCURLY);
                }
                '[' => {
                    push!(tokens, cur_token, in_num);
                    tokens.push(Token::LSQUARE);
                }
                ']' => {
                    push!(tokens, cur_token, in_num);
                    tokens.push(Token::RSQUARE);
                }
                '"' => {
                    in_str = true;
                }
                '.' => {
                    push!(tokens, cur_token, in_num);
                    tokens.push(Token::DOT);
                }
                ',' => {
                    push!(tokens, cur_token, in_num);
                    tokens.push(Token::COMMA);
                }
                ' ' | '\r' | '\n' => {
                    push!(tokens, cur_token, in_num);
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
                    tokens.push(Token::STRING(cur_token));
                    cur_token = String::from("");
                }
                _ => cur_token.push(c)
            }
        }
    }

    return tokens;
}