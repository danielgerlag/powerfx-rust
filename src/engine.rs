use std::process::Output;
use std::sync::Arc;

use crate::evaluator::{ExpressionEvaluator, EvaluationError, Session, GlobalVariables};
use crate::function_registry::{FunctionRegistry, ScalarFunction, Function};
use crate::{functions, lexer, models};
use crate::models::DataValue;


/// The PowerFxEngine is the main entry point for the PowerFx engine. It is responsible for evaluating expressions and managing the function registry.
pub struct PowerFxEngine {
    function_registry: Arc<FunctionRegistry>,
    evaluator: Arc<ExpressionEvaluator>,
}

impl PowerFxEngine {
    pub fn new() -> PowerFxEngine {
        let function_registry = Arc::new(FunctionRegistry::new());

        let evaluator = Arc::new(ExpressionEvaluator::new(function_registry.clone()));

        function_registry.register_function("Table", Function::Scalar(Arc::new(functions::table::Table::new(evaluator.clone()))));
        function_registry.register_function("First", Function::Scalar(Arc::new(functions::table::First::new(evaluator.clone()))));
        function_registry.register_function("Last", Function::Scalar(Arc::new(functions::table::Last::new(evaluator.clone()))));
        function_registry.register_function("Index", Function::Scalar(Arc::new(functions::table::Index::new(evaluator.clone()))));
        function_registry.register_function("Filter", Function::Scalar(Arc::new(functions::table::Filter::new(evaluator.clone()))));
        
        function_registry.register_function("Set", Function::Scalar(Arc::new(functions::context::Set::new(evaluator.clone()))));
        //function_registry.register_function("Set", Function::Scalar(Arc::new(functions::context::::new(evaluator.clone()))));

        function_registry.register_function("If", Function::Scalar(Arc::new(functions::logical::If::new(evaluator.clone()))));
        function_registry.register_function("And", Function::Scalar(Arc::new(functions::logical::And::new(evaluator.clone()))));
        function_registry.register_function("Or", Function::Scalar(Arc::new(functions::logical::Or::new(evaluator.clone()))));
        function_registry.register_function("Not", Function::Scalar(Arc::new(functions::logical::Not::new(evaluator.clone()))));

        function_registry.register_function("Abs", Function::Scalar(Arc::new(functions::math::Abs::new(evaluator.clone()))));
        function_registry.register_function("Sqrt", Function::Scalar(Arc::new(functions::math::Sqrt::new(evaluator.clone()))));

        function_registry.register_function("Left", Function::Scalar(Arc::new(functions::text::Left::new(evaluator.clone()))));
        function_registry.register_function("Mid", Function::Scalar(Arc::new(functions::text::Mid::new(evaluator.clone()))));
        function_registry.register_function("Right", Function::Scalar(Arc::new(functions::text::Right::new(evaluator.clone()))));
        function_registry.register_function("Upper", Function::Scalar(Arc::new(functions::text::Upper::new(evaluator.clone()))));
        function_registry.register_function("Lower", Function::Scalar(Arc::new(functions::text::Lower::new(evaluator.clone()))));

        function_registry.register_function("Average", Function::Scalar(Arc::new(functions::aggregation::Average::new(evaluator.clone()))));
        function_registry.register_function("Sum", Function::Scalar(Arc::new(functions::aggregation::Sum::new(evaluator.clone()))));
        function_registry.register_function("Min", Function::Scalar(Arc::new(functions::aggregation::Min::new(evaluator.clone()))));
        function_registry.register_function("Max", Function::Scalar(Arc::new(functions::aggregation::Max::new(evaluator.clone()))));

        PowerFxEngine {
            function_registry,
            evaluator,
        }
    }

    /// Registers a scalar function with the engine. The function will be available for evaluation in expressions.
    pub fn register_scalar_function(&self, name: &str, factory: fn(Arc<ExpressionEvaluator>) -> Arc<dyn ScalarFunction>) {
        let function = factory(self.evaluator.clone());
        self.function_registry.register_function(name, Function::Scalar(function));
    }

    /// Evaluates the provided expression and returns the result.
    pub fn evaluate(&self, expression: &str, session: Option<&mut Session>) -> Result<DataValue, EvaluationError> {
        let expressions = match lexer::parse(expression) {
            Ok(e) => e,
            Err(e) => return Err(EvaluationError::ParseError(e.to_string())),
        };
        
        let ctx = match session {
            Some(c) => c,
            None => &mut Session::new(),
        };
                
        let mut result = models::DataValue::Blank;
        for expression in expressions {
            result = match self.evaluator.evaluate_expression(ctx, &expression) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
        }

        Ok(result)        
    }
}