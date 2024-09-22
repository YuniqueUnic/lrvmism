use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending, multispace0},
    combinator::{eof, map_res, opt},
    error::context,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::token::Token;

pub fn integer(input: &str) -> IResult<&str, Token> {
    context(
        "integer",
        preceded(
            multispace0,
            terminated(
                map_res(
                    tuple((opt(char('-')), digit1)),
                    |(sign, num): (Option<char>, &str)| {
                        let value = match num.parse::<i64>() {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Unable to parse the value: {}\nError: {}", num, e);
                                return Err(num);
                            }
                        };

                        let value = if sign.is_some() { -value } else { value };

                        Ok::<Token, &str>(Token::Integer { value })
                    },
                ),
                alt((multispace0, eof, line_ending)),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {

    use crate::token::Token;

    use super::integer;

    #[test]
    fn test_parse_integer() {
        let test_array = vec!["0", "-1", "1", "-64", "64"];
        for input in test_array {
            let expect = Token::Integer {
                value: input.parse::<i64>().unwrap(),
            };

            let result = integer(input);
            assert!(result.is_ok());

            let (_reminder, value) = result.unwrap();
            assert_eq!(expect, value);
            assert!(_reminder.is_empty());
        }
    }
}
