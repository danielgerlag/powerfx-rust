use crate::ast::{self, Expression};
use crate::evaluator::{EvaluationError, ExpressionEvaluator};

use crate::models::DataValue;

use crate::evaluator::Session;

use crate::function_registry::ScalarFunction;

use std::sync::Arc;

pub struct Abs {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Abs {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Abs {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 1 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 1 arguments, found {}", args.len())));
        }

        let value = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Number(n) => n.abs(),
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", args[0]))),
        };

        Ok(DataValue::Number(value))
    }
}

pub struct Sqrt {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Sqrt {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Sqrt {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 1 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 1 arguments, found {}", args.len())));
        }

        let value = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Number(n) => n.sqrt(),
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", args[0]))),
        };

        Ok(DataValue::Number(value))
    }
}
