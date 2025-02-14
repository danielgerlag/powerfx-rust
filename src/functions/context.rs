use crate::ast::{self, Expression};
use crate::evaluator::{EvaluationError, ExpressionEvaluator};

use crate::models::DataValue;

use crate::evaluator::Session;

use crate::function_registry::ScalarFunction;

use std::sync::Arc;

pub struct Set {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Set {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Set {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments, found {}", args.len())));
        }

        let var_name = match &args[0] {
            Expression::UnaryExpression(u) => match &*u {
                ast::UnaryExpression::Identifier(name) => name.clone(),
                _ => return Err(EvaluationError::InvalidArgument(format!("Expected an identifier, found {:?}", args[0]))),
            },
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected an identifier, found {:?}", args[0]))),
        };

        let value = self.evaluator.evaluate_expression(context, &args[1])?;

        context.set_variable(&var_name, value);
   

        Ok(DataValue::Blank)
    }
}
