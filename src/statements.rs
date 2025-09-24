
use crate::cell_context::CellContext;
use crate::expressions::Expression;
use crate::runtime::Runtime;
use crate::token::Token;
use crate::token::TokenType;

#[derive(Clone)]
#[derive(Debug)]
pub enum EvaluationResult {
    ReturnValue { expression: Box<Expression> },
    Error { message: String },
    None
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Statement {
    Block { statements: Vec<Statement> },
    Assignment { target: String, value: Box<Expression> },
    IfElse { conditional: Box<Expression>, true_block: Box<Statement>, false_block: Box<Statement> },
    ForEach { variable: String, range_start: Box<Expression>, range_end: Box<Expression>, body: Box<Statement> },
    Return { expression: Box<Expression> }
}

impl Statement {
    pub fn evaluate(&self, runtime: &Runtime, context: &mut CellContext) -> EvaluationResult {
        match self {
            Statement::Block { statements } => {
                for statement in statements {
                    let result = statement.evaluate(runtime, context);
                    match result {
                        EvaluationResult::ReturnValue { .. } | EvaluationResult::Error { .. } => {
                            return result;
                        },
                        EvaluationResult::None => {},
                    };
                }
            },
            Statement::Assignment { target, value } => {
                let evaluated = match value.evaluate(runtime, context) {
                    Ok(value) => value,
                    Err(message) => { 
                        return EvaluationResult::Error { message } 
                    },
                };
                context.set_var(target, evaluated);
                
                return EvaluationResult::None
            },
            Statement::IfElse { conditional, true_block, false_block } => {
                let conditional_result;
                match conditional.evaluate(runtime, context) {
                    Ok(val) => conditional_result = val,
                    Err(message) => return EvaluationResult::Error { message },
                }

                match conditional_result {
                    Expression::Boolean { source_token: _, value } => {
                        if value {
                            return true_block.evaluate(runtime, context)
                        }
                        return false_block.evaluate(runtime, context)
                    },
                    _ => return EvaluationResult::Error { message: String::from("If statement needs a boolean conditional") }
                }
            },
            Statement::ForEach { variable, range_start, range_end, body } => {

                // Start
                let start;
                let raw_start = range_start.evaluate(runtime, context);
                match raw_start {
                    Ok(expression) => {
                        match expression {
                            Expression::Integer { source_token: _, value } => {
                                start = value
                            },
                            _ => {
                                return EvaluationResult::Error { message: String::from("Range start expression must be an integer") }
                            }
                        }
                    },
                    Err(message) => {
                        return EvaluationResult::Error { message }
                    }
                }

                // End
                let end;
                let raw_end = range_end.evaluate(runtime, context);
                match raw_end {
                    Ok(expression) => {
                        match expression {
                            Expression::Integer { source_token: _, value } => {
                                end = value
                            },
                            _ => {
                                return EvaluationResult::Error { message: String::from("Range start expression must be an integer") }
                            }
                        }
                    },
                    Err(message) => {
                        return EvaluationResult::Error { message }
                    }
                }

                // Loop
                let mut current = start;
                while current < end {
                    let source_token = Token::new(TokenType::Unknown, String::from("?"), 0, 0);
                    context.set_var(variable, Expression::Integer { source_token, value: current });
                    let result = body.evaluate(runtime, context);
                    match result {
                        EvaluationResult::ReturnValue { .. } | EvaluationResult::Error { .. } => {
                            return result;
                        },
                        EvaluationResult::None => {},
                    };
                    current += 1;
                }
            },
            Statement::Return { expression } => {
                let result = expression.evaluate(runtime, context);
                match result {
                    Ok(value) => {
                        return EvaluationResult::ReturnValue { expression: Box::new(value) }
                    },
                    Err(message) => {
                        return EvaluationResult::Error { message }
                    },
                }
            },
        };
        
        EvaluationResult::None
    }
}
