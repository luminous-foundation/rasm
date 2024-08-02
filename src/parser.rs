use std::collections::HashMap;
use std::process;

use lazy_static::lazy_static;

use crate::_struct::Struct;
use crate::conversion::{convert_bytecode_string, convert_number, convert_string, convert_type, get_bytes_needed};
use crate::data::Data;
use crate::error::{Error, Loc, Note};
use crate::function::Function;
use crate::tokenizer::{Token, Type};
use crate::_macro::Macro;
use crate::{DEBUG, MACRO_DEPTH_LIMIT};

lazy_static! {
    static ref INSTR_MAP: HashMap<&'static str, u8> = {
        let mut m = HashMap::new();
        m.insert("NOP", 0x00);
        m.insert("PUSH", 0x01);
        m.insert("POP", 0x03);
        m.insert("LDARG", 0x04);
        m.insert("CALL", 0x06);
        m.insert("ADD", 0x08);
        m.insert("SUB", 0x0C);
        m.insert("MUL", 0x10);
        m.insert("DIV", 0x14);
        m.insert("JMP", 0x18);
        m.insert("JNE", 0x1A);
        m.insert("JE", 0x22);
        m.insert("JGE", 0x2A);
        m.insert("JG", 0x32);
        m.insert("JLE", 0x3A);
        m.insert("JL", 0x42);
        m.insert("MOV", 0x4A);
        m.insert("AND", 0x50);
        m.insert("OR", 0x54);
        m.insert("XOR", 0x58);
        m.insert("NOT", 0x5C);
        m.insert("LSH", 0x5E);
        m.insert("RSH", 0x62);
        m.insert("VAR", 0x66);
        m.insert("RET", 0x6A);
        m.insert("DEREF", 0x6D);
        m.insert("REF", 0x6F);
        m.insert("INST", 0x70);
        m.insert("MOD", 0x72);
        m
    };
}

