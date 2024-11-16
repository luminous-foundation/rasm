use r#extern::Extern;
use generation::{generate_extern, generate_module, generate_scope, generate_struct};
use rainbow_wrapper::generation::{generate_function, Arg, generate_import};
use rainbow_wrapper::*;

use crate::instruction::Instruction;
use crate::parser::to_rb_type;
use crate::r#struct::Struct;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Expr {
    INSTR(Instruction, Vec<Value>),
    FUNCDEF(String, Vec<Arg>, Vec<Type>, Vec<Expr>),
    IF_BLOCK(String, String, String, Vec<Expr>),
    ELSEIF_BLOCK(String, String, String, Vec<Expr>),
    ELSE_BLOCK(Vec<Expr>),
    END_BLOCK,
    SCOPE(Vec<Expr>),
    IMPORT(String),
    MODULE(String, Vec<Expr>),
    EXTERN(Extern),
    STRUCT(Struct),
}

impl Expr {
    // TODO: arg length checking
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Expr::INSTR(instruction, args) => {
                match instruction { // The Match Statement
                    Instruction::NOP   => vec![0x00],

                    Instruction::PUSH  => push! (args[0].clone()),
                    Instruction::POP   => pop!  (args[0].clone()),
                    Instruction::PEEK  => peek! (args[0].clone(), args[1].clone()),
                    
                    Instruction::CALL  => call! (args[0].clone()),
                    
                    Instruction::ADD   => add!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::SUB   => sub!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::MUL   => mul!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::DIV   => div!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    
                    Instruction::JMP   => jmp!  (args[0].clone()),
                    Instruction::JNE   => jne!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JE    => je!   (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JGE   => jge!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JG    => jg!   (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JLE   => jle!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JL    => jl!   (args[0].clone(), args[1].clone(), args[2].clone()),
                    
                    Instruction::MOV   => mov!  (args[0].clone(), args[1].clone()),
                    
                    Instruction::AND   => and!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::OR    => or!   (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::XOR   => xor!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::NOT   => not!  (args[0].clone(), args[2].clone()),
                    Instruction::LSH   => lsh!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::RSH   => rsh!  (args[0].clone(), args[1].clone(), args[2].clone()),
                    
                    Instruction::VAR   => var!  (args[0].clone(), args[1].clone()),
                    
                    Instruction::RET   => {
                        match args.len() {
                            0 => ret!(),
                            1 => ret!(args[0].clone()),
                            _ => panic!("too many arguments passed to `ret`")
                        }
                    }
                    
                    Instruction::DEREF => rainbow_wrapper::deref!(args[0].clone(), args[1].clone()),
                    Instruction::REF   => r#ref!(args[0].clone(), args[1].clone()),
                    
                    Instruction::INST  => inst! (args[0].clone(), args[1].clone()),
                    
                    Instruction::MOD   => r#mod!(args[0].clone(), args[1].clone(), args[2].clone()),
                    
                    Instruction::PMOV  => pmov! (args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::ALLOC => alloc!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::FREE  => {
                        match args.len() {
                            1 => free!(args[0].clone()),
                            2 => free!(args[0].clone(), args[1].clone()),
                            _ => panic!("too many arguments passed to `free`")
                        }
                    }
                    
                    Instruction::CALLC => callc!(args[0].clone(), args[1].clone(), args[2].clone()),
                    
                    Instruction::CMP => {
                        let cond = match &args[0] {
                            Value::NAME(n) => {
                                match n.as_str() {
                                    "==" => 0,
                                    "!=" => 1,
                                    ">=" => 2,
                                    ">" => 3,
                                    "<=" => 4,
                                    "<" => 5,
                                    _ => panic!("invalid condition {n} passed to `cmp`")
                                }
                            },
                            _ => panic!("unexpected `{}` in `cmp` args", args[0])
                        };

                        cmp!(Value::UNSIGNED(cond), args[1].clone(), args[2].clone(), args[3].clone())
                    }
                }
            }
            Expr::FUNCDEF(name, args, return_type, body) => {
                let mut body_bytes: Vec<u8> = Vec::new();

                for expr in body {
                    body_bytes.append(&mut expr.to_bytes());
                }

                generate_function(name, args, return_type, &body_bytes)
            }
            Expr::IF_BLOCK(left, cond, right, body) => {
                let mut body_bytes: Vec<u8> = Vec::new();

                for expr in body {
                    body_bytes.append(&mut expr.to_bytes());
                }

                if_block!(left, cond, right, body_bytes)
            }
            Expr::ELSEIF_BLOCK(left, cond, right, body) => {
                let mut body_bytes: Vec<u8> = Vec::new();

                for expr in body {
                    body_bytes.append(&mut expr.to_bytes());
                }

                elseif_block!(left, cond, right, body_bytes)
            }
            Expr::ELSE_BLOCK(body) => {
                let mut body_bytes: Vec<u8> = Vec::new();

                for expr in body {
                    body_bytes.append(&mut expr.to_bytes());
                }

                else_block!(body_bytes)
            }
            Expr::END_BLOCK => {
                end_block!()
            }
            Expr::SCOPE(body) => {
                let mut body_bytes: Vec<u8> = Vec::new();

                for expr in body {
                    body_bytes.append(&mut expr.to_bytes());
                }

                generate_scope(&body_bytes)
            }
            Expr::IMPORT(import) => {
                generate_import(import)
            }
            Expr::MODULE(name, body) => {
                let mut body_bytes: Vec<u8> = Vec::new();

                for expr in body {
                    body_bytes.append(&mut expr.to_bytes());
                }

                generate_module(name, &body_bytes)
            }
            Expr::EXTERN(ext) => {
                generate_extern(ext)
            }
            Expr::STRUCT(strct) => {
                let mut types: Vec<Vec<Type>> = Vec::new();

                for typ in &strct.types {
                    types.push(to_rb_type(typ.to_vec()));
                }

                let wrap_struct: rainbow_wrapper::r#struct::Struct = rainbow_wrapper::r#struct::Struct { name: strct.name.clone(), types, names: strct.names.clone() };

                generate_struct(wrap_struct)
            }
        }
    }
}