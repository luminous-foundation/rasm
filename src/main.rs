use std::{collections::HashSet, env::{self}, fs, io::Write, path::Path};

use parser::{emit, parse};
use rainbow_wrapper::wrapper::Wrapper;
use tokenizer::{tokenize, Token};

mod tokenizer;
mod number;
mod parser;
mod expr;
mod instruction;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("expected file");
    }
    if !args[1].ends_with(".rasm") {
        panic!("expected RASM file");
    }

    let mut i = 0;
    let mut link_paths: HashSet<String> = HashSet::new();
    while i < args.len() {
        match args[i].as_str() {
            "-l" | "--link" => {
                i += 1;
                add_link_path(args[i].clone(), &mut link_paths);
            }
            _ => {}
        }
        i += 1;
    }

    assemble(args[1].clone(), &mut link_paths);
}

pub fn assemble(rasm_file: String, link_paths: &mut HashSet<String>) {
    println!("assembling {rasm_file}");

    let file = rasm_file.split(".rasm").collect::<Vec<&str>>()[0];
    let folder = rasm_file.split(|c| c == '\\' || c == '/').collect::<Vec<&str>>();
    let folder = folder[0..folder.len()-1].to_vec().join("/") + "/";

    add_link_path(folder, link_paths);

    let contents = fs::read_to_string(file.to_string() + ".rasm").expect("failed to read file");

    let mut tokens: Vec<Vec<Token>> = Vec::new();
    
    let lines = contents.split("\n");
    for line in lines {
        let line = tokenize(line.to_string());
        tokens.push(line);
    }

    let mut wrapper = Wrapper::new();

    let program = emit(&parse(tokens, &mut wrapper, link_paths));
    wrapper.push(program);

    let bytes = wrapper.emit();

    let rbb_file = file.to_string() + ".rbb";
    if Path::new(&rbb_file).exists() {
       fs::remove_file(rbb_file.clone()).unwrap();
    }

    let mut file = fs::OpenOptions::new().create_new(true).write(true).open(rbb_file).expect("failed to open file to save");
    let _ = file.write_all(&bytes);
}

// this function shouldnt need to exist
fn add_link_path(mut folder: String, link_paths: &mut HashSet<String>) {
    folder = folder.replace("\\", "/");
    if folder.ends_with("/") {
        folder = folder[..folder.len()-1].to_string();
    }
    link_paths.insert(folder);
}