// TODO: show where errors expanded from
pub fn parse(mut toks: Vec<Vec<Token>>, mut locs: Vec<Vec<Loc>>) -> Vec<u8> {
    let debug = *DEBUG.lock().unwrap();

    let mut result: Vec<u8> = Vec::new();

    let mut vars: Vec<String> = Vec::new();

    let mut macros;
    match parse_macros(&toks, &locs) {
        Ok(list) => macros = list,
        Err(error) => {
            eprintln!("error while parsing macros:\n{}", error);
            process::exit(1);
        }
    }

    if debug == 1 {
        println!("parsed {} macro(s), {:#?}", macros.len(), macros);
    }

    let mut functions;
    match parse_functions(&toks, &locs) {
        Ok(list) => functions = list,
        Err(error) => {
            eprintln!("error while parsing functions:\n{}", error);
            process::exit(1);
        }
    }

    if debug >= 1 {
        println!("parsed {} function(s), {:#?}", functions.len(), functions);
    }

    let data;
    match parse_data(&toks, &locs) {
        Ok(t) => data = t,
        Err(error) => {
            eprintln!("error while parsing data section:\n{}", error);
            process::exit(1);
        }
    }

    if debug >= 1 {
        println!("parsed {} data value(s), {:#?}", data.len(), data);
    }

    let structs;
    match parse_structs(&toks, &locs) {
        Ok(s) => structs = s,
        Err(error) => {
            eprintln!("error while parsing structs:\n{}", error);
            process::exit(1);
        }
    }

    if debug >= 1 {
        println!("parsed {} struct(s), {:#?}", structs.len(), structs);
    }

    // these two loops would all be a function if the borrow checker let me
    // i might make it a function some day but right now i cannot be bothered to figure it out
    let mut resolved_macro = true;
    let mut iteration = 0;

    let mut last_macros: Vec<String> = Vec::new();
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
                            
                            if !last_macros.contains(&name) {
                                last_macros.push(name.clone());
                            }

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

            new_macros.insert(_macro.name.clone(), _macro);
        }

        macros = new_macros;
        iteration = iteration + 1;
    }

    if iteration == MACRO_DEPTH_LIMIT {
        last_macros.sort();

        eprintln!("{}", Error {
            loc: macros.get(&last_macros[0]).expect("unreachable").loc.clone(),
            message: "hit macro depth limit".to_string()
        });

        for i in 1..last_macros.len() { // skip the first one
            eprintln!("{}", Note {
                loc: macros.get(&last_macros[i]).expect("unreachable").loc.clone(),
                message: "also this macro".to_string()
            });
        }
        process::exit(1);
    }

    let mut new_functions: HashMap<String, Function> = HashMap::new();
    for (name, mut function) in functions {
        let mut new_body: Vec<Vec<Token>> = Vec::new();
        let mut new_locs: Vec<Vec<Loc>> = Vec::new();

        let mut i = 0;
        let mut line_num = 0;
        while i < function.body.len() {
            if function.body[i].len() > 0 {
                match &function.body[i][0] {
                    Token::IDENT(n) => {
                        if macros.contains_key(n) {
                            let referenced_macro = macros.get(n).expect("unreachable");
                            let mut j = 0;
                            for line in &referenced_macro.content {
                                new_body.push(Vec::new());
                                
                                new_locs.push(function.body_loc[i].clone());

                                // if j > 0 {
                                    // let mut new_loc = referenced_macro.content_loc[j].clone();
                                    // for l in &mut new_loc {
                                    //     l.line = function.loc.line + i;
                                    // }
                                    // locs.push(new_loc);
                                // }

                                for token in line {
                                    match token {
                                        Token::IDENT(n) => {
                                            if referenced_macro.args.contains(n) {
                                                let index = referenced_macro.args.iter().position(|r| r == n).unwrap();
                                                new_body[line_num + j].push(function.body[i][index + 1].clone());
                                            } else {
                                                new_body[line_num + j].push(token.clone());
                                            }
                                        }
                                        _ => new_body[line_num + j].push(token.clone())
                                    }
                                }
                                j = j + 1;
                            }
                            line_num = line_num + j - 1;
                        } else {
                            new_body.push(Vec::new());
                            new_body[line_num].append(&mut function.body[i]);
                            
                            new_locs.push(function.body_loc[i].clone());
                        }
                    }
                    _ => {
                        new_body.push(Vec::new());
                        new_body[line_num].append(&mut function.body[i]);
                        
                        new_locs.push(function.body_loc[i].clone());
                    }
                }
            } else {
                new_body.push(Vec::new());
                
                new_locs.push(function.body_loc[i].clone());
            }

            line_num = line_num + 1;
            i = i + 1;
        }

        function.body = new_body;
        function.body_loc = new_locs;

        new_functions.insert(name, function);
    }
    functions = new_functions;

    let mut i = 0;
    for line in &mut locs {
        for l in line {
            l.line = i + 1;
        }
        i = i + 1;
    }

    // if debug >= 1 {
    //     println!("after macro resolution...");
    //     println!("parsed {} macro(s), {:?}", macros.len(), macros);
    //     println!("parsed {} functions(s), {:?}", functions.len(), functions);
    //     // println!("locs\n");
    //     // for i in 0..locs.len() {
    //     //     println!("{:?}", locs[i]);
    //     // }
    // }

    for (_, _struct) in structs {
        result.append(&mut emit_struct(&_struct));
    }

    let mut i = 0;
    let functional_tokens = &[Token::DOT, Token::LCURLY, Token::RCURLY, Token::LPAREN, Token::RPAREN];

    let mut in_top_level = true;
    while i < toks.len() {
        let mut line = &mut toks[i];

        if functional_tokens.iter().any(|token| line.contains(token)) {
            in_top_level = false;
        }

        if in_top_level {
            match emit_line(&mut line, &functions, &locs, i, &mut vars) {
                Ok(mut bytes) => result.append(&mut bytes),
                Err(error) => {
                    eprintln!("error while parsing emitting line:\n{}", error);
                    process::exit(1);
                }
            }
        }

        i = i + 1;
    }

    for (_, mut function) in &mut (functions.clone()) {
        match emit_function(&mut function, &functions, &vars) {
            Ok(mut bytes) => result.append(&mut bytes),
            Err(error) => {
                eprintln!("error while parsing emitting function:\n{}", error);
                process::exit(1);
            }
        }
    }

    if data.len() > 0 {
        result.push(0xFC);
        for (_, dat) in data {
            result.append(&mut emit_data(&dat));
        }
    }

    return result;
}

