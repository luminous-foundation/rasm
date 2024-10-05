use ::rainbow_wrapper::rainbow_wrapper::types::Value;
use ::rainbow_wrapper::*;

use crate::instruction::Instruction;

#[derive(Debug)]
pub enum Expr {
    INSTR(Instruction, Vec<Value>)
}

impl Expr {
    // TODO: arg length checking
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Expr::INSTR(instruction, args) => {
                match instruction { // The Match Statement
                    Instruction::NOP => vec![0x00],

                    Instruction::PUSH => push!(args[0].clone()),
                    Instruction::POP => pop!(args[0].clone()),
                    Instruction::PEEK => peek!(args[0].clone(), args[1].clone()),
                    
                    Instruction::CALL => call!(args[0].clone()),
                    
                    Instruction::ADD => add!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::SUB => sub!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::MUL => mul!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::DIV => div!(args[0].clone(), args[1].clone(), args[2].clone()),
                    
                    Instruction::JMP => jmp!(args[0].clone()),
                    Instruction::JNE => jne!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JE => je!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JGE => jge!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JG => jg!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JLE => jle!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::JL => jl!(args[0].clone(), args[1].clone(), args[2].clone()),
                    
                    Instruction::MOV => mov!(args[1].clone(), args[2].clone()),
                    
                    Instruction::AND => and!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::OR => or!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::XOR => xor!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::NOT => not!(args[0].clone(), args[2].clone()),
                    Instruction::LSH => lsh!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::RSH => rsh!(args[0].clone(), args[1].clone(), args[2].clone()),
                    
                    Instruction::VAR => var!(args[0].clone(), args[1].clone()),
                    
                    Instruction::RET => {
                        match args.len() {
                            0 => ret!(),
                            1 => ret!(args[0].clone()),
                            _ => panic!("too many arguments passed to `ret`")
                        }
                    }
                    
                    Instruction::DEREF => deref!(args[0].clone(), args[1].clone()),
                    Instruction::REF => r#ref!(args[0].clone(), args[1].clone()),
                    
                    Instruction::INST => inst!(args[0].clone(), args[1].clone()),
                    
                    Instruction::MOD => r#mod!(args[0].clone(), args[1].clone(), args[2].clone()),
                    
                    Instruction::PMOV => pmov!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::ALLOC => alloc!(args[0].clone(), args[1].clone(), args[2].clone()),
                    Instruction::FREE => {
                        match args.len() {
                            1 => free!(args[0].clone()),
                            2 => free!(args[0].clone(), args[1].clone()),
                            _ => panic!("too many arguments passed to `free`")
                        }
                    }
                    
                    Instruction::CALLC => callc!(args[0].clone(), args[1].clone(), args[2].clone()),
                }
            }
        }
    }
}