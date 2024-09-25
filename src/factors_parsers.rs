use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending, multispace0},
    combinator::{eof, map, map_res, opt},
    error::context,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::{expression_parsers::expression_parser, token::Token};

/// Parser for a `Factor`. A Factor consists of an integer, float, identifier,
/// or a parenthized expression
///
/// # Example
///
/// ```
/// use lrvmism::factors_parsers::factor_parser;
/// use lrvmism::token::Token;
/// fn test_integer(){
///     let test = "(1+2)";
///     let expressions = vec![Token::Expression {
///        left: Box::new(Token::Term {
///            left: Box::new(Token::Factor {
///                value: Box::new(Token::Integer { value: 1 }),
///            }),
///            right: vec![],
///        }),
///        right: vec![(
///            Token::AdditionOperator,
///            Token::Term {
///                left: Box::new(Token::Factor {
///                    value: Box::new(Token::Integer { value: 2 }),
///                }),
///                right: vec![],
///            },
///        )],
///     }];
///     let result = factor_parser(test);
///     assert!(result.is_ok());
///     let (_reminder, program) = result.unwrap();
///     assert_eq!(Token::Program { expressions }, program);
///     assert!(_reminder.is_empty());
/// }
/// ```
///
pub fn factor_parser(input: &str) -> IResult<&str, Token> {
    context(
        "factor_parser",
        preceded(
            multispace0,
            terminated(
                map(
                    alt((
                        integer_parser,
                        float64_parser,
                        delimited(char('('), expression_parser, char(')')),
                    )),
                    |f| Token::Factor { value: Box::new(f) },
                ),
                alt((multispace0, eof, line_ending)),
            ),
        ),
    )(input)
}

/// Parser for a signed 64-bit integer.
///
/// # Example
///
/// ```
/// use lrvmism::factors_parsers::integer_parser;
/// use lrvmism::token::Token;
/// fn test_integer(){
///     let test_arr = ["4"," -4 "];
///     for input in test_arr {
///         let expect = Token::Integer {
///             value: input.parse::<i64>().unwrap(),
///         };
///         let result = integer_parser(input);
///         assert!(result.is_ok());
///         let (_reminder, value) = result.unwrap();
///         assert_eq!(expect, value);
///         assert!(_reminder.is_empty());
///     }
/// }
/// ```
pub fn integer_parser(input: &str) -> IResult<&str, Token> {
    context(
        "integer_parser",
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

/// Parser for a 64-bit float. A float can be negative, and must contain a `.`.
///
/// # Example
///
/// ```
/// use lrvmism::factors_parsers::integer_parser;
/// use lrvmism::token::Token;
/// fn test_integer(){
///     let test_arr = ["4.5"," -4.5 "];
///     for input in test_arr {
///         let expect = Token::Float {
///             value: input.parse::<f64>().unwrap(),
///         };
///         let result = integer_parser(input);
///         assert!(result.is_ok());
///         let (_reminder, value) = result.unwrap();
///         assert_eq!(expect, value);
///         assert!(_reminder.is_empty());
///     }
/// }
/// ```
pub fn float64_parser(input: &str) -> IResult<&str, Token> {
    context(
        "float64_parser",
        preceded(
            multispace0,
            terminated(
                map_res(
                    tuple((opt(char('-')), digit1, char('.'), digit1)),
                    |(sign, left, _dot, right)| {
                        let mut float_str = String::from(left);
                        float_str.push_str(".");
                        float_str.push_str(right);
                        let converted_float = match float_str.parse::<f64>() {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Unable to parse the value: {}\nError: {}", float_str, e);
                                return Err(float_str);
                            }
                        };

                        let value = if sign.is_some() {
                            -converted_float
                        } else {
                            converted_float
                        };
                        Ok(Token::Factor {
                            value: Box::new(Token::Float { value }),
                        })
                    },
                ),
                alt((multispace0, line_ending, eof)),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{factors_parsers::factor_parser, token::Token};

    use super::{float64_parser, integer_parser};

    #[test]
    fn test_parse_integer() {
        let test_array = vec!["0", "-1", "1", "-64", "64"];
        for input in test_array {
            let expect = Token::Integer {
                value: input.parse::<i64>().unwrap(),
            };

            let result = integer_parser(input);
            assert!(result.is_ok());

            let (_reminder, value) = result.unwrap();
            assert_eq!(expect, value);
            assert!(_reminder.is_empty());
        }
    }

    #[test]
    fn test_float64_parser() {
        let input = "1.2";
        let result = float64_parser(input);
        assert_eq!(result.is_ok(), true);

        let input_arr = ["1.0", " -1.0 ", "323.8", "1.453"];
        let expect_arr = [1.0, -1.0, 323.8, 1.453];
        for (i, test_str) in input_arr.iter().enumerate() {
            let result = float64_parser(test_str);
            assert_eq!(result.is_ok(), true);

            let (_reminder, value) = result.unwrap();
            let expect = Token::Factor {
                value: Box::new(Token::Float {
                    value: expect_arr[i],
                }),
            };

            assert!(_reminder.is_empty());
            assert_eq!(expect, value);
        }
    }

    #[test]
    fn test_factor() {
        let test_program = ("(1+2)");
        let result = factor_parser(test_program);
        assert_eq!(result.is_ok(), true);
        let (_, tree) = result.unwrap();
        println!("{:#?}", tree);
    }

    #[test]
    fn test_parse_floats() {
        let test_floats = vec!["100.4", "1.02", "-1.02"];
        for o in test_floats {
            let _parsed_o = o.parse::<f64>().unwrap();
            let result = float64_parser(o);
            assert_eq!(result.is_ok(), true);
        }
    }

    #[test]
    fn test_parse_integer_2() {
        let test_integers = vec!["0", "-1", "1"];
        for o in test_integers {
            let _parsed_o = o.parse::<i64>().unwrap();
            let result = integer_parser(o);
            assert_eq!(result.is_ok(), true);
        }
    }
}