fn emit_struct(_struct: &Struct) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    bytes.push(0xFB);
    bytes.append(&mut convert_bytecode_string(&_struct.name));
    bytes.push(0xFE);

    for i in 0.._struct.var_types.len() {
        bytes.append(&mut convert_type(&_struct.var_types[i]));
        bytes.append(&mut convert_bytecode_string(&_struct.var_names[i]));
    }

    bytes.push(0xFD);
    
    return bytes;
}

fn emit_data(data: &Data) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    bytes.append(&mut convert_bytecode_string(&data.name));
    bytes.append(&mut convert_type(&data._type));
    bytes.push(get_bytes_needed((data.data.len() as u64).into()));
    bytes.append(&mut convert_number((data.data.len() as u64).into()));
    
    for b in &data.data {
        bytes.push(*b);
    }

    return bytes;
}

fn emit_function(function: &mut Function, functions: &HashMap<String, Function>, vars: &Vec<String>) -> Result<Vec<u8>, Error> {
    let mut bytes: Vec<u8> = Vec::new();

    let mut local_vars = vars.clone();

    bytes.push(0xFF);
    
    bytes.append(&mut convert_type(&function.return_type));
    bytes.append(&mut convert_bytecode_string(&function.name));

    for i in 0..function.arg_names.len() {
        let arg_type = &function.arg_types[i];
        let arg_name = &function.arg_names[i];

        bytes.append(&mut convert_type(&arg_type));
        bytes.append(&mut convert_bytecode_string(&arg_name));
    }

    bytes.push(0xFE);

    let mut line_num = 0;
    for mut line in &mut function.body {
        match emit_line(&mut line, functions, &function.body_loc, line_num, &mut local_vars) {
            Ok(mut b) => bytes.append(&mut b),
            Err(error) => return Err(error)
        }
        line_num = line_num + 1;
    }

    bytes.push(0xFD);

    return Ok(bytes);
}

