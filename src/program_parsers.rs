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
            left: Box::new(Token::Integer { value: 1 }),
            op: Box::new(Token::AdditionOperator),
            right: Box::new(Token::Integer { value: 2 }),
        }];
        assert!(r.is_empty());
        assert_eq!(Token::Program { expressions }, program);
    }
}
