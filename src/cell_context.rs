use crate::expressions::Expression;
use crate::runtime::Runtime;

#[derive(Default)]
pub struct CellContext {}

impl CellContext {
    pub fn evaluate_with_context(
        runtime: &Runtime,
        code: &Expression,
    ) -> Result<Expression, String> {
        let mut context = Self::default();
        code.evaluate(runtime, &mut context)
    }
}