fn emit_line(line: &mut Vec<Token>, functions: &HashMap<String, Function>, locs: &Vec<Vec<Loc>>, line_num: usize, vars: &mut Vec<String>) -> Result<Vec<u8>, Error> {
    let mut bytes: Vec<u8> = Vec::new();

    let mut i = 1;
    while i < line.len() { // this is to merge all x.y identifiers into one
        match &line[i] {
            Token::DOT => {
                match &line[i - 1] {
                    Token::IDENT(n) => {
                        match &line[i + 1] { // matches in matches in matches
                            Token::IDENT(n2) => {
                                let new_ident = n.to_string() + "." + n2;
                                line.remove(i - 1);
                                line.remove(i - 1);
                                line.remove(i - 1);
                                line.insert(i - 1, Token::IDENT(new_ident));
                                i = i - 1;
                            }
                            _ => return Err(Error {
                                loc: locs[line_num][i + 1].clone(),
                                message: format!("unexpected token `{:?}`, expected `IDENT`", line[i + 1])
                            })
                        }
                    }
                    _ => return Err(Error {
                        loc: locs[line_num][i - 1].clone(),
                        message: format!("unexpected token `{:?}`, expected `IDENT`", line[i - 1])
                    })
                }
            }
            _ => ()
        }

        i = i + 1;
    }

    if line.len() > 0 {
        match &line[0] {
            Token::IDENT(instr) => {
                let mut variation: u8;
                match instr.to_ascii_uppercase().as_str() {
                    "NOP" | "POP" | "REF" => variation = 0, // all non-variant instructions
                    "PUSH" | "LDARG" | "JMP" | "NOT" | "DEREF" => { // all [imm/var] instructions
                        match get_variation(&line, 1) {
                            Ok(v) => variation = v,
                            Err(err) => 
                            return Err(Error { 
                                loc: locs[line_num][0].clone(),
                                message: err
                            })
                        }
                    }
                    "ADD" | "SUB" | "MUL" | "DIV" | "AND" | "OR" | "XOR" | "LSH" | "RSH" => { // all [imm/var] [imm/var] instructions
                        match get_variation(&line, 2) {
                            Ok(v) => variation = v,
                            Err(err) => 
                            return Err(Error { 
                                loc: locs[line_num][0].clone(),
                                message: err
                            })
                        }
                    }
                    "JNE" | "JE" | "JGE" | "JG" | "JLE" | "JL" => { // all [imm/var] [imm/var] [imm/var] instructions
                        match get_variation(&line, 3) {
                            Ok(v) => variation = v,
                            Err(err) => 
                            return Err(Error { 
                                loc: locs[line_num][0].clone(),
                                message: err
                            })
                        }
                    }
                    "CALL" => { // CALL is special ([func/var])
                        match &line[1] {
                            Token::IDENT(ident) => {
                                if functions.contains_key(ident) {
                                    variation = 0;
                                } else {
                                    variation = 1;
                                }
                            }
                            _ => return Err(Error {
                                loc: locs[line_num][1].clone(),
                                message: format!("unexpected token `{:?}`, expected `IDENT`", line[1])
                            })
                        }
                    }
                    "MOV" => { // MOV is special ([imm/var*] [var*])
                        match &line[1] {
                            Token::NUMBER(_) => variation = 0,
                            Token::IDENT(n) => {
                                if n.starts_with("&") {
                                    variation = 2;
                                } else {
                                    variation = 1;
                                }
                            }
                            _ => return Err(Error {
                                loc: locs[line_num][1].clone(),
                                message: format!("unexpected token `{:?}`, expected `NUMBER` or `IDENT`", line[1])
                            })
                        }
                        match &line[2] {
                            Token::IDENT(n) => {
                                if n.starts_with("&") {
                                    variation += 3;
                                }
                            }
                            _ => return Err(Error {
                                loc: locs[line_num][1].clone(),
                                message: format!("unexpected token `{:?}`, expected `IDENT`", line[2])
                            })
                        }
                    }
                    "VAR" => { // VAR is special ([type/var] [name/var])
                        match &line[1] {
                            Token::TYPE(_) => variation = 0,
                            Token::IDENT(_) => variation = 1,
                            _ => return Err(Error {
                                loc: locs[line_num][1].clone(),
                                message: format!("unexpected token `{:?}`, expected `TYPE` or `IDENT`", line[1])
                            })
                        }
                        match &line[2] {
                            Token::IDENT(n) => {
                                if n.starts_with("&") {
                                    variation += 2;
                                }

                                vars.push(n.clone());
                            }
                            _ => return Err(Error {
                                loc: locs[line_num][2].clone(),
                                message: format!("unexpected token `{:?}`, expected `IDENT`", line[2])
                            })
                        }
                    }
                    "RET" => { // RET is special {imm/var}
                        if line.len() > 1 {
                            match get_variation(&line, 1) {
                                Ok(v) => variation = v + 1,
                                Err(err) => 
                                return Err(Error { 
                                    loc: locs[line_num][0].clone(),
                                    message: err
                                })
                            }
                        } else {
                            variation = 0;
                        }
                    }
                    "INST" => { // INST is special ([name/var] [var])
                        match &line[1] {
                            Token::IDENT(n) => {
                                if vars.contains(n) {
                                    variation = 1;
                                } else {
                                    variation = 0;
                                }
                            }
                            _ => return Err(Error {
                                loc: locs[line_num][2].clone(),
                                message: format!("unexpected token `{:?}`, expected `IDENT`", line[2])
                            })
                        }
                    }
                    _ => return Err(Error {
                        loc: locs[line_num][0].clone(),
                        message: format!("unknown instruction `{}`", instr)
                    })
                }

                let byte = INSTR_MAP.get(instr.as_str()).expect("unreachable, the previous step should've caught this").clone();
                bytes.push(byte + variation);
            }
            _ => return Err(Error {
                loc: locs[line_num][1].clone(),
                message: format!("unexpected token `{:?}`, expected `IDENT`", line[0])
            })
        }

        for i in 1..line.len() {
            match emit_token(&line[i], &mut bytes) {
                Ok(_) => (),
                Err(err) => 
                return Err(Error { 
                    loc: locs[line_num][i].clone(),
                    message: err
                })
            }
        }
    }

    return Ok(bytes);
}

fn emit_token(token: &Token, bytes: &mut Vec<u8>) -> Result<(), String> {
    match token {
        Token::IDENT(str) => bytes.append(&mut convert_bytecode_string(str)),
        Token::NUMBER(n) => bytes.append(&mut convert_number(n.clone())),
        Token::TYPE(t) => bytes.append(&mut convert_type(t)),
        _ => return Err(format!("unexpected token `{:?}`, expected `IDENT`, `NUMBER` or `TYPE`", token))
    }

    return Ok(());
}

