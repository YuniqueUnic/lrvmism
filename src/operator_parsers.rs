use nom::{
    branch::alt,
    character::complete::{char, line_ending, multispace0},
    combinator::{eof, map},
    error::context,
    sequence::{preceded, terminated},
    IResult,
};

use crate::token::Token;

pub fn operator(input: &str) -> IResult<&str, Token> {
    context(
        "operator",
        alt((
            addition_operator,
            substraction_operator,
            multiplication_operator,
            division_operator,
        )),
    )(input)
}

fn addition_operator(input: &str) -> IResult<&str, Token> {
    context(
        "addition_operator",
        preceded(
            multispace0,
            terminated(
                map(char('+'), |_| Token::AdditionOperator),
                alt((multispace0, eof, line_ending)),
            ),
        ),
    )(input)
}

fn substraction_operator(input: &str) -> IResult<&str, Token> {
    context(
        "substraction_operator",
        preceded(
            multispace0,
            terminated(
                map(char('-'), |_| Token::SubtractionOperator),
                alt((multispace0, eof, line_ending)),
            ),
        ),
    )(input)
}

fn multiplication_operator(input: &str) -> IResult<&str, Token> {
    context(
        "multiplication_operator",
        preceded(
            multispace0,
            terminated(
                map(char('*'), |_| Token::MultiplicationOperator),
                alt((multispace0, eof, line_ending)),
            ),
        ),
    )(input)
}

fn division_operator(input: &str) -> IResult<&str, Token> {
    context(
        "division_operator",
        preceded(
            multispace0,
            terminated(
                map(char('/'), |_| Token::DivisionOperator),
                alt((multispace0, eof, line_ending)),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_operator() {
        let test_arr = vec!["  +   ", "  - ", " * ", " / "];
        // let expect: Vec<usize> = vec![0, 1, 2, 3]; // +, -, *, /
        for (i, input) in test_arr.iter().enumerate() {
            let result = operator(input);
            assert_eq!(result.is_ok(), true);

            let (r, t) = result.unwrap();
            assert!(r.is_empty());
            assert_eq!(i, t.into());
        }
    }
}
