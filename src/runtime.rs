use crate::cell_context::CellContext;
use crate::expressions::Expression;
use crate::grid::*;

pub struct Runtime {
    grid_state: GridState,
}

impl Runtime {
    pub fn new(grid_size: (usize, usize)) -> Runtime {
        let (width, height) = grid_size;
        Self {
            grid_state: GridState::new(width, height),
        }
    }

    pub fn set_cell(&mut self, x: i32, y: i32, cell: &Cell) -> Result<(), String> {
        if x < 0 || y < 0 {
            return Err("Trying to get illegal cell address".to_string());
        }
        let ux = x as usize;
        let uy = y as usize;
        self.grid_state.grid[uy][ux].source_code = cell.source_code.clone();
        self.grid_state.grid[uy][ux].code = cell.code.clone();
        self.grid_state.grid[uy][ux].error = cell.error.clone();

        let mut eval_context = CellContext::default();
        let eval_result = self.grid_state.grid[uy][ux]
            .code
            .evaluate(&self, &mut eval_context);
        match eval_result {
            Ok(expression) => {
                self.grid_state.grid[uy][ux].primative = Box::new(expression);
                Ok(())
            }
            Err(message) => Err(message),
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Result<Box<Cell>, String> {
        if x < 0 || y < 0 {
            return Err("Trying to get illegal cell address".to_string());
        }
        let ux = x as usize;
        let uy = y as usize;
        if ux >= self.grid_state.width || uy >= self.grid_state.height {
            return Err("Trying to get illegal cell address".to_string());
        }
        Ok(Box::new(*self.grid_state.grid[uy][ux].clone()))
    }

    pub fn get_cell_code(&self, x: i32, y: i32) -> Result<Expression, String> {
        let cell = self.get_cell(x, y)?;
        Ok(*cell.code)
    }
}