fn get_variation(line: &Vec<Token>, amnt: usize) -> Result<u8, String> {
    let mut variation = 0;

    for i in 0..amnt {
        match &line[i + 1] {
            Token::IDENT(_) => variation = variation | (1 << i),
            Token::NUMBER(_) => (),
            _ => return Err(format!("unexpected token `{:?}`, expected `NUMBER` or `IDENT`", line[i]))
        }
    }

    return Ok(variation);
}

fn parse_structs(toks: &Vec<Vec<Token>>, locs: &Vec<Vec<Loc>>) -> Result<HashMap<String, Struct>, Error> {
    let mut structs: HashMap<String, Struct> = HashMap::new();

    let debug = *DEBUG.lock().unwrap();

    let mut i = 0;
    while i < toks.len() {
        let line = &toks[i];

        if line.len() >= 2 {
            // i would rather this be IDENT("struct") instead of TYPE(struct) but whatever the assembler doesn't care
            if line[0] == Token::DOT && line[1] == Token::TYPE(vec!(Type::STRUCT)) {
                if !line.contains(&Token::LCURLY) && !&toks[i + 1].contains(&Token::LCURLY) {
                    return Err(Error {
                        loc: locs[i][0].clone(),
                        message: "struct does not have open curly".to_string()
                    });
                }

                let name;
                match &line[2] {
                    Token::IDENT(n) => name = n,
                    _ => {
                        return Err(Error {
                            loc: locs[i][2].clone(),
                            message: format!("unexpected token `{:?}`, expected `IDENT`", line[2])
                        });
                    }
                }

                if debug >= 1 {
                    println!("found struct named {}", name);
                }

                i = i + 1;

                let mut _struct = Struct {name: name.clone(), var_names: Vec::new(), var_types: Vec::new()};
                while !toks[i].contains(&Token::RCURLY) {
                    let line = &toks[i];
                    
                    let _type;
                    match &line[0] {
                        Token::TYPE(t) => _type = t,
                        _ =>  {
                            return Err(Error {
                                loc: locs[i][0].clone(),
                                message: format!("unexpected token `{:?}`, expected `TYPE`", line[2])
                            });
                        }
                    }

                    let name;
                    match &line[1] {
                        Token::IDENT(n) => name = n,
                        _ =>  {
                            return Err(Error {
                                loc: locs[i][1].clone(),
                                message: format!("unexpected token `{:?}`, expected `IDENT`", line[2])
                            });
                        }
                    }

                    _struct.var_names.push(name.clone());
                    _struct.var_types.push(_type.clone());
                    i = i + 1;
                }

                structs.insert(name.clone(), _struct);
            }
        }

        i = i + 1;
    }

    return Ok(structs);
}

