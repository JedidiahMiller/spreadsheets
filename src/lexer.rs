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
            ("max".to_string(), TokenType::Max),
            ("min".to_string(), TokenType::Min),
            ("mean".to_string(), TokenType::Mean),
            ("sum".to_string(), TokenType::Sum),
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
                }
                lexer.save_token(TokenType::Number);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn types(source: &str) -> Vec<TokenType> {
        Lexer::lex(&source.to_string())
            .unwrap()
            .into_iter()
            .map(|t| t.token_type)
            .collect()
    }

    #[test]
    fn lexes_integer() {
        let tokens = Lexer::lex(&"123".to_string()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].source, "123");
    }

    #[test]
    fn lexes_decimal() {
        let tokens = Lexer::lex(&"3.14".to_string()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].source, "3.14");
    }

    #[test]
    fn lexes_symbols() {
        assert_eq!(
            types("()[]#,"),
            vec![
                TokenType::OpeningParenthesis,
                TokenType::ClosingParenthesis,
                TokenType::OpeningSquareBracket,
                TokenType::ClosingSquareBracket,
                TokenType::PoundSign,
                TokenType::Comma,
            ]
        );
    }

    #[test]
    fn lexes_single_char_operators() {
        assert_eq!(
            types("+-/%"),
            vec![
                TokenType::Addition,
                TokenType::Subtraction,
                TokenType::Division,
                TokenType::Modulo,
            ]
        );
    }

    #[test]
    fn lexes_multiplication_as_single_star() {
        assert_eq!(
            types("2*3"),
            vec![
                TokenType::Number,
                TokenType::Multiplication,
                TokenType::Number
            ]
        );
    }

    #[test]
    fn lexes_double_star_as_exponentiation_not_two_multiplications() {
        let tokens = Lexer::lex(&"2**3".to_string()).unwrap();
        assert_eq!(
            tokens
                .iter()
                .map(|t| t.token_type.clone())
                .collect::<Vec<_>>(),
            vec![
                TokenType::Number,
                TokenType::Exponentiation,
                TokenType::Number
            ]
        );
        assert_eq!(tokens[1].source, "**");
    }

    #[test]
    fn lexes_keywords_case_insensitively() {
        assert_eq!(types("MAX"), vec![TokenType::Max]);
        assert_eq!(types("Min"), vec![TokenType::Min]);
        assert_eq!(types("mean"), vec![TokenType::Mean]);
        assert_eq!(types("SUM"), vec![TokenType::Sum]);
    }

    #[test]
    fn skips_whitespace_between_tokens() {
        assert_eq!(
            types("1   +\t2\n"),
            vec![TokenType::Number, TokenType::Addition, TokenType::Number]
        );
    }

    #[test]
    fn tracks_token_source_positions() {
        let tokens = Lexer::lex(&"12+3".to_string()).unwrap();
        assert_eq!((tokens[0].start_index, tokens[0].end_index), (0, 2));
        assert_eq!((tokens[1].start_index, tokens[1].end_index), (2, 3));
        assert_eq!((tokens[2].start_index, tokens[2].end_index), (3, 4));
    }

    #[test]
    fn errors_on_unknown_symbol() {
        let result = Lexer::lex(&"1 @ 2".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn empty_source_produces_no_tokens() {
        let tokens = Lexer::lex(&"".to_string()).unwrap();
        assert!(tokens.is_empty());
    }
}
