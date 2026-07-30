use std::{usize, vec};

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
            self.expect(TokenType::OpeningSquareBracket)?;
            let x_val = self.expression_level_1()?;
            self.expect(TokenType::Comma)?;
            let y_val = self.expression_level_1()?;

            let token = self.expect(TokenType::ClosingSquareBracket)?;
            let address = Expression::CellAddress {
                source_token: token.clone(),
                x_value: Box::new(x_val),
                y_value: Box::new(y_val),
            };

            return Ok(Expression::RValue {
                source_token: token,
                cell_address: Box::new(address),
            });
        }
        if self.accept(TokenType::OpeningSquareBracket)? {
            let x_val = self.expression_level_1()?;
            self.expect(TokenType::Comma)?;
            let y_val = self.expression_level_1()?;

            let token = self.expect(TokenType::ClosingSquareBracket)?;
            let address = Expression::CellAddress {
                source_token: token.clone(),
                x_value: Box::new(x_val),
                y_value: Box::new(y_val),
            };

            return Ok(Expression::LValue {
                source_token: token,
                cell_address: Box::new(address),
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
}
