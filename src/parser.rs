use std::collections::HashMap;

use crate::conversion::convert_string;
use crate::data::Data;
use crate::function::Function;
use crate::tokenizer::{Token, Type};
use crate::_macro::Macro;
use crate::{DEBUG, MACRO_DEPTH_LIMIT};

pub fn parse(toks: Vec<Vec<Token>>) -> Vec<u8> {
    let result: Vec<u8> = Vec::new();

    let mut macros;
    match parse_macros(&toks) {
        Ok(list) => macros = list,
        Err(error) => panic!("error while parsing macros:\n{error}")
    }

    if DEBUG == 1 {
        println!("parsed {} macro(s), {:?}", macros.len(), macros);
    }

    let mut functions;
    match parse_functions(&toks) {
        Ok(list) => functions = list,
        Err(error) => panic!("error while parsing functions\n:{error}")
    }

    if DEBUG == 1 {
        println!("parsed {} function(s), {:?}", functions.len(), functions);
    }

    let mut data;
    match parse_data(&toks) {
        Ok(t) => data = t,
        Err(error) => panic!("error while parsing data section:\n{error}")
    }

    if DEBUG == 1 {
        println!("parsed {} data value(s), {:?}", data.len(), data);
    }

    // these two loops would all be a function if the borrow checker let me
    // i might make it a function some day but right now i cannot be bothered to figure it out
    let mut resolved_macro = true;
    let mut iteration = 0;
    while resolved_macro && iteration < MACRO_DEPTH_LIMIT { //TODO: smarter macro depth limit system
        resolved_macro = false;
        // borrow checker madness
        // for some ungodly reason iterating over a hashmap takes ownership of the hashmap
        let rust_why = macros.clone(); // resulting in this cloning
        let mut new_macros: HashMap<String, Macro> = HashMap::new(); // and reconstructing mess
        for (name, mut _macro) in macros {
            // again i'd just modify the original macros content (even a clone of it)
            // but rust doesn't let me because somehow im doing mutable and immutable borrows when im not
            let mut new_content: Vec<Vec<Token>> = Vec::new();

            let mut i = 0;
            while i < _macro.content.len() {
                match &_macro.content[i][0] {
                    Token::IDENT(n) => {
                        if rust_why.contains_key(n) {
                            resolved_macro = true;

                            let referenced_macro = rust_why.get(n).expect("unreachable");
                            let mut j = 0;
                            for line in &referenced_macro.content {
                                new_content.push(Vec::new());

                                for token in line {
                                    match token {
                                        Token::IDENT(n) => {
                                            if referenced_macro.args.contains(n) {
                                                let index = referenced_macro.args.iter().position(|r| r == n).unwrap();
                                                new_content[i + j].push(_macro.content[i][index + 1].clone());
                                            } else {
                                                new_content[i + j].push(token.clone());
                                            }
                                        }
                                        _ => new_content[i + j].push(token.clone())
                                    }
                                }
                                j = j + 1;
                            }
                        } else {
                            new_content.push(Vec::new());
                            new_content[i].append(&mut _macro.content[i]);
                        }
                    }
                    _ => {
                        new_content.push(Vec::new());
                        new_content[i].append(&mut _macro.content[i]);
                    }
                }

                i = i + 1;
            }

            _macro.content = new_content;

            new_macros.insert(name, _macro);
        }

        macros = new_macros;
        iteration = iteration + 1;
    }

    if iteration == MACRO_DEPTH_LIMIT {
        panic!("error while resolving macros:\nhit macro depth limit");
    }

    let mut new_functions: HashMap<String, Function> = HashMap::new();
    for (name, mut function) in functions {
        let mut new_body: Vec<Vec<Token>> = Vec::new();

        let mut i = 0;
        while i < function.body.len() {
            match &function.body[i][0] {
                Token::IDENT(n) => {
                    if macros.contains_key(n) {
                        let referenced_macro = macros.get(n).expect("unreachable");
                        let mut j = 0;
                        for line in &referenced_macro.content {
                            new_body.push(Vec::new());

                            for token in line {
                                match token {
                                    Token::IDENT(n) => {
                                        if referenced_macro.args.contains(n) {
                                            let index = referenced_macro.args.iter().position(|r| r == n).unwrap();
                                            new_body[i + j].push(function.body[i][index + 1].clone());
                                        } else {
                                            new_body[i + j].push(token.clone());
                                        }
                                    }
                                    _ => new_body[i + j].push(token.clone())
                                }
                            }
                            j = j + 1;
                        }
                    } else {
                        new_body.push(Vec::new());
                        new_body[i].append(&mut function.body[i]);
                    }
                }
                _ => {
                    new_body.push(Vec::new());
                    new_body[i].append(&mut function.body[i]);
                }
            }

            i = i + 1;
        }

        function.body = new_body;

        new_functions.insert(name, function);
    }
    functions = new_functions;

    if DEBUG >= 1 {
        println!("after macro resolution...");
        println!("parsed {} macro(s), {:?}", macros.len(), macros);
        println!("parsed {} functions(s), {:?}", functions.len(), functions);
    }

    return result;
}

