use crate::ast::{self, Expression};
use crate::evaluator::{EvaluationError, ExpressionEvaluator};

use crate::models::DataValue;

use crate::evaluator::Session;

use crate::function_registry::ScalarFunction;

use std::sync::Arc;

pub struct If {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl If {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for If {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() < 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected at least 2 arguments, found {}", args.len())));
        }

        let condition = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Boolean(b) => b,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a boolean, found {:?}", args[0]))),
        };

        if condition {
            return self.evaluator.evaluate_expression(context, &args[1]);
        }

        if args.len() == 3 {
            return self.evaluator.evaluate_expression(context, &args[2]);
        }

        Ok(DataValue::Blank)
    }
}

pub struct And {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl And {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for And {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() < 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected at least 2 arguments, found {}", args.len())));
        }

        for arg in args {
            let value = match self.evaluator.evaluate_expression(context, arg)? {
                DataValue::Boolean(b) => b,
                _ => return Err(EvaluationError::InvalidArgument(format!("Expected a boolean, found {:?}", arg))),
            };

            if !value {
                return Ok(DataValue::Boolean(false));
            }
        }

        Ok(DataValue::Boolean(true))
    }
}

pub struct Or {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Or {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Or {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() < 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected at least 2 arguments, found {}", args.len())));
        }

        for arg in args {
            let value = match self.evaluator.evaluate_expression(context, arg)? {
                DataValue::Boolean(b) => b,
                _ => return Err(EvaluationError::InvalidArgument(format!("Expected a boolean, found {:?}", arg))),
            };

            if value {
                return Ok(DataValue::Boolean(true));
            }
        }

        Ok(DataValue::Boolean(false))
    }
}

pub struct Not {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Not {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Not {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 1 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 1 argument, found {}", args.len())));
        }

        let value = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Boolean(b) => !b,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a boolean, found {:?}", args[0]))),
        };

        Ok(DataValue::Boolean(value))
    }
}
