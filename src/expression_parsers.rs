use nom::{
    branch::alt,
    character::complete::{line_ending, multispace0},
    combinator::{eof, map},
    error::context,
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::{
    operator_parsers::{addition_operator, substraction_operator},
    term_parsers::term_parser,
    token::Token,
};

pub fn expression_parser(input: &str) -> IResult<&str, Token> {
    context(
        "expression_parser",
        preceded(
            multispace0,
            terminated(
                map(
                    tuple((
                        term_parser,
                        many0(tuple((
                            alt((addition_operator, substraction_operator)),
                            term_parser,
                        ))),
                    )),
                    |(left, right)| Token::Expression {
                        left: Box::new(left),
                        right,
                    },
                ),
                alt((multispace0, line_ending, eof)),
            ),
        ),
    )(input)
}
