use crate::function_registry::ScalarFunction;
use crate::models::DataValue;

use crate::ast::Expression;

use crate::evaluator::{EvaluationError, ExpressionEvaluator, Session};

use std::sync::Arc;

pub struct Table {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Table {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Table {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
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

pub struct First {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl First {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for First {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 1 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 1 argument, found {}", args.len())));
        }

        let table = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Table(t) => t,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a table, found {:?}", args[0]))),
        };

        if table.is_empty() {
            return Ok(DataValue::Blank);
        }

        Ok(DataValue::Record(table[0].clone()))
    }
}

pub struct Last {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Last {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Last {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 1 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 1 argument, found {}", args.len())));
        }

        let table = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Table(t) => t,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a table, found {:?}", args[0]))),
        };

        if table.is_empty() {
            return Ok(DataValue::Blank);
        }

        Ok(DataValue::Record(table[table.len() - 1].clone()))
    }
}

pub struct Index {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Index {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Index {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() != 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments, found {}", args.len())));
        }

        let table = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Table(t) => t,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a table, found {:?}", args[0]))),
        };

        let index = match self.evaluator.evaluate_expression(context, &args[1])? {
            DataValue::Number(n) => n as usize,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", args[1]))),
        };

        if index >= table.len() {
            return Ok(DataValue::Blank);
        }

        Ok(DataValue::Record(table[index].clone()))            
    }
}

pub struct Filter {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Filter {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Filter {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() < 2 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected at least 2 arguments, found {}", args.len())));
        }

        let table = match self.evaluator.evaluate_expression(context, &args[0])? {
            DataValue::Table(t) => t,
            _ => return Err(EvaluationError::InvalidArgument(format!("Expected a table, found {:?}", args[0]))),
        };

        let mut result = Vec::new();

        for record in table {
            let mut and_result = true;
            let mut session = Session::from_record_with_context(&record, &context);
            for arg in &args[1..] {
                let condition = match self.evaluator.evaluate_expression(&mut session, arg)? {
                    DataValue::Boolean(b) => b,
                    _ => return Err(EvaluationError::InvalidArgument(format!("Expected a boolean, found {:?}", arg))),
                };

                if !condition {
                    and_result = false;
                    break;
                }
            }
            if and_result {
                result.push(record.clone());
            }
        }

        Ok(DataValue::Table(result))
    }
}
