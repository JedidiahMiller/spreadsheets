use std::{usize, vec};

use crate::column::column_letters_to_index;
use crate::errors::SourceCodeError;
use crate::expressions::Expression;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    fn consume(&mut self) -> Result<Token, SourceCodeError> {
        if self.cursor >= self.tokens.len() {
            return Err(SourceCodeError {
                location: vec![self.tokens[self.cursor - 1].end_index],
                error_message: String::from("Expected to find a token starting here"),
            });
        }
        self.cursor += 1;
        Ok(self.tokens[self.cursor - 1].clone())
    }

    fn accept(&mut self, token: TokenType) -> Result<bool, SourceCodeError> {
        if self.has(token) {
            self.consume()?;
            return Ok(true);
        }
        Ok(false)
    }

    fn expect(&mut self, token: TokenType) -> Result<Token, SourceCodeError> {
        if self.has(token.clone()) {
            return Ok(self.consume()?);
        }
        Err(SourceCodeError {
            location: vec![self.tokens[self.cursor - 1].end_index],
            error_message: format!("Expected to find {:?}", token),
        })
    }

    fn has(&mut self, token: TokenType) -> bool {
        if self.cursor >= self.tokens.len() {
            return false;
        }
        self.tokens[self.cursor].token_type == token
    }

    fn peek(&self) -> Result<Token, ()> {
        if self.cursor >= self.tokens.len() {
            return Err(());
        }
        Ok(self.tokens[self.cursor].clone())
    }

    pub fn parse_code(tokens: Vec<Token>) -> Result<Expression, SourceCodeError> {
        let mut parser = Parser { tokens, cursor: 0 };
        let expression = parser.expression_level_1()?;

        if let Ok(token) = parser.peek() {
            return Err(SourceCodeError {
                location: vec![token.start_index],
                error_message: String::from("Unexpected trailing input after expression"),
            });
        }

        Ok(expression)
    }

    fn expression_level_1(&mut self) -> Result<Expression, SourceCodeError> {
        let mut first = self.expression_level_2()?;

        while self.has(TokenType::Addition) || self.has(TokenType::Subtraction) {
            if self.has(TokenType::Addition) {
                first = Expression::Addition {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_2()?),
                }
            }
            if self.has(TokenType::Subtraction) {
                first = Expression::Subtraction {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_2()?),
                }
            }
        }
        Ok(first)
    }

    fn expression_level_2(&mut self) -> Result<Expression, SourceCodeError> {
        let mut first = self.expression_level_3()?;

        while self.has(TokenType::Multiplication)
            || self.has(TokenType::Division)
            || self.has(TokenType::Modulo)
        {
            if self.has(TokenType::Multiplication) {
                first = Expression::Multiplication {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_3()?),
                }
            }
            if self.has(TokenType::Division) {
                first = Expression::Division {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_3()?),
                }
            }
            if self.has(TokenType::Modulo) {
                first = Expression::Modulo {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_3()?),
                }
            }
        }
        Ok(first)
    }

    fn expression_level_3(&mut self) -> Result<Expression, SourceCodeError> {
        if self.accept(TokenType::Max)? {
            self.expect(TokenType::OpeningParenthesis)?;
            let first = self.expression_level_1()?;
            self.expect(TokenType::Comma)?;
            let second = self.expression_level_1()?;
            return Ok(Expression::Max {
                source_token: self.expect(TokenType::ClosingParenthesis)?,
                first_address: Box::new(first),
                last_address: Box::new(second),
            });
        }
        if self.accept(TokenType::Min)? {
            self.expect(TokenType::OpeningParenthesis)?;
            let first = self.expression_level_1()?;
            self.expect(TokenType::Comma)?;
            let second = self.expression_level_1()?;
            return Ok(Expression::Min {
                source_token: self.expect(TokenType::ClosingParenthesis)?,
                first_address: Box::new(first),
                last_address: Box::new(second),
            });
        }
        if self.accept(TokenType::Mean)? {
            self.expect(TokenType::OpeningParenthesis)?;
            let first = self.expression_level_1()?;
            self.expect(TokenType::Comma)?;
            let second = self.expression_level_1()?;
            return Ok(Expression::Mean {
                source_token: self.expect(TokenType::ClosingParenthesis)?,
                first_address: Box::new(first),
                last_address: Box::new(second),
            });
        }
        if self.accept(TokenType::Sum)? {
            self.expect(TokenType::OpeningParenthesis)?;
            let first = self.expression_level_1()?;
            self.expect(TokenType::Comma)?;
            let second = self.expression_level_1()?;
            return Ok(Expression::Sum {
                source_token: self.expect(TokenType::ClosingParenthesis)?,
                first_address: Box::new(first),
                last_address: Box::new(second),
            });
        }
        self.expression_level_4()
    }

    fn expression_level_4(&mut self) -> Result<Expression, SourceCodeError> {
        if self.has(TokenType::Negation) {
            return Ok(Expression::Negation {
                source_token: self.consume()?,
                expression: Box::new(self.expression_level_4()?),
            });
        }
        if self.has(TokenType::Subtraction) {
            return Ok(Expression::Negation {
                source_token: self.consume()?,
                expression: Box::new(self.expression_level_4()?),
            });
        }

        let first = self.expression_level_5()?;
        if self.has(TokenType::Exponentiation) {
            return Ok(Expression::Exponentiation {
                source_token: self.consume()?,
                left_expression: Box::new(first),
                right_expression: Box::new(self.expression_level_4()?),
            });
        }
        Ok(first)
    }

    fn expression_level_5(&mut self) -> Result<Expression, SourceCodeError> {
        if self.has(TokenType::Number) {
            let token = self.consume()?;
            // This should have been checked by the lexer
            let value: f64 = token.source.parse().unwrap();
            return Ok(Expression::Number {
                source_token: token,
                value,
            });
        }
        if self.accept(TokenType::PoundSign)? {
            let token = self.expect(TokenType::CellReference)?;
            return Ok(Expression::RValue {
                source_token: token.clone(),
                cell_address: Box::new(Self::parse_cell_reference(token)),
            });
        }
        if self.has(TokenType::CellReference) {
            let token = self.consume()?;
            return Ok(Expression::LValue {
                source_token: token.clone(),
                cell_address: Box::new(Self::parse_cell_reference(token)),
            });
        }

        if self.accept(TokenType::OpeningParenthesis)? {
            let expression = self.expression_level_1()?;
            self.expect(TokenType::ClosingParenthesis)?;
            return Ok(expression);
        }

        if self.peek().is_ok() && self.peek().unwrap().token_type == TokenType::Unknown {
            panic!("Unknown token type should be depricated");
        }
        Err(SourceCodeError {
            location: vec![],
            error_message: String::from("Incomplete expression"),
        })
    }

    // The lexer guarantees a CellReference token's source is column letters
    // followed by a row number, ie "ab12".
    fn parse_cell_reference(token: Token) -> Expression {
        let split_at = token
            .source
            .find(|c: char| c.is_ascii_digit())
            .expect("Lexer should guarantee a row number follows column letters");
        let (letters, digits) = token.source.split_at(split_at);

        let column = column_letters_to_index(letters)
            .expect("Lexer should guarantee only letters precede the row number");
        let row_number: i32 = digits
            .parse()
            .expect("Lexer should guarantee a valid row number");

        Expression::CellAddress {
            source_token: token.clone(),
            x_value: Box::new(Expression::Number {
                source_token: token.clone(),
                value: column as f64,
            }),
            y_value: Box::new(Expression::Number {
                source_token: token,
                value: (row_number - 1) as f64,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(source: &str) -> Result<Expression, SourceCodeError> {
        let tokens = Lexer::lex(&source.to_string()).unwrap();
        Parser::parse_code(tokens)
    }

    fn serialized(source: &str) -> String {
        parse(source).unwrap().serialize().unwrap()
    }

    #[test]
    fn parses_number() {
        assert_eq!(serialized("42"), "42");
    }

    #[test]
    fn parses_addition_and_subtraction_left_associatively() {
        assert_eq!(serialized("10-3-2"), "((10 - 3) - 2)");
    }

    #[test]
    fn multiplication_binds_tighter_than_addition() {
        assert_eq!(serialized("2+3*4"), "(2 + (3 * 4))");
    }

    #[test]
    fn exponentiation_is_right_associative() {
        assert_eq!(serialized("2**3**2"), "(2 ^ (3 ^ 2))");
    }

    #[test]
    fn exponentiation_binds_tighter_than_multiplication() {
        assert_eq!(serialized("2*3**2"), "(2 * (3 ^ 2))");
    }

    #[test]
    fn parentheses_override_precedence() {
        assert_eq!(serialized("(2+3)*4"), "((2 + 3) * 4)");
    }

    #[test]
    fn parses_unary_negation() {
        assert_eq!(serialized("-5"), "-5");
    }

    #[test]
    fn negation_wraps_the_whole_exponent_expression() {
        assert_eq!(serialized("-2**2"), "-(2 ^ 2)");
    }

    #[test]
    fn parses_lvalue_cell_address() {
        assert_eq!(serialized("B3"), "B3");
    }

    #[test]
    fn parses_multi_letter_column_cell_address() {
        assert_eq!(serialized("AB12"), "AB12");
    }

    #[test]
    fn parses_rvalue_cell_address() {
        assert_eq!(serialized("#B3"), "#B3");
    }

    #[test]
    fn parses_aggregate_functions() {
        assert_eq!(serialized("sum(A1,B2)"), "Sum(A1, B2)");
        assert_eq!(serialized("max(A1,B2)"), "Max(A1, B2)");
        assert_eq!(serialized("min(A1,B2)"), "Min(A1, B2)");
        assert_eq!(serialized("mean(A1,B2)"), "Mean(A1, B2)");
    }

    #[test]
    fn errors_on_empty_input() {
        assert!(parse("").is_err());
    }

    #[test]
    fn errors_on_trailing_input() {
        assert!(parse("1 2").is_err());
    }

    #[test]
    fn errors_on_unclosed_parenthesis() {
        assert!(parse("(1+2").is_err());
    }

    #[test]
    fn errors_on_missing_comma_in_function_call() {
        assert!(parse("sum(A1 B2)").is_err());
    }
}
