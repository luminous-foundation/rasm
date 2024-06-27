use std::collections::HashMap;

use crate::function::Function;
use crate::tokenizer::{Token, Type};
use crate::_macro::Macro;
use crate::DEBUG;

pub fn parse(toks: Vec<Vec<Token>>) -> Vec<u8> {
    let result: Vec<u8> = Vec::new();

    let macros;
    match parse_macros(&toks) {
        Ok(list) => macros = list,
        Err(error) => panic!("error while parsing macros\n{error:?}")
    }

    if DEBUG == 1 {
        println!("parsed {} macro(s), {:?}", macros.len(), macros);
    }

    let functions;
    match parse_functions(&toks) {
        Ok(list) => functions = list,
        Err(error) => panic!("error while parsing functions\n{error:?}")
    }

    if DEBUG == 1 {
        println!("parsed {} function(s), {:?}", functions.len(), functions);
    }

    return result;
}

fn parse_functions(toks: &Vec<Vec<Token>>) -> Result<HashMap<String, Function>, String> {
    let mut functions: HashMap<String, Function> = HashMap::new();

    let mut i = 0;
    while i < toks.len() {
        let line = &toks[i];
        
        if line.len() > 3 {
            let mut is_func = matches!(
                (&line[0], &line[1], &line[2]),
                (&Token::TYPE(_), &Token::IDENT(_), &Token::LPAREN)
            );

            if !line.contains(&Token::LCURLY) {
                if line[line.len() - 1] != Token::RPAREN {
                    is_func = false;
                }

                i = i + 1;
            } else {
                if line[line.len() - 2] != Token::RPAREN && line[line.len() - 1] != Token::LCURLY {
                    is_func = false;
                }
            }

            if is_func {
                let return_type;
                match &line[0] {
                    Token::TYPE(t) => return_type = *t,
                    _ => {
                        return Err("unreachable".to_string())
                    }
                }

                let name;
                match &line[1] {
                    Token::IDENT(function_name) => name = function_name,
                    _ => {
                        return Err("unreachable".to_string())
                    }
                }

                if DEBUG >= 1 {
                    println!("found function named {}", name);
                }

                let mut arg_types: Vec<Type> = Vec::new();
                let mut arg_names: Vec<String> = Vec::new();

                let mut j = 3;

                while line[j] != Token::RPAREN {
                    let _type;
                    match line[j] {
                        Token::TYPE(t) => _type = t,
                        _ => return Err(format!("unexpected token {:?}, expected TYPE", line[j]))
                    }

                    let name;
                    match &line[j + 1] {
                        Token::IDENT(n) => name = n,
                        _ => return Err(format!("unexpected token {:?}, expected IDENT", line[j]))
                    }

                    arg_types.push(_type);
                    arg_names.push(name.clone());

                    j = j + 2;
                }

                let mut function = Function {name: name.to_string(), arg_types, arg_names, return_type, body: Vec::new()};
                
                i = i + 1;
                while !toks[i].contains(&Token::RCURLY) {
                    function.body.push(toks[i].clone());

                    i = i + 1;
                }

                functions.insert(name.clone(), function);
            }
        }

        i = i + 1;
    }

    return Ok(functions);
}

// TODO: better error handling
// (print location and file)
fn parse_macros(toks: &Vec<Vec<Token>>) -> Result<HashMap<String, Macro>, String> {
    let mut macros: HashMap<String, Macro> = HashMap::new();

    let mut i = 0;
    while i < toks.len() {
        let line = &toks[i];
        if line[0] == Token::DOT && line[1] == Token::IDENT("macro".to_string()) {
            if !line.contains(&Token::LCURLY) && !&toks[i + 1].contains(&Token::LCURLY) {
                return Err("macro does not have open curly".to_string());
            }

            let name;
            match &line[2] {
                Token::IDENT(macro_name) => name = macro_name,
                _ => {
                    return Err(format!("unexpected token {:?}, expected IDENT", line[2]))
                }
            }

            if DEBUG >= 1 {
                println!("found macro named {}", name);
            }

            if !line.contains(&Token::LCURLY) {
                i = i + 1;
            }

            let mut _macro = Macro {name: name.clone(), content: Vec::new()};

            i = i + 1;
            while !toks[i].contains(&Token::RCURLY) {
                _macro.content.push(toks[i].clone());

                i = i + 1;
            }

            macros.insert(name.clone(), _macro);
        }
        i = i + 1;
    }

    return Ok(macros);
}