fn parse_data(toks: &Vec<Vec<Token>>, locs: &Vec<Vec<Loc>>) -> Result<HashMap<String, Data>, Error> {
    let debug = *DEBUG.lock().unwrap();

    let mut data: HashMap<String, Data> = HashMap::new();

    let mut in_section = false;
    let mut i = 0;
    while i < toks.len() {
        let line = &toks[i];

        if in_section {
            let name;
            match &line[0] {
                Token::IDENT(n) => name = n,
                _ => 
                return Err(Error {
                    loc: locs[i][0].clone(),
                    message: format!("unexpected token `{:?}`, expected `IDENT`", line[0])
                })
            }

            if debug >= 1 {
                println!("found data value named {}", name);
            }

            let _type;
            match &line[1] {
                Token::TYPE(n) => _type = n,
                _ => 
                return Err(Error {
                    loc: locs[i][0].clone(),
                    message: format!("unexpected token `{:?}`, expected `TYPE`", line[1])
                })
            }

            let bytes: Vec<u8>;
            match &line[2] { // i'm just going to implement this as i go along
                Token::NUMBER(n) => bytes = convert_number(n.clone()),
                Token::TYPE(t) => bytes = convert_type(t),
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

        if line.len() > 0 {
            if line[0] == Token::DOT && line[1] == Token::IDENT("data".to_string()) {
                in_section = true;
            }
        }

        i = i + 1;
    }

    return Ok(data);
}

fn parse_functions(toks: &Vec<Vec<Token>>, locs: &Vec<Vec<Loc>>) -> Result<HashMap<String, Function>, Error> {
    let mut functions: HashMap<String, Function> = HashMap::new();

    let debug = *DEBUG.lock().unwrap();

    let mut i = 0;
    while i < toks.len() {
        let line = &toks[i];
        let start = i;
        
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
                        return Err(Error {
                            loc: locs[i][0].clone(),
                            message: "unreachable".to_string()
                        })
                    }
                }

                let name;
                match &line[1] {
                    Token::IDENT(function_name) => name = function_name,
                    _ => {
                        return Err(Error {
                            loc: locs[i][1].clone(),
                            message: "unreachable".to_string()
                        })
                    }
                }

                if debug >= 1 {
                    println!("found function named {}", name);
                }

                let mut arg_types: Vec<Vec<Type>> = Vec::new();
                let mut arg_names: Vec<String> = Vec::new();

                let mut j = 3;

                while line[j] != Token::RPAREN {
                    let _type;
                    match &line[j] {
                        Token::TYPE(t) => _type = t,
                        _ => 
                        return Err(Error {
                            loc: locs[i][j].clone(),
                            message: format!("unexpected token `{:?}`, expected `TYPE`", line[j])
                        })
                    }

                    let name;
                    match &line[j + 1] {
                        Token::IDENT(n) => name = n,
                        _ => 
                        return Err(Error {
                            loc: locs[i][j].clone(),
                            message: format!("unexpected token `{:?}`, expected `IDENT`", line[j])
                        })
                    }

                    arg_types.push(_type.clone());
                    arg_names.push(name.clone());

                    j = j + 2;
                }

                let mut function = Function {
                    loc: locs[start][0].clone(), 

                    name: name.to_string(), 
                    arg_types, arg_names, return_type, 
                    
                    body: Vec::new(),
                    body_loc: Vec::new()
                };
                
                i = i + 1;
                while !toks[i].contains(&Token::RCURLY) {
                    function.body.push(toks[i].clone());
                    function.body_loc.push(locs[i].clone());

                    i = i + 1;
                }

                functions.insert(name.clone(), function);
            }
        }

        i = i + 1;
    }

    return Ok(functions);
}

fn parse_macros(toks: &Vec<Vec<Token>>, locs: &Vec<Vec<Loc>>) -> Result<HashMap<String, Macro>, Error> {
    let mut macros: HashMap<String, Macro> = HashMap::new();

    let debug = *DEBUG.lock().unwrap();

    let mut i = 0;
    while i < toks.len() {
        let line = &toks[i];
        let start = i;

        if line.len() > 0 {
            if line[0] == Token::DOT && line[1] == Token::IDENT("macro".to_string()) {
                if !line.contains(&Token::LCURLY) && !&toks[i + 1].contains(&Token::LCURLY) {
                    return Err(Error {
                        loc: locs[i][0].clone(),
                        message: "macro does not have open curly".to_string()
                    });
                }

                let name;
                match &line[2] {
                    Token::IDENT(macro_name) => name = macro_name,
                    _ => {
                        return Err(Error {
                            loc: locs[i][2].clone(),
                            message: format!("unexpected token `{:?}`, expected `IDENT`", line[2])
                        });
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
                                return Err(Error {
                                    loc: locs[i][j].clone(),
                                    message: format!("found duplicate macro argument `{:?}`", line[j])
                                });
                            }
                        }
                        _ => 
                        return Err(Error {
                            loc: locs[i][j].clone(),
                            message: format!("unexpected token `{:?}`, expected `IDENT`", line[j])
                        })
                    }

                    j = j + 1;
                }

                if debug >= 1 {
                    println!("found macro named {}", name);
                }

                if !line.contains(&Token::LCURLY) {
                    i = i + 1;
                }

                let mut _macro = Macro {
                    loc: locs[start][0].clone(), 
                    
                    name: name.clone(), 
                    args: args, 
                    
                    content: Vec::new(),
                    content_loc: Vec::new(),
                };

                i = i + 1;
                while !toks[i].contains(&Token::RCURLY) {
                    _macro.content.push(toks[i].clone());
                    _macro.content_loc.push(locs[i].clone());

                    i = i + 1;
                }

                macros.insert(name.clone(), _macro);
            }
        }

        i = i + 1;
    }

    return Ok(macros);
}