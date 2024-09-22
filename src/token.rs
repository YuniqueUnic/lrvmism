#[derive(Debug, PartialEq)]
pub enum Token {
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    Integer {
        value: i64,
    },
    Experssion {
        left: Box<Token>,
        op: Box<Token>,
        right: Box<Token>,
    },
    Program {
        expressions: Vec<Token>,
    },
}

impl From<usize> for Token {
    fn from(value: usize) -> Self {
        match value {
            0 => Token::AdditionOperator,
            1 => Token::SubtractionOperator,
            2 => Token::MultiplicationOperator,
            3 => Token::DivisionOperator,
            4 => Token::Integer { value: 0 },
            5 => Token::Experssion {
                left: Box::new(Token::Integer { value: 0 }),
                op: Box::new(Token::AdditionOperator),
                right: Box::new(Token::Integer { value: 0 }),
            },
            6 => Token::Program {
                expressions: vec![],
            },
            _ => panic!("invalid token"),
        }
    }
}

impl Into<usize> for Token {
    fn into(self) -> usize {
        match self {
            Token::AdditionOperator => 0,
            Token::SubtractionOperator => 1,
            Token::MultiplicationOperator => 2,
            Token::DivisionOperator => 3,
            Token::Integer { value } => 4,
            Token::Experssion { left, op, right } => 5,
            Token::Program { expressions } => 6,
        }
    }
}
