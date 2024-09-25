#[derive(Debug, PartialEq)]
pub enum Token {
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    Integer {
        value: i64,
    },
    Float {
        value: f64,
    },
    Factor {
        value: Box<Token>,
    },
    Term {
        left: Box<Token>,
        right: Vec<(Token, Token)>,
    },
    Expression {
        left: Box<Token>,
        right: Vec<(Token, Token)>,
    },
    Program {
        expressions: Vec<Token>,
    },
}

impl From<usize> for Token {
    fn from(value: usize) -> Self {
        let default_add = Box::new(Token::AdditionOperator);
        let default_vec = vec![(Token::Integer { value: 0 }, Token::Integer { value: 0 })];
        match value {
            0 => Token::AdditionOperator,
            1 => Token::SubtractionOperator,
            2 => Token::MultiplicationOperator,
            3 => Token::DivisionOperator,
            4 => Token::Integer { value: 0 },
            5 => Token::Float { value: 0.0 },
            6 => Token::Factor { value: default_add },
            7 => Token::Term {
                left: default_add,
                right: default_vec,
            },
            8 => Token::Expression {
                left: Box::new(Token::Integer { value: 0 }),
                right: vec![(Token::Integer { value: 0 }, Token::Integer { value: 0 })],
            },
            9 => Token::Program {
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
            Token::Float { value } => 5,
            Token::Factor { value } => 6,
            Token::Term { left, right } => 7,
            Token::Expression { left, right } => 8,
            Token::Program { expressions } => 9,
        }
    }
}
