use std::{collections::{HashMap, HashSet}, fs, path::Path};

use crate::{assemble, expr::Expr, instruction::Instruction, number::Number, tokenizer::{self, Token}};
use lazy_static::lazy_static;
use rainbow_wrapper::{ident, immediate, name, r#extern::Extern, functions::Arg, types::{Type, Value}, wrapper::Wrapper};

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
        m.insert("CMP", Instruction::CMP);
        m
    };
}

pub fn parse(mut tokens: Vec<Vec<Token>>, wrapper: &mut Wrapper, link_paths: &mut HashSet<String>) -> Vec<Expr> {
    let mut i = 0;

    // pre-processing
    let mut labels: HashMap<String, usize> = HashMap::new();

    // labels
    parse_labels(&tokens, &mut labels, &mut 0);

    while i < tokens.len() {
        let mut j = 0;
        while j < tokens[i].len() {
            if tokens[i][j] == Token::COLON {
                match tokens[i][j + 1].clone() {
                    Token::IDENT(s) => {
                        if tokens[i].len() > 2 {
                            tokens[i].remove(j);
                            tokens[i].remove(j);
                            
                            tokens[i].insert(j, Token::NUMBER(Number::UNSIGNED(*labels.get(&s).expect(format!("unknown label {s}").as_str()) as u64)));
                        } else {
                            tokens.remove(i);
                        }
                    }
                    _ => panic!("unexpected token {:?}", tokens[i][j + 1])
                }
            }

            j += 1;

            if i >= tokens.len() {
                break;
            }
        }

        i += 1;
    }

    // processing
    let mut res: Vec<Expr> = Vec::new();
    i = 0;
    while i < tokens.len() {
        let line = &tokens[i];

        // println!("{line:?}");

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
                                    Value::TYPE(to_rb_type(t))
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
                            let mut body = vec![line.clone()];

                            body.append(&mut parse_block(&tokens, &mut i));

                            res.push(parse_function(body, wrapper, link_paths));
                        }
                        _ => panic!("unexpected token {:?}", line[1])
                    }
                }
                Token::DOT => {
                    if line.len() < 2 {
                        panic!("unexpected token {:?}", line[0]);
                    }

                    match &line[1] {
                        Token::IDENT(s) => {
                            // println!("{s}");

                            match s.to_lowercase().as_str() {
                                "include" => {
                                    match &line[2] {
                                        Token::IDENT(s) => {
                                            wrapper.push_import(&(s.clone() + ".rbb"));
                                        }
                                        Token::STRING(s) => {
                                            if s.ends_with(".rasm") {
                                                let mut import_path = String::new();
                                                for path in link_paths.clone() {
                                                    let paths = match fs::read_dir(path) {
                                                        Ok(p) => p,
                                                        Err(e) => panic!("{}", e.to_string()),
                                                    };

                                                    for path in paths {
                                                        let dir_entry = path.unwrap();
                                                        let path = &dir_entry.path();
                                                        let path_str = path.as_os_str().to_str().unwrap();
                                                        if path_str.ends_with(s) {
                                                            if import_path == "" {
                                                                import_path = path_str.to_owned();
                                                            } else {
                                                                panic!("ambiguous import {s}");
                                                            }
                                                        }
                                                    }
                                                }

                                                if Path::exists(Path::new(&import_path)) {
                                                    assemble(import_path, link_paths);
                                                }

                                                wrapper.push_import(&(s.split(".").next().unwrap().to_string() + ".rbb"));
                                            } else if s.ends_with(".rbb") {
                                                wrapper.push_import(s);
                                            } else {
                                                wrapper.push_import(&(s.clone() + ".rbb"));
                                            }
                                        }
                                        _ => panic!("unexpected token {:?}", line[2])
                                    }
                                }
                                "extern" => {
                                    let ret_type = match &line[2] {
                                        Token::TYPE(t) => {
                                            to_rb_type(t.clone())
                                        }
                                        _ => panic!("unexpected token {:?}", line[2])
                                    };

                                    let name = match &line[3] {
                                        Token::IDENT(s) => s,
                                        _ => panic!("unexpected token {:?}", line[3])
                                    }.clone();

                                    let mut arg_types: Vec<Vec<Type>> = Vec::new();
                                    let mut index = 5;
                                    while line[index] != Token::RPAREN {
                                        match &line[index] {
                                            Token::TYPE(t) => arg_types.push(to_rb_type(t.clone())),
                                            _ => panic!("unexpected token {:?}", line[index])
                                        }
                                        index += 1;
                                    }

                                    let file = match &line[index+2] {
                                        Token::STRING(s) => s,
                                        _ => panic!("unexpected token {:?}", line[index + 2])
                                    }.clone();

                                    wrapper.push_extern(Extern { ret_type, name, arg_types, file });
                                }
                                "if" | "elseif" => {
                                    let left = match &line[2] {
                                        Token::IDENT(s) => s,
                                        _ => panic!("unexpected token {:?}", line[2])
                                    }.clone();
                                    
                                    let cond = match &line[3] {
                                        Token::IDENT(s) => s,
                                        _ => panic!("unexpected token {:?}", line[3])
                                    }.clone();

                                    let right = match &line[4] {
                                        Token::IDENT(s) => s,
                                        _ => panic!("unexpected token {:?}", line[4])
                                    }.clone();

                                    let end = get_block_body(&tokens, i);

                                    let body = parse(tokens[i+1..end].to_vec(), wrapper, link_paths);
                                    i = end - 1;

                                    match s.to_lowercase().as_str() {
                                        "if" => res.push(Expr::IF_BLOCK(left, cond, right, body)),
                                        "elseif" => res.push(Expr::ELSEIF_BLOCK(left, cond, right, body)),
                                        _ => unreachable!()
                                    }
                                }
                                "else" => {
                                    let end = get_block_body(&tokens, i);

                                    let body = parse(tokens[i+1..end].to_vec(), wrapper, link_paths);

                                    i = end - 1;

                                    res.push(Expr::ELSE_BLOCK(body));
                                }
                                "end" => {
                                    res.push(Expr::END_BLOCK);
                                }
                                _ => panic!("unexpected token {:?}", line[1])
                            }
                        }
                        _ => panic!("unexpected token {:?}", line[1])
                    }
                }
                Token::LCURLY => {
                    let body = parse_block(&tokens, &mut i);

                    res.push(Expr::SCOPE(parse(body, wrapper, link_paths)));
                }
                _ => {
                    todo!("unhandled token {:?}", line[0])
                }
            }
        }

        // if res.len() > 0 {
        //     println!("{:?}", res[res.len()-1]);
        // }

        i += 1;
    }

    // println!("{res:#?}");

    return res;
}

