use nom::{
    branch::alt,
    character::complete::{line_ending, multispace0},
    combinator::{eof, map},
    error::context,
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

use crate::{expression_parsers::expression_parser, token::Token};

pub fn program_parser(input: &str) -> IResult<&str, Token> {
    context(
        "program_parser",
        preceded(
            multispace0,
            terminated(
                map(many1(expression_parser), |expressions| Token::Program {
                    expressions,
                }),
                alt((multispace0, line_ending, eof)),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::program_parser;
    #[test]
    fn test_parse_program() {
        let test_program = "1+2";

        let result = program_parser(test_program);
        assert_eq!(result.is_ok(), true);

        let (r, program) = result.unwrap();
        let expressions = vec![Token::Expression {
            left: Box::new(Token::Term {
                left: Box::new(Token::Factor {
                    value: Box::new(Token::Integer { value: 1 }),
                }),
                right: vec![],
            }),
            right: vec![(
                Token::AdditionOperator,
                Token::Term {
                    left: Box::new(Token::Factor {
                        value: Box::new(Token::Integer { value: 2 }),
                    }),
                    right: vec![],
                },
            )],
        }];
        assert!(r.is_empty());
        assert_eq!(Token::Program { expressions }, program);
    }

    #[test]
    fn test_parse_program_2() {
        let test_program = "3*4";

        let result = program_parser(test_program);
        assert_eq!(result.is_ok(), true);

        let (r, program) = result.unwrap();
        let expressions = vec![Token::Expression {
            left: Box::new(Token::Term {
                left: Box::new(Token::Factor {
                    value: Box::new(Token::Integer { value: 3 }),
                }),
                right: vec![(
                    Token::MultiplicationOperator,
                    Token::Factor {
                        value: Box::new(Token::Integer { value: 4 }),
                    },
                )],
            }),
            right: vec![],
        }];
        assert!(r.is_empty());
        assert_eq!(Token::Program { expressions }, program);
    }
}
