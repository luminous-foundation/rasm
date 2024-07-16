use std::{env, fs::{self, metadata, File}, io::Write, path::Path, process::{self, Command, Output}, sync::Mutex};

use lazy_static::lazy_static;

use colored::{ColoredString, Colorize};
use error::{printerr, Loc};
use parser::parse;
use tokenizer::{tokenize, Token};

mod tokenizer;
mod parser;
mod _macro;
mod function;
mod data;
mod conversion;
mod error;
mod _struct;
mod number;

lazy_static! {
    static ref DEBUG: Mutex<u8> = Mutex::new(0);
}

static MACRO_DEPTH_LIMIT: u16 = 16;

// TODO: file importing
// TODO: labels
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut timing = false;
    if args.len() > 1 {
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--debug" => {
                    if args.len() - i < 2 {
                        printerr(format!("expected debug level"));
                        process::exit(1);
                    }

                    let mut debug = DEBUG.lock().unwrap();
                    *debug = args[i + 1].parse::<u8>().unwrap();

                    i = i + 2;
                }
                "--time" => {
                    timing = true;
                    i = i + 1;
                }
                "assemble" => {
                    if args.len() - i < 2 {
                        printerr(format!("expected input file"));
                        process::exit(1);
                    }

                    assemble(args[i + 1].clone(), timing);
                    i = i + 2;
                }
                "test" => {
                    if args.len() - i < 2 {
                        printerr(format!("expected either run or update"));
                        process::exit(1);
                    }
                    if args.len() - 1 < 3 {
                        printerr(format!("expected test folder"));
                        process::exit(1);
                    }

                    match args[2].as_str() {
                        "run" => {
                            tests_run(args[i + 2].clone());
                        }
                        "update" => {
                            if metadata(args[i + 2].clone()).unwrap().is_dir() {
                                tests_update(args[i + 2].clone());
                            } else {
                                update_singular(args[i + 2].clone());
                            }
                        }
                        _ => {
                            printerr(format!("expected either run or update"));
                            process::exit(1);
                        }
                    }

                    i = i + 3;
                }
                _ => {
                    usage();
                    printerr(format!("unknown subcommand `{}`", args[i]));
                    process::exit(1);
                }
            }
        }
    } else {
        usage();
        printerr(format!("expected subcommand"));
        process::exit(1);
    }
}

fn usage() {
    println!("Usage:");
    println!("Flags");
    println!("  --debug  [level]                sets the debug level (0-2)");
    println!("  --time                          enables assembly timing");
    println!("Subcommands");
    println!("  help                            prints this subcommand list");
    println!("  assemble [file]                 assembles the given rasm file to rbb file");
    println!("  test     [run/update] [folder]  updates expected test results or runs tests on a given folder");
}

fn exec_test(self_path: String, file: String) -> Result<Output, String> {
    let prog = Command::new(self_path).args(["assemble", file.as_str()]).output();
    match prog {
        Ok(output) => {
            return Ok(output);
        }
        Err(error) => {
            return Err(format!("failed to run test on {} due to error\nlog:\n{}", file, error));
        }
    }
}

fn tests_run(folder: String) {
    let files = fs::read_dir(folder.clone());
    let current_exe = env::current_exe().expect("failed to get current executable path").display().to_string();

    println!("running tests...");

    let mut pass = 0;
    let mut fail = 0;
    match files {
        Ok(files) => {
            for file in files {
                match file {
                    Ok(file) => {
                        let path: String = file.path().display().to_string();
                        if path.ends_with(".rasm") {
                            let result = exec_test(current_exe.clone(), path.clone());
                            match result {
                                Ok(result) => {
                                    let status: ColoredString;
                                    if check_test(path.clone(), result) {
                                        status = "passed".green().bold();
                                        pass = pass + 1;
                                    } else {
                                        status = "failed".red().bold();
                                        fail = fail + 1;
                                    }
                                    println!("{}: {}", path.clone(), status);
                                }
                                Err(error) => printerr(error)
                            }
                        }
                    }
                    Err(_) => ()
                }
            }
        }
        Err(error) => {
            printerr(format!("failed to run tests on {} due to error\nlog:\n{}", folder, error));
        } 
    }

    println!("{} passed {} failed", pass.to_string().green(), fail.to_string().red());
}

fn process_test(output: Output) -> String {
    return format!("{}\nstdout: \n{}\nstderr: \n{}", output.status, String::from_utf8(output.stdout).unwrap(), String::from_utf8(output.stderr).unwrap());
}

