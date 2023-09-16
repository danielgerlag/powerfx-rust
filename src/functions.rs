use std::sync::Arc;

use crate::{evaluator::{ExpressionEvaluator, ExpressionEvaluationContext, EvaluationError}, function_registry::ScalarFunction, ast::Expression, models::DataValue};


pub struct Table {
    evaluator: Arc<ExpressionEvaluator>,
}

impl Table {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}


impl ScalarFunction for Table {
    fn call(&self, context: &ExpressionEvaluationContext, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
        let mut result = Vec::new();
        for arg in args {
            let value = self.evaluator.evaluate_expression(context, arg)?;
            match value {
                DataValue::Table(t) => {
                    for record in t {
                        result.push(record);
                    }
                },
                DataValue::Record(r) => {
                    result.push(r);
                },
                _ => {
                    return Err(EvaluationError::InvalidArgument(format!("Expected a table or record, found {}", value)));
                }
            }
        }

        Ok(DataValue::Table(result))
    }
}