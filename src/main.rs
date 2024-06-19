use std::fs;

use tokenizer::tokenize;

mod tokenizer;

fn main() {
    let file_path = "helloworld.rasm";

    let contents = fs::read_to_string(file_path).expect("failed to read file");
    let lines = contents.split("\n");

    
    for line in lines {
        let tokens = tokenize(line);
        println!("{}", line)
    }
}