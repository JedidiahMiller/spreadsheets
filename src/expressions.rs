use crate::cell_context::CellContext;
use crate::runtime::Runtime;
use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Expression {
    Number {
        source_token: Token,
        value: f64,
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

    LValue {
        source_token: Token,
        cell_address: Box<Expression>,
    },
    RValue {
        source_token: Token,
        cell_address: Box<Expression>,
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
            Expression::Number {
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

            Expression::LValue {
                source_token: _,
                cell_address,
            } => Ok(format!("{}", cell_address.serialize()?)),
            Expression::RValue {
                source_token: _,
                cell_address,
            } => Ok(format!("#{}", cell_address.serialize()?)),

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
            Expression::Number {
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

            Expression::Addition {
                source_token: _,
                left_expression,
                right_expression,
            } => {
                let left_evaluation = left_expression.evaluate(runtime, context)?;
                let right_evaluation = right_expression.evaluate(runtime, context)?;

                match (left_evaluation, right_evaluation) {
                    (
                        Expression::Number {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Number {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Number {
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
                        Expression::Number {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Number {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Number {
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
                        Expression::Number {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Number {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Number {
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

                match (left_evaluation, right_evaluation) {
                    (
                        Expression::Number {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Number {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Number {
                        source_token: left_source.merge(right_source),
                        value: left_value / right_value,
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
                        Expression::Number {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Number {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Number {
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
                        Expression::Number {
                            source_token: left_source,
                            value: left_value,
                        },
                        Expression::Number {
                            source_token: right_source,
                            value: right_value,
                        },
                    ) => Ok(Expression::Number {
                        source_token: left_source.merge(right_source),
                        value: left_value.powf(right_value),
                    }),
                    _ => Err("Cannot exponentiate non-numeric expressions".to_string()),
                }
            }
            Expression::Negation {
                source_token: _,
                expression,
            } => {
                let evaluation = expression.evaluate(runtime, context)?;

                match evaluation {
                    Expression::Number {
                        source_token,
                        value,
                    } => Ok(Expression::Number {
                        source_token: source_token.capture(),
                        value: -1.0 * value,
                    }),
                    _ => Err("Cannot numerically negate non-numeric expressions".to_string()),
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
                                Expression::Number {
                                    source_token: _,
                                    value: x,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: y,
                                },
                            ) => {
                                let result = runtime
                                    .get_cell_code(x as i32, y as i32)?
                                    .evaluate(runtime, context)?;
                                Ok(result)
                            }
                            _ => Err("Something went very wrong".to_string()),
                        }
                    }
                    _ => Err("RValue must contain a cell address".to_string()),
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
                        // The items should be numbers at this point.
                        match (*first_x, *first_y, *last_x, *last_y) {
                            (
                                Expression::Number {
                                    source_token: _,
                                    value: first_x,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: first_y,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: last_x,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: last_y,
                                },
                            ) => {
                                let (first_x, first_y, last_x, last_y) =
                                    (first_x as i32, first_y as i32, last_x as i32, last_y as i32);

                                // Prep first item
                                let first_cell_statement =
                                    runtime.get_cell_code(first_x, first_y)?;
                                let first_item = first_cell_statement.evaluate(runtime, context)?;

                                let mut max_value;

                                match first_item {
                                    Expression::Number {
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
                                            cell_statement.evaluate(runtime, context)?;

                                        match expression {
                                            Expression::Number {
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
                        // The items should be numbers at this point.
                        match (*first_x, *first_y, *last_x, *last_y) {
                            (
                                Expression::Number {
                                    source_token: _,
                                    value: first_x,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: first_y,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: last_x,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: last_y,
                                },
                            ) => {
                                let (first_x, first_y, last_x, last_y) =
                                    (first_x as i32, first_y as i32, last_x as i32, last_y as i32);

                                // Prep first item
                                let first_cell_statement =
                                    runtime.get_cell_code(first_x, first_y)?;
                                let first_item = first_cell_statement.evaluate(runtime, context)?;
                                let mut min_value;

                                match first_item {
                                    Expression::Number {
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
                                            cell_statement.evaluate(runtime, context)?;

                                        match expression {
                                            Expression::Number {
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
                // The items should be numbers at this point.
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
                                Expression::Number {
                                    source_token: _,
                                    value: first_x,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: first_y,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: last_x,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: last_y,
                                },
                            ) => {
                                let (first_x, first_y, last_x, last_y) =
                                    (first_x as i32, first_y as i32, last_x as i32, last_y as i32);

                                // Prep first item
                                let cell_count = (last_x - first_x + 1) * (last_y - first_y + 1);
                                let mut sum_amount = 0.0;

                                // Iterate over items in the range and apply the function
                                for row in first_y..=last_y {
                                    for col in first_x..=last_x {
                                        let cell_statement = runtime.get_cell_code(col, row)?;
                                        let expression =
                                            cell_statement.evaluate(runtime, context)?;

                                        match expression {
                                            Expression::Number {
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
                                return Ok(Expression::Number {
                                    source_token: source_token.clone().capture(),
                                    value: sum_amount / cell_count as f64,
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
                        // The items should be numbers at this point.
                        match (*first_x, *first_y, *last_x, *last_y) {
                            (
                                Expression::Number {
                                    source_token: _,
                                    value: first_x,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: first_y,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: last_x,
                                },
                                Expression::Number {
                                    source_token: _,
                                    value: last_y,
                                },
                            ) => {
                                let (first_x, first_y, last_x, last_y) =
                                    (first_x as i32, first_y as i32, last_x as i32, last_y as i32);

                                // Prep first item
                                let mut sum_amount = 0.0;

                                // Check items
                                for row in first_y..=last_y {
                                    for col in first_x..=last_x {
                                        let cell_statement = runtime.get_cell_code(col, row)?;
                                        let expression =
                                            cell_statement.evaluate(runtime, context)?;

                                        match expression {
                                            Expression::Number {
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
                                return Ok(Expression::Number {
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

fn float_equals(a: f64, b: f64) -> bool {
    // Numbers within a millionth of each other are consitered equal
    const FLOAT_COMPARISON_TOLERANCE: f64 = 0.000001;

    (a - b).abs() < FLOAT_COMPARISON_TOLERANCE
}