fn parse_data(toks: &Vec<Vec<Token>>) -> Result<HashMap<String, Data>, String> {
    let mut data: HashMap<String, Data> = HashMap::new();

    let mut in_section = false;
    let mut i = 0;
    while i < toks.len() {
        let line = &toks[i];

        if in_section {
            let name;
            match &line[0] {
                Token::IDENT(n) => name = n,
                _ => return Err(format!("unexpected token {:?}, expected IDENT", line[0]))
            }

            if DEBUG >= 1 {
                println!("found data value named {}", name);
            }

            let _type;
            match &line[1] {
                Token::TYPE(n) => _type = n,
                _ => return Err(format!("unexpected token {:?}, expected TYPE", line[0]))
            }

            let bytes: Vec<u8>;
            match &line[2] { // i'm just going to implement this as i go along
                Token::NUMBER(_) => todo!(),
                Token::TYPE(_) => todo!(),
                Token::LPAREN => todo!(),
                Token::RPAREN => todo!(),
                Token::LCURLY => todo!(),
                Token::RCURLY => todo!(),
                Token::LSQUARE => todo!(),
                Token::RSQUARE => todo!(),
                Token::STRING(text) => bytes = convert_string(text),
                Token::DOT => todo!(),
                Token::COMMA => todo!(),
                Token::IDENT(_) => todo!(),
            }

            data.insert(name.clone(), Data {name: name.clone(), _type: _type.clone(), data: bytes});
        }

        if line[0] == Token::DOT && line[1] == Token::IDENT("data".to_string()) {
            in_section = true;
        }

        i = i + 1;
    }

    return Ok(data);
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
                    Token::TYPE(t) => return_type = t.clone(),
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

                let mut arg_types: Vec<Vec<Type>> = Vec::new();
                let mut arg_names: Vec<String> = Vec::new();

                let mut j = 3;

                while line[j] != Token::RPAREN {
                    let _type;
                    match &line[j] {
                        Token::TYPE(t) => _type = t,
                        _ => return Err(format!("unexpected token {:?}, expected TYPE", line[j]))
                    }

                    let name;
                    match &line[j + 1] {
                        Token::IDENT(n) => name = n,
                        _ => return Err(format!("unexpected token {:?}, expected IDENT", line[j]))
                    }

                    arg_types.push(_type.clone());
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

            let mut args: Vec<String> = Vec::new();

            let mut j = 3;
            while j < line.len() && line[j] != Token::LCURLY {
                match &line[j] {
                    Token::IDENT(n) => {
                        if !args.contains(n) {
                            args.push(n.clone())
                        } else {
                            return Err(format!("found duplicate macro argument {:?}", line[j]))
                        }
                    }
                    _ => return Err(format!("unexpected token {:?}, expected IDENT", line[j]))
                }

                j = j + 1;
            }

            if DEBUG >= 1 {
                println!("found macro named {}", name);
            }

            if !line.contains(&Token::LCURLY) {
                i = i + 1;
            }

            let mut _macro = Macro {name: name.clone(), args: args, content: Vec::new()};

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