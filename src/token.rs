
use std::default::Default;

#[derive(Clone)]
#[derive(Default)]
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub source: String,
    pub start_index: usize,
    pub end_index: usize
}

impl Token {
    pub fn new(token_type: TokenType, source: String, start_index: usize, end_index: usize) -> Self {
        Self { token_type, source, start_index, end_index }
    }

    pub fn merge(&self, other: Token) -> Token {
        if self.start_index < other.start_index {
            return Token {
                token_type: TokenType::Many,
                source: format!("{} {}", self.source, other.source),
                start_index: self.start_index,
                end_index: other.end_index
            };
        }
        return Token {
            token_type: TokenType::Many,
            source: format!("{} {}", other.source, self.source),
            start_index: other.start_index,
            end_index: self.end_index
        };
    }

    pub fn capture(self) -> Token {
        return Token {
            token_type: TokenType::Many,
            source: self.source.clone(),
            start_index: self.start_index,
            end_index: self.end_index
        };
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum TokenType {

    Unknown,
    Many,

    OpeningParenthesis,
    ClosingParenthesis,
    OpeningSquareBracket,
    ClosingSquareBracket,
    PoundSign,
    Comma,

    Integer,
    Float,
    Boolean,
    String,
    Variable,

    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponentiation,
    Negation,

    And,
    Or,
    Not,

    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,

    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,

    FloatToInt,
    IntToFloat,

    Max,
    Min,
    Mean,
    Sum,

    StartBlock,
    EndBlock,

    Assignment,
    EndOfLine,
    If,
    Else,
    For,
    In,
    Range,
    Return,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Unknown
    }
}
