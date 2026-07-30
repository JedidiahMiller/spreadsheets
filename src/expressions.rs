use crate::cell_context::CellContext;
use crate::runtime::Runtime;
use crate::statements::EvaluationResult;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Expression {
    Integer {
        source_token: Token,
        value: i32,
    },
    Float {
        source_token: Token,
        value: f32,
    },
    Boolean {
        source_token: Token,
        value: bool,
    },
    String {
        source_token: Token,
        value: String,
    },
    CellAddress {
        source_token: Token,
        x_value: Box<Expression>,
        y_value: Box<Expression>,
    },
    Variable {
        source_token: Token,
        name: String,
    },

    Addition {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    Subtraction {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    Multiplication {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    Division {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    Modulo {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    Exponentiation {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    Negation {
        source_token: Token,
        expression: Box<Expression>,
    },

    And {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    Or {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    Not {
        source_token: Token,
        expression: Box<Expression>,
    },

    LValue {
        source_token: Token,
        cell_address: Box<Expression>,
    },
    RValue {
        source_token: Token,
        cell_address: Box<Expression>,
    },

    BitwiseAnd {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    BitwiseOr {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    BitwiseNot {
        source_token: Token,
        expression: Box<Expression>,
    },
    BitwiseXor {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    BitwiseLeftShift {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    BitwiseRightShift {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },

    Equals {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    NotEquals {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    LessThan {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    LessThanOrEquals {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    GreaterThan {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },
    GreaterThanOrEquals {
        source_token: Token,
        left_expression: Box<Expression>,
        right_expression: Box<Expression>,
    },

    FloatToInt {
        source_token: Token,
        expression: Box<Expression>,
    },
    IntToFloat {
        source_token: Token,
        expression: Box<Expression>,
    },

    Max {
        source_token: Token,
        first_address: Box<Expression>,
        last_address: Box<Expression>,
    },
    Min {
        source_token: Token,
        first_address: Box<Expression>,
        last_address: Box<Expression>,
    },
    Mean {
        source_token: Token,
        first_address: Box<Expression>,
        last_address: Box<Expression>,
    },
    Sum {
        source_token: Token,
        first_address: Box<Expression>,
        last_address: Box<Expression>,
    },
}

impl Expression {
    pub fn serialize(&self) -> Result<String, String> {
        match self {
            Expression::Integer {
                source_token: _,
                value,
            } => Ok(value.to_string()),
            Expression::Float {
                source_token: _,
                value,
            } => {
                let mut decimal_string = value.to_string();
                if !decimal_string.contains('.') {
                    decimal_string.push_str(&".0");
                }
                Ok(decimal_string)
            }
            Expression::Boolean {
                source_token: _,
                value,
            } => Ok(value.to_string()),
            Expression::String {
                source_token: _,
                value,
            } => Ok(value.to_string()),
            Expression::CellAddress {
                source_token: _,
                x_value,
                y_value,
            } => Ok(format!(
                "[{}, {}]",
                x_value.serialize()?,
                y_value.serialize()?
            )),
            Expression::Variable {
                source_token: _,
                name,
            } => Ok(String::from(name)),

            Expression::Addition {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} + {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::Subtraction {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} - {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::Multiplication {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} * {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::Division {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} / {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::Modulo {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} % {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::Exponentiation {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} ^ {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::Negation {
                source_token: _,
                expression,
            } => Ok(format!("-{}", expression.serialize()?)),

            Expression::And {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} && {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::Or {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} || {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::Not {
                source_token: _,
                expression,
            } => Ok(format!("!{}", expression.serialize()?)),

            Expression::LValue {
                source_token: _,
                cell_address,
            } => Ok(format!("{}", cell_address.serialize()?)),
            Expression::RValue {
                source_token: _,
                cell_address,
            } => Ok(format!("#{}", cell_address.serialize()?)),

            Expression::BitwiseAnd {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} & {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::BitwiseOr {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} | {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::BitwiseNot {
                source_token: _,
                expression,
            } => Ok(format!("~{}", expression.serialize()?)),
            Expression::BitwiseXor {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} xor {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::BitwiseLeftShift {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} << {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::BitwiseRightShift {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} >> {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),

            Expression::Equals {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} == {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::NotEquals {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} != {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::LessThan {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} < {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::LessThanOrEquals {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} <= {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::GreaterThan {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} > {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),
            Expression::GreaterThanOrEquals {
                source_token: _,
                left_expression,
                right_expression,
            } => Ok(format!(
                "({} >= {})",
                left_expression.serialize()?,
                right_expression.serialize()?
            )),

            Expression::FloatToInt {
                source_token: _,
                expression,
            } => Ok(format!("Int({})", expression.serialize()?)),
            Expression::IntToFloat {
                source_token: _,
                expression,
            } => Ok(format!("Float({})", expression.serialize()?)),

            Expression::Max {
                source_token: _,
                first_address,
                last_address,
            } => Ok(format!(
                "Max({}, {})",
                first_address.serialize()?,
                last_address.serialize()?
            )),
            Expression::Min {
                source_token: _,
                first_address,
                last_address,
            } => Ok(format!(
                "Min({}, {})",
                first_address.serialize()?,
                last_address.serialize()?
            )),
            Expression::Mean {
                source_token: _,
                first_address,
                last_address,
            } => Ok(format!(
                "Mean({}, {})",
                first_address.serialize()?,
                last_address.serialize()?
            )),
            Expression::Sum {
                source_token: _,
                first_address,
                last_address,
            } => Ok(format!(
                "Sum({}, {})",
                first_address.serialize()?,
                last_address.serialize()?
            )),
        }
    }

    pub fn evaluate(
        &self,
        runtime: &Runtime,
        context: &mut CellContext,
    ) -> Result<Expression, String> {
        match self {
            Expression::Integer {
                source_token: _,
                value: _,
            } => Ok(self.clone()),
            Expression::Float {
                source_token: _,
                value: _,
            } => Ok(self.clone()),
            Expression::Boolean {
                source_token: _,
                value: _,
            } => Ok(self.clone()),
            Expression::String {
                source_token: _,
                value: _,
            } => Ok(self.clone()),
            Expression::CellAddress {
                source_token: _,
                x_value: _,
                y_value: _,
            } => Ok(self.clone()),
            Expression::Variable {
                source_token: _,
                name,
            } => {
                let val = context.get_var(name);
                if val.is_err() {
                    return Err(format!("Could not find variable \"{}\"", name));
                }
                Ok(val.unwrap())
            }

            Expression::Addition {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_evaluation = left_expression.evaluate(runtime, context)?;
                let right_evaluation = right_expression.evaluate(runtime, context)?;

                match (left_evaluation, right_evaluation) {
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value + right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value + right_value as f32,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value as f32 + right_value,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value + right_value,
                    }),
                    _ => Err("Cannot add non-numeric expressions".to_string()),
                }
            }
            Expression::Subtraction {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_evaluation = left_expression.evaluate(runtime, context)?;
                let right_evaluation = right_expression.evaluate(runtime, context)?;

                match (left_evaluation, right_evaluation) {
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value - right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value - right_value as f32,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value as f32 - right_value,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value - right_value,
                    }),
                    _ => Err("Cannot subtract non-numeric expressions".to_string()),
                }
            }
            Expression::Multiplication {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_evaluation = left_expression.evaluate(runtime, context)?;
                let right_evaluation = right_expression.evaluate(runtime, context)?;

                match (left_evaluation, right_evaluation) {
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value * right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value * right_value as f32,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value as f32 * right_value,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value * right_value,
                    }),
                    _ => Err("Cannot multiply non-numeric expressions".to_string()),
                }
            }
            Expression::Division {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_evaluation = left_expression.evaluate(runtime, context)?;
                let right_evaluation = right_expression.evaluate(runtime, context)?;

                // Division always returns a float. Theoretically it could be smart and try to use
                // integers where possible, but this inconsistency seems like a worse UX
                match (left_evaluation, right_evaluation) {
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value / right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value / right_value as f32,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value as f32 / right_value,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value as f32 / right_value as f32,
                    }),
                    _ => Err("Cannot divide non-numeric expressions".to_string()),
                }
            }
            Expression::Modulo {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_evaluation = left_expression.evaluate(runtime, context)?;
                let right_evaluation = right_expression.evaluate(runtime, context)?;

                match (left_evaluation, right_evaluation) {
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value % right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value % right_value as f32,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value as f32 % right_value,
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value % right_value,
                    }),
                    _ => Err("Cannot mod non-numeric expressions".to_string()),
                }
            }
            Expression::Exponentiation {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_evaluation = left_expression.evaluate(runtime, context)?;
                let right_evaluation = right_expression.evaluate(runtime, context)?;

                match (left_evaluation, right_evaluation) {
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value.powf(right_value),
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Float {
                        source_token: left_source.merge(right_source),
                        value: left_value.powi(right_value),
                    }),
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => {
                        // This is not natural for rust. It does not directly support int^float
                        Ok(Expression::Float {
                            source_token: left_source.merge(right_source),
                            value: (left_value as f32).powf(right_value),
                        })
                    }
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value.pow(right_value as u32),
                    }),
                    _ => Err("Cannot add non-numeric expressions".to_string()),
                }
            }
            Expression::Negation {
                source_token: _,
                expression,
            } => {
                let evaluation = expression.evaluate(runtime, context)?;

                match evaluation {
                    Expression::Float {
                        source_token,
                        value,
                    } => Ok(Expression::Float {
                        source_token: source_token.capture(),
                        value: -1.0 * value,
                    }),
                    Expression::Integer {
                        source_token,
                        value,
                    } => {
                        // This is not natural for rust. It does not directly support int^float
                        Ok(Expression::Integer {
                            source_token: source_token.capture(),
                            value: -1 * value,
                        })
                    }
                    _ => Err("Cannot numerically negate non-numeric expressions".to_string()),
                }
            }

            Expression::And {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_evaluation = left_expression.evaluate(runtime, context)?;
                let right_evaluation = right_expression.evaluate(runtime, context)?;

                let left_value: bool;
                let left_token: Token;
                match left_evaluation {
                    Expression::Boolean {
                        source_token,
                        value,
                    } => {
                        left_value = value;
                        left_token = source_token;
                    }
                    _ => return Err("Cannot or non-boolean expressions".to_string()),
                }
                if !left_value {
                    return Ok(Expression::Boolean {
                        source_token: left_token.capture(),
                        value: false,
                    });
                }
                match right_evaluation {
                    Expression::Boolean {
                        source_token,
                        value,
                    } => Ok(Expression::Boolean {
                        source_token: left_token.merge(source_token),
                        value,
                    }),
                    _ => Err("Cannot or non-boolean expressions".to_string()),
                }
            }
            Expression::Or {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_evaluation = left_expression.evaluate(runtime, context)?;
                let right_evaluation = right_expression.evaluate(runtime, context)?;

                let left_value: bool;
                let left_token: Token;
                match left_evaluation {
                    Expression::Boolean {
                        source_token,
                        value,
                    } => {
                        left_value = value;
                        left_token = source_token;
                    }
                    _ => return Err("Cannot or non-boolean expressions".to_string()),
                }
                if left_value {
                    return Ok(Expression::Boolean {
                        source_token: left_token.capture(),
                        value: true,
                    });
                }
                match right_evaluation {
                    Expression::Boolean {
                        source_token,
                        value,
                    } => Ok(Expression::Boolean {
                        source_token: left_token.merge(source_token),
                        value,
                    }),
                    _ => Err("Cannot or non-boolean expressions".to_string()),
                }
            }
            Expression::Not {
                source_token: _,
                expression,
            } => {
                let evaluation = expression.evaluate(runtime, context)?;

                match evaluation {
                    Expression::Boolean {
                        source_token,
                        value,
                    } => Ok(Expression::Boolean {
                        source_token: source_token.capture(),
                        value: !value,
                    }),
                    _ => Err("Cannot do boolean negation on non-boolean expressions".to_string()),
                }
            }

            Expression::LValue {
                source_token: _,
                cell_address,
            } => {
                let address = cell_address.evaluate(runtime, context)?;

                match address {
                    Expression::CellAddress {
                        source_token,
                        x_value,
                        y_value,
                    } => Ok(Expression::CellAddress {
                        source_token: source_token.capture(),
                        x_value,
                        y_value,
                    }),
                    _ => Err("Cell address must be specified using two integers".to_string()),
                }
            }
            Expression::RValue {
                source_token: _,
                cell_address,
            } => {
                let address = cell_address.evaluate(runtime, context)?;

                match address {
                    Expression::CellAddress {
                        source_token: _,
                        x_value: x_expression,
                        y_value: y_expression,
                    } => {
                        let x_value = x_expression.evaluate(runtime, context)?;
                        let y_value = y_expression.evaluate(runtime, context)?;

                        match (x_value, y_value) {
                            (
                                Expression::Integer {
                                    source_token: _,
                                    value: x,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: y,
                                },
                            ) => {
                                let result = *match runtime
                                    .get_cell_code(x, y)?
                                    .evaluate(runtime, context)
                                {
                                    EvaluationResult::ReturnValue { expression } => Ok(expression),
                                    EvaluationResult::Error { message } => Err(message),
                                    EvaluationResult::None => {
                                        Err(String::from("Did not find return value"))
                                    }
                                }?;
                                Ok(result)
                            }
                            _ => Err("Something went very wrong".to_string()),
                        }
                    }
                    _ => Err("RValue must contain a cell address".to_string()),
                }
            }

            Expression::BitwiseAnd {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value & right_value,
                    }),
                    _ => Err("Bitwise and must use two integers".to_string()),
                }
            }
            Expression::BitwiseOr {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value | right_value,
                    }),
                    _ => Err("Bitwise or must use two integers".to_string()),
                }
            }
            Expression::BitwiseNot {
                source_token: _,
                expression,
            } => {
                let value = expression.evaluate(runtime, context)?;

                match value {
                    Expression::Integer {
                        source_token,
                        value,
                    } => Ok(Expression::Integer {
                        source_token: source_token.capture(),
                        value: !value,
                    }),
                    _ => Err("Bitwise not must use a integer".to_string()),
                }
            }
            Expression::BitwiseXor {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value ^ right_value,
                    }),
                    _ => Err("Bitwise xor must use two integers".to_string()),
                }
            }
            Expression::BitwiseLeftShift {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value << right_value,
                    }),
                    _ => Err("Bitwise shifts must use two integers".to_string()),
                }
            }
            Expression::BitwiseRightShift {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Integer {
                        source_token: left_source.merge(right_source),
                        value: left_value >> right_value,
                    }),
                    _ => Err("Bitwise shifts must use two integers".to_string()),
                }
            }

            Expression::Equals {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Boolean {
                        source_token: left_source.merge(right_source),
                        value: left_value == right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Boolean {
                        source_token: left_source.merge(right_source),
                        value: float_equals(left_value, right_value),
                    }),
                    _ => Err(
                        "Equality check can only be done on similar numerical values".to_string(),
                    ),
                }
            }
            Expression::NotEquals {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Boolean {
                        source_token: left_source.merge(right_source),
                        value: left_value != right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Boolean {
                        source_token: left_source.merge(right_source),
                        value: !float_equals(left_value, right_value),
                    }),
                    _ => Err(
                        "Equality check can only be done on similar numerical values".to_string(),
                    ),
                }
            }
            Expression::LessThan {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Boolean {
                        source_token: left_source.merge(right_source),
                        value: left_value < right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => {
                        let not_equal = !float_equals(left_value, right_value);
                        Ok(Expression::Boolean {
                            source_token: left_source.merge(right_source),
                            value: left_value < right_value && not_equal,
                        })
                    }
                    _ => Err(
                        "Equality check can only be done on similar numerical values".to_string(),
                    ),
                }
            }
            Expression::LessThanOrEquals {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Boolean {
                        source_token: left_source.merge(right_source),
                        value: left_value < right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => {
                        let equal = float_equals(left_value, right_value);
                        Ok(Expression::Boolean {
                            source_token: left_source.merge(right_source),
                            value: left_value < right_value || equal,
                        })
                    }
                    _ => Err(
                        "Equality check can only be done on similar numerical values".to_string(),
                    ),
                }
            }
            Expression::GreaterThan {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Boolean {
                        source_token: left_source.merge(right_source),
                        value: left_value > right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => {
                        let not_equal = !float_equals(left_value, right_value);
                        Ok(Expression::Boolean {
                            source_token: left_source.merge(right_source),
                            value: left_value > right_value && not_equal,
                        })
                    }
                    _ => Err(
                        "Equality check can only be done on similar numerical values".to_string(),
                    ),
                }
            }
            Expression::GreaterThanOrEquals {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_value = left_expression.evaluate(runtime, context)?;
                let right_value = right_expression.evaluate(runtime, context)?;

                match (left_value, right_value) {
                    (
                        Expression::Integer {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Integer {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Boolean {
                        source_token: left_source.merge(right_source),
                        value: left_value > right_value,
                    }),
                    (
                        Expression::Float {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Float {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => {
                        let equal = float_equals(left_value, right_value);
                        Ok(Expression::Boolean {
                            source_token: left_source.merge(right_source),
                            value: left_value > right_value || equal,
                        })
                    }
                    _ => Err(
                        "Equality check can only be done on similar numerical values".to_string(),
                    ),
                }
            }

            Expression::FloatToInt {
                source_token: _,
                expression,
            } => {
                let value = expression.evaluate(runtime, context)?;

                match value {
                    Expression::Float {
                        source_token,
                        value,
                    } => Ok(Expression::Integer {
                        source_token: source_token.capture(),
                        value: value as i32,
                    }),
                    _ => Err("Only floats can be converted to an integer".to_string()),
                }
            }
            Expression::IntToFloat {
                source_token: _,
                expression,
            } => {
                let value = expression.evaluate(runtime, context)?;

                match value {
                    Expression::Integer {
                        source_token,
                        value,
                    } => Ok(Expression::Float {
                        source_token: source_token.capture(),
                        value: value as f32,
                    }),
                    _ => Err("Only integers can be converted to a float".to_string()),
                }
            }

            Expression::Max {
                source_token: _,
                first_address,
                last_address,
            } => {
                let first_postition = first_address.evaluate(runtime, context)?;
                let last_position = last_address.evaluate(runtime, context)?;

                match (first_postition, last_position) {
                    (
                        Expression::CellAddress {
                            source_token: _,
                            x_value: first_x,
                            y_value: first_y,
                        },
                        Expression::CellAddress {
                            source_token: _,
                            x_value: last_x,
                            y_value: last_y,
                        },
                    ) => {
                        // This match purely to appease the rust type system.
                        // The items should be integers at this point.
                        match (*first_x, *first_y, *last_x, *last_y) {
                            (
                                Expression::Integer {
                                    source_token: _,
                                    value: first_x,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: first_y,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: last_x,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: last_y,
                                },
                            ) => {
                                // Prep first item
                                let first_cell_statement =
                                    runtime.get_cell_code(first_x, first_y)?;
                                let first_item = *match first_cell_statement
                                    .evaluate(runtime, context)
                                {
                                    EvaluationResult::ReturnValue { expression } => Ok(expression),
                                    EvaluationResult::Error { message } => Err(message),
                                    EvaluationResult::None => {
                                        Err(String::from("Did not find return value"))
                                    }
                                }?;

                                let mut max_value;

                                match first_item {
                                    Expression::Integer {
                                        source_token: _,
                                        value,
                                    } => {
                                        max_value = value as f32;
                                    }
                                    Expression::Float {
                                        source_token: _,
                                        value,
                                    } => {
                                        max_value = value;
                                    }
                                    _ => {
                                        return Err("Cannot find max of a collection that includes a non-numeric value".to_string());
                                    }
                                }
                                let mut max_expression: Expression = first_item;
                                // Iterate over items in the range and apply the function
                                for row in first_y..=last_y {
                                    for col in first_x..=last_x {
                                        let cell_statement = runtime.get_cell_code(col, row)?;
                                        let expression =
                                            *match cell_statement.evaluate(runtime, context) {
                                                EvaluationResult::ReturnValue { expression } => {
                                                    Ok(expression)
                                                }
                                                EvaluationResult::Error { message } => Err(message),
                                                EvaluationResult::None => {
                                                    Err(String::from("Did not find return value"))
                                                }
                                            }?;

                                        match expression {
                                            Expression::Integer {
                                                source_token: _,
                                                value,
                                            } => {
                                                if value as f32 > max_value
                                                    && !float_equals(value as f32, max_value)
                                                {
                                                    max_value = value as f32;
                                                    max_expression = expression.clone();
                                                }
                                            }
                                            Expression::Float {
                                                source_token: _,
                                                value,
                                            } => {
                                                if value > max_value
                                                    && !float_equals(value, max_value)
                                                {
                                                    max_value = value;
                                                    max_expression = expression.clone();
                                                }
                                            }
                                            _ => {
                                                return Err("Cannot find max of a collection that includes a non-numeric value".to_string());
                                            }
                                        }
                                    }
                                }
                                return Ok(max_expression);
                            }
                            _ => {
                                return Err("Something went very wrong".to_string());
                            }
                        }
                    }
                    _ => return Err("Parameters must be addresses".to_string()),
                }
            }
            Expression::Min {
                source_token: _,
                first_address,
                last_address,
            } => {
                let first_postition = first_address.evaluate(runtime, context)?;
                let last_position = last_address.evaluate(runtime, context)?;

                match (first_postition, last_position) {
                    (
                        Expression::CellAddress {
                            source_token: _,
                            x_value: first_x,
                            y_value: first_y,
                        },
                        Expression::CellAddress {
                            source_token: _,
                            x_value: last_x,
                            y_value: last_y,
                        },
                    ) => {
                        // This match purely to appease the rust type system.
                        // The items should be integers at this point.
                        match (*first_x, *first_y, *last_x, *last_y) {
                            (
                                Expression::Integer {
                                    source_token: _,
                                    value: first_x,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: first_y,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: last_x,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: last_y,
                                },
                            ) => {
                                // Prep first item
                                let first_cell_statement =
                                    runtime.get_cell_code(first_x, first_y)?;
                                let first_item = *match first_cell_statement
                                    .evaluate(runtime, context)
                                {
                                    EvaluationResult::ReturnValue { expression } => Ok(expression),
                                    EvaluationResult::Error { message } => Err(message),
                                    EvaluationResult::None => {
                                        Err(String::from("Did not find return value"))
                                    }
                                }?;
                                let mut min_value;

                                match first_item {
                                    Expression::Integer {
                                        source_token: _,
                                        value,
                                    } => {
                                        min_value = value as f32;
                                    }
                                    Expression::Float {
                                        source_token: _,
                                        value,
                                    } => {
                                        min_value = value;
                                    }
                                    _ => {
                                        return Err("Cannot find min of a collection that includes a non-numeric value".to_string());
                                    }
                                }
                                let mut min_expression = first_item;
                                // Iterate over items in the range and apply the function
                                for row in first_y..=last_y {
                                    for col in first_x..=last_x {
                                        let cell_statement = runtime.get_cell_code(col, row)?;
                                        let expression =
                                            *match cell_statement.evaluate(runtime, context) {
                                                EvaluationResult::ReturnValue { expression } => {
                                                    Ok(expression)
                                                }
                                                EvaluationResult::Error { message } => Err(message),
                                                EvaluationResult::None => {
                                                    Err(String::from("Did not find return value"))
                                                }
                                            }?;

                                        match expression {
                                            Expression::Integer {
                                                source_token: _,
                                                value,
                                            } => {
                                                if (value as f32) < min_value
                                                    && !float_equals(value as f32, min_value)
                                                {
                                                    min_value = value as f32;
                                                    min_expression = expression.clone();
                                                }
                                            }
                                            Expression::Float {
                                                source_token: _,
                                                value,
                                            } => {
                                                if value < min_value
                                                    && !float_equals(value, min_value)
                                                {
                                                    min_value = value;
                                                    min_expression = expression.clone();
                                                }
                                            }
                                            _ => {
                                                return Err("Cannot find min of a collection that includes a non-numeric value".to_string());
                                            }
                                        }
                                    }
                                }
                                return Ok(min_expression);
                            }
                            _ => {
                                return Err("Something went very wrong".to_string());
                            }
                        }
                    }
                    _ => return Err("Parameters must be addresses".to_string()),
                }
            }
            Expression::Mean {
                source_token,
                first_address,
                last_address,
            } => {
                let first_postition = first_address.evaluate(runtime, context)?;
                let last_position = last_address.evaluate(runtime, context)?;

                // This match purely to appease the rust type system.
                // The items should be integers at this point.
                match (first_postition, last_position) {
                    (
                        Expression::CellAddress {
                            source_token: _,
                            x_value: first_x,
                            y_value: first_y,
                        },
                        Expression::CellAddress {
                            source_token: _,
                            x_value: last_x,
                            y_value: last_y,
                        },
                    ) => {
                        match (*first_x, *first_y, *last_x, *last_y) {
                            (
                                Expression::Integer {
                                    source_token: _,
                                    value: first_x,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: first_y,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: last_x,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: last_y,
                                },
                            ) => {
                                // Prep first item
                                let cell_count = (last_x - first_x + 1) * (last_y - first_y + 1);
                                let mut sum_amount = 0.0;

                                // Iterate over items in the range and apply the function
                                for row in first_y..=last_y {
                                    for col in first_x..=last_x {
                                        let cell_statement = runtime.get_cell_code(col, row)?;
                                        let expression =
                                            *match cell_statement.evaluate(runtime, context) {
                                                EvaluationResult::ReturnValue { expression } => {
                                                    Ok(expression)
                                                }
                                                EvaluationResult::Error { message } => Err(message),
                                                EvaluationResult::None => {
                                                    Err(String::from("Did not find return value"))
                                                }
                                            }?;

                                        match expression {
                                            Expression::Integer {
                                                source_token: _,
                                                value,
                                            } => {
                                                sum_amount += value as f32;
                                            }
                                            Expression::Float {
                                                source_token: _,
                                                value,
                                            } => {
                                                sum_amount += value;
                                            }
                                            _ => {
                                                return Err("Cannot find average of a collection that includes a non-numeric value".to_string());
                                            }
                                        }
                                    }
                                }
                                return Ok(Expression::Float {
                                    source_token: source_token.clone().capture(),
                                    value: sum_amount / cell_count as f32,
                                });
                            }
                            _ => {
                                return Err("Something went very wrong".to_string());
                            }
                        }
                    }
                    _ => return Err("Parameters must be addresses".to_string()),
                }
            }
            Expression::Sum {
                source_token,
                first_address,
                last_address,
            } => {
                let first_postition = first_address.evaluate(runtime, context)?;
                let last_position = last_address.evaluate(runtime, context)?;

                match (first_postition, last_position) {
                    (
                        Expression::CellAddress {
                            source_token: _,
                            x_value: first_x,
                            y_value: first_y,
                        },
                        Expression::CellAddress {
                            source_token: _,
                            x_value: last_x,
                            y_value: last_y,
                        },
                    ) => {
                        // This match purely to appease the rust type system.
                        // The items should be integers at this point.
                        match (*first_x, *first_y, *last_x, *last_y) {
                            (
                                Expression::Integer {
                                    source_token: _,
                                    value: first_x,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: first_y,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: last_x,
                                },
                                Expression::Integer {
                                    source_token: _,
                                    value: last_y,
                                },
                            ) => {
                                // Prep first item
                                let mut sum_amount = 0.0;

                                // Check items
                                for row in first_y..=last_y {
                                    for col in first_x..=last_x {
                                        let cell_statement = runtime.get_cell_code(col, row)?;
                                        let expression =
                                            *match cell_statement.evaluate(runtime, context) {
                                                EvaluationResult::ReturnValue { expression } => {
                                                    Ok(expression)
                                                }
                                                EvaluationResult::Error { message } => Err(message),
                                                EvaluationResult::None => {
                                                    Err(String::from("Did not find return value"))
                                                }
                                            }?;

                                        match expression {
                                            Expression::Integer {
                                                source_token: _,
                                                value,
                                            } => {
                                                sum_amount += value as f32;
                                            }
                                            Expression::Float {
                                                source_token: _,
                                                value,
                                            } => {
                                                sum_amount += value;
                                            }
                                            _ => {
                                                return Err("Cannot find sum of a collection that includes a non-numeric value".to_string());
                                            }
                                        }
                                    }
                                }
                                return Ok(Expression::Float {
                                    source_token: source_token.clone().capture(),
                                    value: sum_amount,
                                });
                            }
                            _ => {
                                return Err("Something went very wrong".to_string());
                            }
                        }
                    }
                    _ => return Err("Parameters must be addresses".to_string()),
                }
            }
        }
    }
}

fn float_equals(a: f32, b: f32) -> bool {
    // Floats within a millionth of each other are consitered equal
    const FLOAT_COMPARISON_TOLERANCE: f32 = 0.000001;

    (a - b).abs() < FLOAT_COMPARISON_TOLERANCE
}
