
use crate::statements::Statement;
use crate::token::Token;

use crate::expressions::Expression;

#[derive(Clone)]
pub struct Cell {
    pub source_code: String,
    pub code: Box<Statement>,
    pub primative: Box<Expression>
}

impl Default for Cell {
    fn default() -> Self {
        let default_value = Box::new(Expression::String { source_token: Token::default(), value: "".to_owned() });
        let code = Box::new(Statement::Return { expression: default_value });
        let primative = Box::new(Expression::String { source_token: Token::default(), value: "".to_owned() });
        let source_code = "".to_string();

        Self { source_code, code, primative }
    }
}

pub struct GridState {
    pub grid: Vec<Vec<Box<Cell>>>,
    pub width: usize,
    pub height: usize,
}

impl GridState {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Box::new(Cell::default()); width]; height];
        GridState { grid, width, height }
    }
}
