use std::{fs, io::Write, path::Path};

use parser::{emit, parse};
use tokenizer::{tokenize, Token};

mod tokenizer;
mod number;
mod parser;
mod expr;
mod instruction;

fn main() {
    let file = "./examples/addition";

    let contents = fs::read_to_string(file.to_string() + ".rasm").expect("failed to read file");

    let mut tokens: Vec<Vec<Token>> = Vec::new();
    
    let lines = contents.split("\n");
    for line in lines {
        let line = tokenize(line.to_string());
        tokens.push(line);
    }

    let bytes = emit(parse(tokens));

    let rbb_file = file.to_string() + ".rbb";
    if Path::new(&rbb_file).exists() {
       fs::remove_file(rbb_file.clone()).unwrap();
    }

    let mut file = fs::OpenOptions::new().create_new(true).write(true).open(rbb_file).expect("failed to open file to save");
    let _ = file.write_all(&bytes); 
}