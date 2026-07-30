use std::default::Default;

#[derive(Clone, Default, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub source: String,
    pub start_index: usize,
    pub end_index: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        source: String,
        start_index: usize,
        end_index: usize,
    ) -> Self {
        Self {
            token_type,
            source,
            start_index,
            end_index,
        }
    }

    pub fn merge(&self, other: Token) -> Token {
        if self.start_index < other.start_index {
            return Token {
                token_type: TokenType::Many,
                source: format!("{} {}", self.source, other.source),
                start_index: self.start_index,
                end_index: other.end_index,
            };
        }
        return Token {
            token_type: TokenType::Many,
            source: format!("{} {}", other.source, self.source),
            start_index: other.start_index,
            end_index: self.end_index,
        };
    }

    pub fn capture(self) -> Token {
        return Token {
            token_type: TokenType::Many,
            source: self.source.clone(),
            start_index: self.start_index,
            end_index: self.end_index,
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Unknown,
    Many,

    OpeningParenthesis,
    ClosingParenthesis,
    OpeningSquareBracket,
    ClosingSquareBracket,
    PoundSign,
    Comma,

    Number,

    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponentiation,
    Negation,

    Max,
    Min,
    Mean,
    Sum,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Unknown
    }
}
