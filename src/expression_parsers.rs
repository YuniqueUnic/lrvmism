use nom::{
    branch::alt,
    character::complete::{line_ending, multispace0},
    combinator::{eof, map},
    error::context,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::{operand_parsers::integer, operator_parsers::operator, token::Token};

pub fn expression_parser(input: &str) -> IResult<&str, Token> {
    context(
        "expression_parser",
        preceded(
            multispace0,
            terminated(
                map(tuple((integer, operator, integer)), |(left, op, right)| {
                    Token::Expression {
                        left: Box::new(left),
                        op: Box::new(op),
                        right: Box::new(right),
                    }
                }),
                alt((multispace0, line_ending, eof)),
            ),
        ),
    )(input)
}
