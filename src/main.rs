use std::fs;

use tokenizer::tokenize;

mod tokenizer;

fn main() {
    let start = std::time::Instant::now();
    let file_path = "helloworld.rasm";

    let contents = fs::read_to_string(file_path).expect("failed to read file");
    let lines = contents.split("\n");

    for line in lines {
        if line.trim().len() > 0 {
            let tokens = tokenize(line.to_string());
            println!("{:?}", tokens)
        }
    }
    eprintln!("assembly took {:.2}ms", start.elapsed().as_secs_f32() * 1000f32);
}