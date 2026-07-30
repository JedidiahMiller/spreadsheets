use std::{usize, vec};

use crate::errors::SourceCodeError;
use crate::expressions::Expression;
use crate::statements::Statement;
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

    pub fn parse_code(tokens: Vec<Token>) -> Result<Statement, SourceCodeError> {
        let mut parser = Parser { tokens, cursor: 0 };
        return parser.block_level_1();
    }

    fn block_level_1(&mut self) -> Result<Statement, SourceCodeError> {
        let mut lines = Vec::new();

        while self.peek().is_ok() {
            // Check for end of block
            if self.has(TokenType::EndBlock) {
                self.consume()?;
                break;
            }

            // Handle normal statement
            let line = self.block_level_2()?;
            self.expect(TokenType::EndOfLine)?;
            lines.push(line);
        }

        Ok(Statement::Block { statements: lines })
    }

    fn block_level_2(&mut self) -> Result<Statement, SourceCodeError> {
        if self.has(TokenType::Variable) {
            let name = self.consume()?.source;
            self.expect(TokenType::Assignment)?;
            let value = Box::new(self.expression_level_1()?);

            return Ok(Statement::Assignment {
                target: name,
                value,
            });
        }
        if self.accept(TokenType::Return)? {
            let expression = Box::new(self.expression_level_1()?);

            return Ok(Statement::Return { expression });
        }
        if self.accept(TokenType::If)? {
            let conditional = Box::new(self.expression_level_1()?);
            self.expect(TokenType::StartBlock)?;
            let true_block = Box::new(self.block_level_1()?);
            self.expect(TokenType::Else)?;
            self.expect(TokenType::StartBlock)?;
            let false_block = Box::new(self.block_level_1()?);

            return Ok(Statement::IfElse {
                conditional,
                true_block,
                false_block,
            });
        }
        if self.accept(TokenType::For)? {
            let variable = self.expect(TokenType::Variable)?.source;
            self.expect(TokenType::In)?;
            let range_start = Box::new(self.expression_level_1()?);
            self.expect(TokenType::Range)?;
            let range_end = Box::new(self.expression_level_1()?);
            self.expect(TokenType::StartBlock)?;
            let body = Box::new(self.block_level_1()?);

            return Ok(Statement::ForEach {
                variable,
                range_start,
                range_end,
                body,
            });
        }

        Err(SourceCodeError {
            location: vec![self.cursor],
            error_message: String::from("Did not find a statement"),
        })
    }

    fn expression_level_1(&mut self) -> Result<Expression, SourceCodeError> {
        let mut first = self.expression_level_2()?;

        while self.has(TokenType::Addition)
            || self.has(TokenType::Subtraction)
            || self.has(TokenType::And)
            || self.has(TokenType::Or)
        {
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
            if self.has(TokenType::And) {
                first = Expression::And {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_2()?),
                }
            }
            if self.has(TokenType::Or) {
                first = Expression::Or {
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
            || self.has(TokenType::Equals)
            || self.has(TokenType::NotEquals)
            || self.has(TokenType::LessThan)
            || self.has(TokenType::GreaterThan)
            || self.has(TokenType::LessThanOrEquals)
            || self.has(TokenType::GreaterThanOrEquals)
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
            if self.has(TokenType::Equals) {
                first = Expression::Equals {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_3()?),
                }
            }
            if self.has(TokenType::NotEquals) {
                first = Expression::NotEquals {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_3()?),
                }
            }
            if self.has(TokenType::LessThan) {
                first = Expression::LessThan {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_3()?),
                }
            }
            if self.has(TokenType::GreaterThan) {
                first = Expression::GreaterThan {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_3()?),
                }
            }
            if self.has(TokenType::LessThanOrEquals) {
                first = Expression::LessThanOrEquals {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_3()?),
                }
            }
            if self.has(TokenType::GreaterThanOrEquals) {
                first = Expression::GreaterThanOrEquals {
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
        if self.accept(TokenType::FloatToInt)? {
            self.expect(TokenType::OpeningParenthesis)?;
            let target = self.expression_level_1()?;
            return Ok(Expression::FloatToInt {
                source_token: self.expect(TokenType::ClosingParenthesis)?,
                expression: Box::new(target),
            });
        }
        if self.accept(TokenType::IntToFloat)? {
            self.expect(TokenType::OpeningParenthesis)?;
            let target = self.expression_level_1()?;
            return Ok(Expression::IntToFloat {
                source_token: self.expect(TokenType::ClosingParenthesis)?,
                expression: Box::new(target),
            });
        }
        self.expression_level_4()
    }

    fn expression_level_4(&mut self) -> Result<Expression, SourceCodeError> {
        let mut first = self.expression_level_5()?;

        while self.has(TokenType::BitwiseAnd)
            || self.has(TokenType::BitwiseOr)
            || self.has(TokenType::BitwiseXor)
            || self.has(TokenType::BitwiseLeftShift)
            || self.has(TokenType::BitwiseRightShift)
        {
            if self.has(TokenType::BitwiseAnd) {
                first = Expression::BitwiseAnd {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_5()?),
                }
            }
            if self.has(TokenType::BitwiseOr) {
                first = Expression::BitwiseOr {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_5()?),
                }
            }
            if self.has(TokenType::BitwiseXor) {
                first = Expression::BitwiseXor {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_5()?),
                }
            }
            if self.has(TokenType::BitwiseLeftShift) {
                first = Expression::BitwiseLeftShift {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_5()?),
                }
            }
            if self.has(TokenType::BitwiseRightShift) {
                first = Expression::BitwiseRightShift {
                    source_token: self.consume()?,
                    left_expression: Box::new(first),
                    right_expression: Box::new(self.expression_level_5()?),
                }
            }
        }
        Ok(first)
    }

    fn expression_level_5(&mut self) -> Result<Expression, SourceCodeError> {
        if self.has(TokenType::Negation) {
            return Ok(Expression::Negation {
                source_token: self.consume()?,
                expression: Box::new(self.expression_level_5()?),
            });
        }
        if self.has(TokenType::BitwiseNot) {
            return Ok(Expression::BitwiseNot {
                source_token: self.consume()?,
                expression: Box::new(self.expression_level_5()?),
            });
        }
        if self.has(TokenType::Not) {
            return Ok(Expression::Not {
                source_token: self.consume()?,
                expression: Box::new(self.expression_level_5()?),
            });
        }
        if self.has(TokenType::Subtraction) {
            return Ok(Expression::Negation {
                source_token: self.consume()?,
                expression: Box::new(self.expression_level_5()?),
            });
        }

        let first = self.expression_level_6()?;
        if self.has(TokenType::Exponentiation) {
            return Ok(Expression::Exponentiation {
                source_token: self.consume()?,
                left_expression: Box::new(first),
                right_expression: Box::new(self.expression_level_5()?),
            });
        }
        Ok(first)
    }

    fn expression_level_6(&mut self) -> Result<Expression, SourceCodeError> {
        if self.has(TokenType::Variable) {
            let token = self.consume()?;
            return Ok(Expression::Variable {
                source_token: token.clone(),
                name: token.source,
            });
        }

        if self.has(TokenType::Integer) {
            let token = self.consume()?;
            // This should have been checked by the lexer
            let value: i32 = token.source.parse().unwrap();
            return Ok(Expression::Integer {
                source_token: token,
                value,
            });
        }
        if self.has(TokenType::Float) {
            let token = self.consume()?;
            // This should have been checked by the lexer
            let value: f32 = token.source.parse().unwrap();
            return Ok(Expression::Float {
                source_token: token,
                value,
            });
        }
        if self.has(TokenType::Boolean) {
            let token = self.consume()?;
            // This should have been checked by the lexer
            let value: bool = token.source.parse().unwrap();
            return Ok(Expression::Boolean {
                source_token: token,
                value,
            });
        }
        if self.has(TokenType::String) {
            let token = self.consume()?;
            let len = token.source.len();
            let value = token.source[1..(len - 1)].to_string();
            return Ok(Expression::String {
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
