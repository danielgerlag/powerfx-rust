use crate::ast::{self, Expression};
use crate::evaluator::{EvaluationError, ExpressionEvaluator};

use crate::models::DataValue;

use crate::evaluator::Session;

use crate::function_registry::ScalarFunction;

use std::sync::Arc;

pub struct Lower {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Lower {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Lower {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 1 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 1 arguments, found {}", args.len())));
        }

        let value = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Text(t) => t.to_lowercase(),
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected text, found {:?}", args[0]))),
        };

        Ok(DataValue::Text(value.into()))
    }
}

pub struct Upper {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Upper {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Upper {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 1 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 1 arguments, found {}", args.len())));
        }

        let value = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Text(t) => t.to_uppercase(),
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected text, found {:?}", args[0]))),
        };

        Ok(DataValue::Text(value.into()))
    }
}


pub struct Left {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Left {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Left {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments, found {}", args.len())));
        }

        let text = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Text(t) => t,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected text, found {:?}", args[0]))),
        };

        let count = match self.evaluator.evaluate_expression(context, &args[1])? {
            DataValue::Number(n) => n as usize,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected number, found {:?}", args[1]))),
        };

        let value = text.chars().take(count).collect::<String>();

        Ok(DataValue::Text(value.into()))
    }
}

pub struct Right {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Right {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Right {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments, found {}", args.len())));
        }

        let text = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Text(t) => t,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected text, found {:?}", args[0]))),
        };

        let count = match self.evaluator.evaluate_expression(context, &args[1])? {
            DataValue::Number(n) => n as usize,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected number, found {:?}", args[1]))),
        };

        let value = text.chars().rev().take(count).collect::<String>().chars().rev().collect::<String>();

        Ok(DataValue::Text(value.into()))
    }
}

pub struct Mid {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Mid {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Mid {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() < 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments or more, found {}", args.len())));
        }

        let text = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Text(t) => t,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected text, found {:?}", args[0]))),
        };

        let start = match self.evaluator.evaluate_expression(context, &args[1])? {
            DataValue::Number(n) => n as usize,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected number, found {:?}", args[1]))),
        };

        let count = if args.len() == 3 {
            match self.evaluator.evaluate_expression(context, &args[2])? {
                DataValue::Number(n) => n as usize,
                _ => return Err(EvaluationError::InvalidArgument(format!("Expected number, found {:?}", args[2]))),
            }
        } else {
            text.len()
        };

        let value = text.chars().skip(start - 1).take(count).collect::<String>();

        Ok(DataValue::Text(value.into()))
    }
}