fn parse_labels(tokens: &Vec<Vec<Token>>, labels: &mut HashMap<String, usize>, i: &mut usize) {
    let mut instr = 0;

    while *i < tokens.len() {
        let line = &tokens[*i];

        if line.len() > 0 {
            match &line[0] {
                Token::IDENT(s) => {
                    if INSTR_MAP.contains_key(s.as_str()) {
                        instr += 1;
                    }
                }
                Token::COLON => {
                    match &line[1] {
                        Token::IDENT(s) => {
                            if labels.contains_key(s) {
                                panic!("redefined label {s}");
                            } else {
                                labels.insert(s.to_string(), instr);
                            }
                        }
                        _ => panic!("unexpected token {:?}", line[1])
                    }
                }
                Token::LCURLY => {
                    *i += 1;
                    instr += 1;
                    parse_labels(tokens, labels, i);
                }
                Token::RCURLY => {
                    *i += 1;
                    break;
                }
                _ => {}
            }
        }

        *i += 1;
    }
}

fn parse_block(tokens: &Vec<Vec<Token>>, i: &mut usize) -> Vec<Vec<Token>> {
    let mut res: Vec<Vec<Token>> = Vec::new();

    *i += 1;

    while *i < tokens.len() {
        if tokens[*i].len() > 0 {
            match tokens[*i][0] {
                Token::LCURLY => {
                    res.push(vec![Token::LCURLY]);
                    res.append(&mut parse_block(tokens, i));
                    res.push(vec![Token::RCURLY]);
                }
                Token::RCURLY => {
                    // *i += 1;
                    break;
                }
                _ => res.push(tokens[*i].clone())
            }
        }

        *i += 1;
    }

    return res;
}

fn get_block_body(tokens: &Vec<Vec<Token>>, i: usize) -> usize {
    let mut end = i + 1;
    while end < tokens.len() {
        // :)
        if tokens[end].len() >= 2 {
            match &tokens[end][0] {
                Token::DOT => {
                    match &tokens[end][1] {
                        Token::IDENT(s) => {
                            match s.as_str() {
                                "if" | "elseif" | "else" | "end" => {
                                    break;
                                }
                                _ => {}
                            }
                        }       
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        end += 1;
    }

    return end;
}

pub fn to_rb_type(t: Vec<tokenizer::Type>) -> Vec<Type> {
    let mut new_type = Vec::new();
    for typ in t {
        new_type.push(typ.to_rbtype());
    }

    return new_type;
}

pub fn parse_function(tokens: Vec<Vec<Token>>, wrapper: &mut Wrapper, link_paths: &mut HashSet<String>) -> Expr {
    let ret_type = match &tokens[0][0] {
        Token::TYPE(t) => {
            to_rb_type(t.clone())
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
                to_rb_type(t.clone())
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

    let body = parse(tokens[1..].to_vec(), wrapper, link_paths);

    return Expr::FUNCDEF(name, args, ret_type, body);
}

pub fn emit(exprs: &Vec<Expr>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for expr in exprs {
        res.append(&mut expr.to_bytes());
    }

    return res;
}