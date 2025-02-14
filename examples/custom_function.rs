extern crate powerfx;

use std::sync::Arc;

use powerfx::{DataValue, EvaluationError, Expression, ExpressionEvaluator, PowerFxEngine, ScalarFunction, Session};


fn main() {
    let engine = PowerFxEngine::new();
    
    // Register the custom function
    engine.register_scalar_function("MyFunction", |evaluator| MyFunction::new(evaluator));

    let result = engine.evaluate("MyFunction(2, 3)", None).unwrap();
    println!("{:?}", result);
}

// Define the custom function
pub struct MyFunction {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl MyFunction {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Arc<Self> {
        Arc::new(Self {
            evaluator,
        })
    }
}

impl ScalarFunction for MyFunction {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments, found {}", args.len())));
        }

        let value1 = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Number(n) => n,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", args[0]))),
        };

        let value2 = match self.evaluator.evaluate_expression(context, &args[1])? {
            DataValue::Number(n) => n,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", args[1]))),
        };

        Ok(DataValue::Number(value1 + value2))
    }
}