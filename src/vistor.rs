use std::u8;

use crate::token::Token;
use lrvm::assembler::{prepend_header, Assembler};

pub trait Visitor {
    fn visit_token(&mut self, node: &Token);
}

#[derive(Default)]
/// Compiles the code into assembly. Also contains an Assembler so it can just
/// write out bytecode. It also handles register allocation.
pub struct Compiler {
    /// Unused Registers
    free_registers: Vec<u8>,
    /// Used Registers
    used_registers: Vec<u8>,
    /// The assembly statements created so far. These are just Strings that are
    /// emitted by the `Compiler` as it walks the tree
    assembly: Vec<String>,
    /// An `Assembler` for the lrvm VM, so the `Compiler` can emit bytecode directly
    assembler: Assembler,
}

impl Compiler {
    pub fn new() -> Self {
        let mut free_registers: Vec<u8> = (0..31).collect();
        free_registers.reverse();

        Compiler {
            free_registers,
            used_registers: vec![],
            assembly: vec![],
            assembler: Assembler::new(),
        }
    }

    pub fn prepend_two_section(&self, input: &str) -> String {
        let mut instructions = String::new();
        let data_section = input.contains(".data");
        let code_section = input.contains(".code");
        if !code_section {
            instructions.push_str(".code\n");
        }
        if !data_section {
            instructions.push_str(".data\n");
        }
        instructions.push_str(input);
        instructions
    }

    /// Takes a Vector of Strings that represent the text of a program and compiles
    /// it into bytecode
    pub fn compile(&mut self) -> Vec<u8> {
        let program = self.assembly.join("\n");
        let program = self.prepend_two_section(&program);
        let bytecode = self.assembler.assemble(&program);
        match bytecode {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Error on compile: {:#?}", e);
                vec![]
            }
        }
    }

    pub fn print_asm(&self) {
        for line in &self.assembly {
            println!("{:#?}", line);
        }
    }

    pub fn print_used_registers(&self) {
        println!("--------------------");
        println!("|  Used Registers  |");
        println!("--------------------");
        for r in &self.used_registers {
            println!("{:#?}", r);
        }
    }

    pub fn print_free_registers(&self) {
        println!("--------------------");
        println!("|  Free Registers  |");
        println!("--------------------");
        for r in &self.free_registers {
            println!("{:#?}", r);
        }
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            Token::AdditionOperator => {
                println!("Visiting AdditionOperator");
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!(
                    "ADD ${} ${} ${}",
                    right_register, left_register, result_register
                );
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            }
            Token::SubtractionOperator => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!(
                    "SUB ${} ${} ${}",
                    right_register, left_register, result_register
                );
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            }
            Token::MultiplicationOperator => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!(
                    "MUL ${} ${} ${}",
                    left_register, right_register, result_register
                );
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            }
            Token::DivisionOperator => {
                let result_register = self.free_registers.pop().unwrap();
                let left_register = self.used_registers.pop().unwrap();
                let right_register = self.used_registers.pop().unwrap();
                let line = format!(
                    "DIV ${} ${} ${}",
                    left_register, right_register, result_register
                );
                self.assembly.push(line);
                self.used_registers.push(result_register);
                self.free_registers.push(left_register);
                self.free_registers.push(right_register);
            }
            Token::Integer { value } => {
                println!("Visited Integer with value of: {:#?}", value);
                let next_register = self.free_registers.pop().unwrap();
                let line = format!("LOAD ${} #{}", next_register, value);
                self.used_registers.push(next_register);
                self.assembly.push(line);
            }
            Token::Expression {
                ref left,
                ref right,
            } => {
                println!("Visiting an expression");
                self.visit_token(left);
                for term in right {
                    self.visit_token(&term.1);
                    self.visit_token(&term.0);
                }
                println!("Done visiting expression");
            }
            Token::Program { ref expressions } => {
                println!("Visiting program");
                for expression in expressions {
                    self.visit_token(expression);
                }
                println!("Done visiting program");
            }
            Token::Float { value } => {
                let next_register = self.free_registers.pop().unwrap();
                let line = format!("LOAD ${} #{}", next_register, value);
                self.used_registers.push(next_register);
                self.assembly.push(line);
            }
            Token::Factor { value } => {
                self.visit_token(value);
            }
            Token::Term { left, right } => {
                self.visit_token(&left);
                for factor in right {
                    self.visit_token(&factor.1);
                    self.visit_token(&factor.0);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{program_parsers::program_parser, token::Token};

    use super::{Compiler, Visitor};

    fn generate_test_program(source: &str) -> Token {
        let (_, tree) = program_parser(source).unwrap();
        tree
    }

    #[test]
    fn test_visit_addition_token() {
        let source = "1+2";
        let mut compiler = Compiler::new();
        let test_program = generate_test_program(&source);
        compiler.visit_token(&test_program);
        let bytecode = compiler.compile();
        println!("({}) bytecodes: {:?}", source, bytecode);
    }

    #[test]
    fn test_nested_operators() {
        let source = "(4*3)-1";
        let mut compiler = Compiler::new();
        let test_program = generate_test_program(&source);
        compiler.visit_token(&test_program);
        let bytecode = compiler.compile();
        println!("({}) bytecodes: {:?}", source, bytecode);
    }
}
