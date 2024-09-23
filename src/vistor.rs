use std::u8;

use crate::token::Token;

pub trait Visitor {
    fn visit_token(&mut self, node: &Token);
}

pub struct Compiler {
    free_registers: Vec<u8>,
    used_registers: Vec<u8>,
    assembly: Vec<String>,
}

impl Compiler {
    pub fn new() -> Self {
        // let mut free_registers: Vec<u8> = (0..31).collect();
        // free_registers.reverse();
        let mut free_registers = vec![];
        for i in 0..31 {
            free_registers.push(i);
        }
        free_registers.reverse();

        Compiler {
            free_registers,
            used_registers: vec![],
            assembly: vec![],
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
                ref op,
                ref right,
            } => {
                println!("Visiting an expression");
                self.visit_token(left);
                self.visit_token(right);
                self.visit_token(op);
                println!("Done visiting expression");
            }
            Token::Program { ref expressions } => {
                println!("Visiting program");
                for expression in expressions {
                    self.visit_token(expression);
                }
                println!("Done visiting program");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{program_parsers::program_parser, token::Token};

    use super::{Compiler, Visitor};

    fn generate_test_program() -> Token {
        let source = "1+2";
        let (_, tree) = program_parser(source).unwrap();
        tree
    }

    #[test]
    fn test_visit_addition_token() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program();
        compiler.visit_token(&test_program);
    }
}
