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
    factors_parsers::factor_parser,
    operator_parsers::{division_operator, multiplication_operator},
    token::Token,
};

/// Looks for `Terms`. A `Term` consists of a `Factor` on the left,
/// and then an `Operator` and `Factor` on the right.
///
/// # Example
///
/// ```
/// (3*4)*2
/// ```
///
pub fn term_parser(input: &str) -> IResult<&str, Token> {
    context(
        "term_parser",
        preceded(
            multispace0,
            terminated(
                map(
                    tuple((
                        factor_parser,
                        many0(tuple((
                            alt((multiplication_operator, division_operator)),
                            factor_parser,
                        ))),
                    )),
                    |(left, right)| Token::Term {
                        left: Box::new(left),
                        right,
                    },
                ),
                alt((multispace0, eof, line_ending)),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{term_parsers::term_parser, token::Token};

    #[test]
    fn test_parse_term() {
        let result = term_parser("3*4");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_parse_nested_term() {
        let result = term_parser("(3*4)*2");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_parse_really_nested_term() {
        let result = term_parser("((3*4)*2)");
        assert_eq!(result.is_ok(), true);
    }
}
