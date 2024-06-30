use std::{fs, io::Write, path::Path};

use error::Loc;
use parser::parse;
use tokenizer::{tokenize, Token};

mod tokenizer;
mod parser;
mod _macro;
mod function;
mod data;
mod conversion;
mod error;

static DEBUG: u8 = 0;
static MACRO_DEPTH_LIMIT: u16 = 16;

// TODO: file importing
// TODO: labels
// TODO: structs
fn main() {
    let start = std::time::Instant::now();
    let file_path = "helloworld".to_string();

    let contents = fs::read_to_string(file_path.clone() + ".rasm").expect("failed to read file");

    let lines = contents.split("\n");

    let mut tokens: Vec<Vec<Token>> = Vec::new();
    let mut locations: Vec<Vec<Loc>> = Vec::new();
    let mut loc: Loc = Loc {file: file_path.clone() + ".rasm", line: 0, col: 0};
    for line in lines {
        loc.line = loc.line + 1;
        loc.col = line.chars().take_while(|ch| ch.is_whitespace() && *ch != '\n').count() + 1;
        if line.trim().len() > 0 {
            let line_tokens = tokenize(line.to_string(), &mut loc);
            match line_tokens {
                (toks, locs) => {
                    if toks.len() > 0 {   
                        if DEBUG >= 2 {
                            println!("{:?}", toks);
                            println!("{:?}", locs);
                        }
                        tokens.push(toks);
                        locations.push(locs);
                    }
                }
            }
        }
    }

    println!("assembly took {:.2}ms", start.elapsed().as_secs_f32() * 1000f32);   

    let rbb_file = "./".to_string() + &file_path.clone() + ".rbb";

    let bytes = parse(tokens, locations);

    if Path::new(&rbb_file).exists() {
       fs::remove_file(rbb_file.clone()).unwrap();
    }

    let mut file = fs::OpenOptions::new().create_new(true).write(true).open(rbb_file).expect("failed to open file to save");
    let _ = file.write_all(&bytes); 
}