use std::collections::HashMap;

use crate::expressions::Expression;
use crate::runtime::Runtime;
use crate::statements::EvaluationResult;
use crate::statements::Statement;

#[derive(Default)]
pub struct CellContext {
    variables: HashMap<String, Expression>,
}

impl CellContext {
    pub fn get_var(&self, name: &String) -> Result<Expression, ()> {
        if self.variables.contains_key(name) {
            return Ok(self.variables.get(name).unwrap().clone());
        }
        Err(())
    }

    pub fn set_var(&mut self, name: &String, value: Expression) -> bool {
        self.variables.insert(String::from(name), value).is_none()
    }

    pub fn evaluate_with_context(
        runtime: &Runtime,
        code: &Statement,
    ) -> Result<Expression, String> {
        let mut context = Self::default();
        match code.evaluate(runtime, &mut context) {
            EvaluationResult::ReturnValue { expression } => Ok(*expression),
            EvaluationResult::None => Err(String::from("Did not find a return value")),
            EvaluationResult::Error { message } => Err(message),
        }
    }
}
