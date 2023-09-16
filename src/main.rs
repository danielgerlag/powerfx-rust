use std::sync::Arc;

use evaluator::{ExpressionEvaluator, EvaluationError, ExpressionEvaluationContext, GlobalVariables};
use function_registry::{FunctionRegistry, ScalarFunction, Function};
use models::DataValue;

pub mod ast;
pub mod lexer;
pub mod function_registry;
pub mod evaluator;
pub mod models;
pub mod functions;

fn main() {
    let engine = PowerFxEngine::new();
    let result = engine.evaluate("Table({ Name: 'Foo', Age: 42 }, { Name: 'Bar', Age: 43 })");
    println!("{:?}", result);
}


struct PowerFxEngine {
    function_registry: Arc<FunctionRegistry>,
    evaluator: Arc<ExpressionEvaluator>,
}

impl PowerFxEngine {
    pub fn new() -> PowerFxEngine {
        let function_registry = Arc::new(FunctionRegistry::new());

        let evaluator = Arc::new(ExpressionEvaluator::new(function_registry.clone()));

        function_registry.register_function("Table", Function::Scalar(Arc::new(functions::Table::new(evaluator.clone()))));

        PowerFxEngine {
            function_registry,
            evaluator,
        }
    }

    pub fn register_function(&self, name: &str, function: Function) {
        self.function_registry.register_function(name, function);
    }

    pub fn evaluate(&self, expression: &str) -> Result<DataValue, EvaluationError> {
        let expressions = match lexer::parse(expression) {
            Ok(e) => e,
            Err(e) => return Err(EvaluationError::ParseError(e.to_string())),
        };
        
        let variables = GlobalVariables::new();
        let mut context = ExpressionEvaluationContext::new(variables);
        
        let mut result = models::DataValue::Blank;
        for expression in expressions {
            result = match self.evaluator.evaluate_expression(&mut context, &expression) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
        }

        Ok(result)        
    }
}