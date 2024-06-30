use std::{fs, io::Write, path::Path};

use parser::parse;
use tokenizer::{tokenize, Token};

mod tokenizer;
mod parser;
mod _macro;
mod function;
mod data;
mod conversion;
mod loc;

static DEBUG: u8 = 0;
static MACRO_DEPTH_LIMIT: u16 = 16;

fn main() {
    let start = std::time::Instant::now();
    let file_path = "helloworld";

    let contents = fs::read_to_string(file_path.to_string() + ".rasm").expect("failed to read file");

    let lines = contents.split("\n");

    let mut tokens: Vec<Vec<Token>> = Vec::new();
    for line in lines {
        if line.trim().len() > 0 {
            let line_tokens = tokenize(line.to_string());
            if line_tokens.len() > 0 {   
                if DEBUG >= 2 {
                    println!("{:?}", line_tokens);
                }

                tokens.push(line_tokens);
            }
        }
    }

    println!("assembly took {:.2}ms", start.elapsed().as_secs_f32() * 1000f32);   

    let rbb_file = "./".to_string() + file_path + ".rbb";

    if Path::new(&rbb_file).exists() {
       fs::remove_file(rbb_file.clone()).unwrap();
    }

    let bytes = parse(tokens);
    let mut file = fs::OpenOptions::new().create_new(true).write(true).open(rbb_file).expect("failed to open file to save");
    let _ = file.write_all(&bytes); 
}