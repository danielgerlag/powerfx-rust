use crate::ast::{self, Expression};
use crate::evaluator::{EvaluationError, ExpressionEvaluator};

use crate::models::DataValue;

use crate::evaluator::Session;

use crate::function_registry::ScalarFunction;

use std::sync::Arc;

pub struct Average {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Average {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Average {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() == 0 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected at least 1 arguments, found {}", args.len())));
        }

        let mut sum = 0.0;
        let mut count = 0;

        if let DataValue::Table(table) = self.evaluator.evaluate_expression(context, &args[0])? {
            if args.len() != 2 {
                return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments, found {}", args.len())));
            }
            let expr = &args[1];
            for row in &table {
                let mut session = Session::from_record_with_context(row, &context);
                let value = match self.evaluator.evaluate_expression(&mut session, expr)? {
                    DataValue::Number(n) => n,
                    _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", expr))),
                };
                sum += value;
                count += 1;
            }

            return Ok(DataValue::Number(sum / count as f64));            
        }

        for arg in args {
            let value = match self.evaluator.evaluate_expression(context, arg)? {
                DataValue::Number(n) => n,
                _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", arg))),
            };

            sum += value;
            count += 1;
        }

        Ok(DataValue::Number(sum / count as f64))
    }
}


pub struct Sum {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Sum {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Sum {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() == 0 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected at least 1 arguments, found {}", args.len())));
        }

        let mut sum = 0.0;

        if let DataValue::Table(table) = self.evaluator.evaluate_expression(context, &args[0])? {
            if args.len() != 2 {
                return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments, found {}", args.len())));
            }
            let expr = &args[1];
            for row in &table {
                let mut session = Session::from_record_with_context(row, &context);
                let value = match self.evaluator.evaluate_expression(&mut session, expr)? {
                    DataValue::Number(n) => n,
                    _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", expr))),
                };
                sum += value;
            }

            return Ok(DataValue::Number(sum));            
        }

        for arg in args {
            let value = match self.evaluator.evaluate_expression(context, arg)? {
                DataValue::Number(n) => n,
                _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", arg))),
            };

            sum += value;
        }

        Ok(DataValue::Number(sum))
    }
}


pub struct Min {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Min {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Min {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() == 0 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected at least 1 arguments, found {}", args.len())));
        }

        let mut result = None;

        if let DataValue::Table(table) = self.evaluator.evaluate_expression(context, &args[0])? {
            if args.len() != 2 {
                return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments, found {}", args.len())));
            }
            let expr = &args[1];
            for row in &table {
                let mut session = Session::from_record_with_context(row, &context);
                let value = match self.evaluator.evaluate_expression(&mut session, expr)? {
                    DataValue::Number(n) => n,
                    _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", expr))),
                };

                result = match result {
                    Some(r) => {
                        if value < r {
                            Some(value)
                        } else {
                            Some(r)
                        }
                    },
                    None => Some(value),
                };
            }

            return Ok(match result {
                Some(r) => DataValue::Number(r),
                None => DataValue::Blank,
            });            
        }

        for arg in args {
            let value = match self.evaluator.evaluate_expression(context, arg)? {
                DataValue::Number(n) => n,
                _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", arg))),
            };

            result = match result {
                Some(r) => {
                    if value < r {
                        Some(value)
                    } else {
                        Some(r)
                    }
                },
                None => Some(value),
            };
        }

        Ok(match result {
            Some(r) => DataValue::Number(r),
            None => DataValue::Blank,
        })
    }
}

pub struct Max {
    pub(crate) evaluator: Arc<ExpressionEvaluator>,
}

impl Max {
    pub fn new(evaluator: Arc<ExpressionEvaluator>) -> Self {
        Self {
            evaluator,
        }
    }
}

impl ScalarFunction for Max {
    fn call(&self, context: &mut Session, args: &Vec<Expression>) -> Result<DataValue, EvaluationError> {
    
        if args.len() == 0 {
            return Err(EvaluationError::InvalidArgumentCount(format!("Expected at least 1 arguments, found {}", args.len())));
        }

        let mut result = None;

        if let DataValue::Table(table) = self.evaluator.evaluate_expression(context, &args[0])? {
            if args.len() != 2 {
                return Err(EvaluationError::InvalidArgumentCount(format!("Expected 2 arguments, found {}", args.len())));
            }
            let expr = &args[1];
            for row in &table {
                let mut session = Session::from_record_with_context(row, &context);
                let value = match self.evaluator.evaluate_expression(&mut session, expr)? {
                    DataValue::Number(n) => n,
                    _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", expr))),
                };
                result = match result {
                    Some(r) => {
                        if value > r {
                            Some(value)
                        } else {
                            Some(r)
                        }
                    },
                    None => Some(value),
                };
            }

            return Ok(match result {
                Some(r) => DataValue::Number(r),
                None => DataValue::Blank,
            });            
        }

        for arg in args {
            let value = match self.evaluator.evaluate_expression(context, arg)? {
                DataValue::Number(n) => n,
                _ => return Err(EvaluationError::InvalidArgument(format!("Expected a number, found {:?}", arg))),
            };

            result = match result {
                Some(r) => {
                    if value > r {
                        Some(value)
                    } else {
                        Some(r)
                    }
                },
                None => Some(value),
            };
        }

        Ok(match result {
            Some(r) => DataValue::Number(r),
            None => DataValue::Blank,
        })
    }
}