fn check_test(path: String, output: Output) -> bool {
    let path = (&path.as_str()[..path.len()-5]).to_string();

    if !Path::new(&(path.clone() + ".testout")).exists() {
        println!("\nno test out recorded for this test");
        return false;
    }

    let saved_test = fs::read_to_string(path.clone() + ".testout").expect("unreachable, failed to read test file, should already exist");
    let test_res = process_test(output);

    if test_res != saved_test {
        println!("\nEXPECTED:\n{}\nACTUAL:\n{}", saved_test, test_res);
    }

    if Path::new(&(path.clone() + ".rbbtestout")).exists() {
        if !Path::new(&(path.clone() + ".rbb")).exists() {
            println!("\nexpected emitted bytecode, did not find it");
            return false;
        }

        let rbb_out = std::fs::read(path.clone() + ".rbb").unwrap();
        let rbb_expected = std::fs::read(path + ".rbbtestout").unwrap();

        if rbb_out != rbb_expected {
            println!("\nemitted bytecode did not match");
            return false;
        }
    } else {
        if Path::new(&(path.clone() + ".rbb")).exists() {
            println!("\ndid not expect emitted bytecode but found it");
            return false;
        }
    }

    return test_res == saved_test;
}

fn save_test(path: String, output: Output) {
    let path = (&path.as_str()[..path.len()-5]).to_string();
    let mut output_file = File::create(path.clone() + ".testout").expect(format!("failed to create file {}", path).as_str());

    let line = process_test(output);

    let _ = write!(output_file, "{}", line);

    if Path::new(&(path.clone() + ".rbb")).exists() {
        let _ = fs::rename(path.clone() + ".rbb", path + ".rbbtestout");
    } else {
        if Path::new(&(path.clone() + ".rbbtestout")).exists() {
            let _ = fs::remove_file(path.clone() + ".rbbtestout");
        }
    }
}

fn update_singular(test: String) {
    let current_exe = env::current_exe().expect("Failed to get current executable path").display().to_string();

    if test.ends_with(".rasm") {
        let result = exec_test(current_exe.clone(), test.clone());
        match result {
            Ok(result) => save_test(test.clone(), result),
            Err(error) => printerr(error)
        }
    }

    println!("updated test {}", test);
}

fn tests_update(folder: String) {
    let files = fs::read_dir(folder.clone());
    let current_exe = env::current_exe().expect("failed to get current executable path").display().to_string();

    let mut count = 0;

    match files {
        Ok(files) => {
            for file in files {
                match file {
                    Ok(file) => {
                        let path = file.path().display().to_string();
                        if path.ends_with(".rasm") {
                            count = count + 1;
                            let result = exec_test(current_exe.clone(), path.clone());
                            match result {
                                Ok(result) => save_test(path, result),
                                Err(error) => printerr(error)
                            }
                        }
                    }
                    Err(_) => ()
                }
            }
        }
        Err(error) => {
            printerr(format!("failed to update tests on {} due to error\nlog:\n{}", folder, error));
        } 
    }

    println!("updated {} tests", count);
}

fn assemble(file: String, timing: bool) {
    let start = std::time::Instant::now();

    if !file.ends_with(".rasm") {
        eprintln!("{} expected `*.rasm` file, got `{}`", "ERROR:".red().bold().underline(), file);
        process::exit(1);
    }

    let file_path = (&file.as_str()[..file.len()-5]).to_string();

    let contents = fs::read_to_string(file_path.clone() + ".rasm").expect("failed to read file");

    let lines = contents.split("\n");

    let mut tokens: Vec<Vec<Token>> = Vec::new();
    let mut locations: Vec<Vec<Loc>> = Vec::new();
    let mut loc: Loc = Loc {file: file_path.clone() + ".rasm", line: 0, col: 0};
    for line in lines {
        loc.line = loc.line + 1;
        loc.col = line.chars().take_while(|ch| ch.is_whitespace() && *ch != '\n').count() + 1;
        let line_tokens = tokenize(line.to_string(), &mut loc);
        match line_tokens {
            (toks, locs) => {
                if *DEBUG.lock().unwrap() >= 2 {
                    println!("{:?}", toks);
                    println!("{:?}", locs);
                }
                tokens.push(toks);
                locations.push(locs);
            }
        }
    }

    let rbb_file = file_path.clone().to_string() + ".rbb";

    let bytes = parse(tokens, locations);

    if timing {
        println!("assembly took {:.2}ms", start.elapsed().as_secs_f32() * 1000f32);
    }

    if Path::new(&rbb_file).exists() {
       fs::remove_file(rbb_file.clone()).unwrap();
    }

    let mut file = fs::OpenOptions::new().create_new(true).write(true).open(rbb_file).expect("failed to open file to save");
    let _ = file.write_all(&bytes); 
}