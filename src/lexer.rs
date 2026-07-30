use crate::{
    errors::SourceCodeError,
    token::{Token, TokenType},
};
use std::collections::HashMap;

pub struct Lexer {
    chars: Vec<char>,
    current_index: usize,
    tokens: Vec<Token>,

    current_token_start: usize,
    current_token_source: Vec<char>,
}

impl Lexer {
    fn new(source_code: String) -> Self {
        let chars: Vec<char> = source_code.to_lowercase().chars().collect();
        let current_index: usize = 0;
        let tokens: Vec<Token> = Vec::new();
        let current_token_start: usize = 0;
        let current_token_source: Vec<char> = Vec::new();
        Self {
            chars,
            current_index,
            tokens,
            current_token_start,
            current_token_source,
        }
    }

    fn has_next(&self) -> bool {
        self.current_index < self.chars.len()
    }

    fn peek(&self) -> Result<char, String> {
        if self.has_next() {
            return Ok(self.chars[self.current_index]);
        }
        Err("Cannot peek out of bounds".to_string())
    }

    fn peek_equals(&self, other: String) -> Result<bool, String> {
        for (i, c) in other.chars().enumerate() {
            if self.current_index + i >= self.chars.len() {
                return Err("Cannot peek out of bounds".to_string());
            }
            if self.chars[self.current_index + i] != c {
                return Ok(false);
            }
        }
        return Ok(true);
    }

    fn capture(&mut self) {
        self.current_index += 1;
        self.current_token_source
            .push(self.chars[self.current_index - 1]);
    }

    fn capture_n(&mut self, n: usize) {
        for _ in 0..n {
            self.capture();
        }
    }

    fn skip(&mut self, step: usize) {
        self.current_index += step;
        self.current_token_start = self.current_index
    }

    fn save_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            self.current_token_source.iter().collect(),
            self.current_token_start,
            self.current_index,
        ));
        self.current_token_start = self.current_index;
        self.current_token_source.clear();
    }

    pub fn lex(source_code: &String) -> Result<Vec<Token>, SourceCodeError> {
        let static_tokens: HashMap<String, TokenType> = HashMap::from([
            // Parentheses
            ("(".to_string(), TokenType::OpeningParenthesis),
            (")".to_string(), TokenType::ClosingParenthesis),
            ("[".to_string(), TokenType::OpeningSquareBracket),
            ("]".to_string(), TokenType::ClosingSquareBracket),
            ("#".to_string(), TokenType::PoundSign),
            (",".to_string(), TokenType::Comma),
            // Keywords
            ("+".to_string(), TokenType::Addition),
            ("-".to_string(), TokenType::Subtraction), // This doubles as negation
            ("*".to_string(), TokenType::Multiplication),
            ("/".to_string(), TokenType::Division),
            ("%".to_string(), TokenType::Modulo),
            ("**".to_string(), TokenType::Exponentiation),
            ("&&".to_string(), TokenType::And),
            ("||".to_string(), TokenType::Or),
            ("!".to_string(), TokenType::Not),
            ("&".to_string(), TokenType::BitwiseAnd),
            ("|".to_string(), TokenType::BitwiseOr),
            ("~".to_string(), TokenType::BitwiseNot),
            ("^".to_string(), TokenType::BitwiseXor),
            ("<<".to_string(), TokenType::BitwiseLeftShift),
            (">>".to_string(), TokenType::BitwiseRightShift),
            ("==".to_string(), TokenType::Equals),
            ("!=".to_string(), TokenType::NotEquals),
            ("<".to_string(), TokenType::LessThan),
            ("<=".to_string(), TokenType::LessThanOrEquals),
            (">".to_string(), TokenType::GreaterThan),
            (">=".to_string(), TokenType::GreaterThanOrEquals),
            ("int".to_string(), TokenType::FloatToInt),
            ("float".to_string(), TokenType::IntToFloat),
            ("max".to_string(), TokenType::Max),
            ("min".to_string(), TokenType::Min),
            ("mean".to_string(), TokenType::Mean),
            ("sum".to_string(), TokenType::Sum),
            // Statement stuff
            ("{".to_string(), TokenType::StartBlock),
            ("}".to_string(), TokenType::EndBlock),
            (";".to_string(), TokenType::EndOfLine),
            ("=".to_string(), TokenType::Assignment),
            ("if".to_string(), TokenType::If),
            ("else".to_string(), TokenType::Else),
            ("for".to_string(), TokenType::For),
            ("in".to_string(), TokenType::In),
            ("to".to_string(), TokenType::Range),
            ("return".to_string(), TokenType::Return),
            // Edge case for boolean primatives
            ("true".to_string(), TokenType::Boolean),
            ("false".to_string(), TokenType::Boolean),
        ]);

        let mut lexer = Lexer::new(source_code.clone());

        while lexer.has_next() {
            // The only thing whitespace does is break apart tokens.
            if lexer.peek().unwrap().is_whitespace() {
                lexer.skip(1);
                continue;
            }

            // Check for static token keywords. This means tokens
            // that are the same all the time (ie, +, +=).
            // Not things like primitives which vary.
            let mut keywords: Vec<&String> = static_tokens.keys().collect();

            // Sort or else it could jump the gun and just get something like + instead of +=
            keywords.sort_by(|a, b| b.len().cmp(&a.len()));

            let mut found_token = false;
            for key in keywords {
                // This does a big check and capture in one shot
                if lexer.peek_equals(key.to_string()).unwrap_or(false) {
                    lexer.capture_n(key.len());
                    let token_type = static_tokens.get(key).unwrap().clone();
                    lexer.save_token(token_type);
                    found_token = true;
                    break;
                }
            }
            if found_token {
                continue;
            }

            // Capture primatives (These are more dynamic).

            // Numeric primitives
            if lexer.peek().unwrap().is_digit(10) {
                while lexer.peek().is_ok() && lexer.peek().unwrap().is_digit(10) {
                    lexer.capture();
                }
                if lexer.peek().is_ok() && lexer.peek().unwrap() == '.' {
                    lexer.capture();
                    while lexer.peek().is_ok() && lexer.peek().unwrap().is_digit(10) {
                        lexer.capture();
                    }
                    lexer.save_token(TokenType::Float);
                    continue;
                }
                lexer.save_token(TokenType::Integer);
                continue;
            }

            // String literals
            if lexer.peek().unwrap() == '\"' {
                lexer.capture();
                while lexer.peek().is_ok() {
                    if lexer.peek().unwrap() == '\"' {
                        break;
                    }
                    lexer.capture();
                }
                if lexer.peek().unwrap() == '\"' {
                    lexer.capture();
                    lexer.save_token(TokenType::String);
                } else {
                    return Err(SourceCodeError {
                        location: vec![lexer.current_index],
                        error_message: String::from("Unknown symbol"),
                    });
                }
                continue;
            }

            // Variable
            if lexer.peek().unwrap().is_alphabetic() {
                lexer.capture();
                while lexer.peek().is_ok() {
                    if !lexer.peek().unwrap().is_alphanumeric() {
                        break;
                    }
                    lexer.capture();
                }
                lexer.save_token(TokenType::Variable);
                continue;
            }

            // Default to capturing any garbage
            return Err(SourceCodeError {
                location: vec![lexer.current_index],
                error_message: String::from("Unknown symbol"),
            });
        }
        Ok(lexer.tokens)
    }
}
