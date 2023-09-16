use std::{collections::BTreeMap, sync::Arc, ops::Deref};

use time::Date;

use crate::{ast, ast::Literal, function_registry::{FunctionRegistry, Function}, models::{DataValue, self}};



#[derive(Debug)]
pub enum EvaluationError {
    DivideByZero,
    InvalidType,
    UnknownIdentifier(String),
    UnknownFunction(String),
    InvalidArgument(String),
    InvalidArgumentCount(String),
    ParseError(String),
}


pub type GlobalVariables = BTreeMap<Arc<str>, DataValue>;

pub trait MyToString {
    fn to_string(&self) -> String;
}

impl MyToString for GlobalVariables {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for (key, value) in self {
            result.push_str(&format!("{}={}\n", key, value));
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionEvaluationContext {
  variables: GlobalVariables,
  
}

impl ExpressionEvaluationContext {

  pub fn new(variables: GlobalVariables) -> ExpressionEvaluationContext {
    ExpressionEvaluationContext {
        variables,
    }
  }

  pub fn replace_variables(&mut self, new_data: GlobalVariables) {
    self.variables = new_data;
  }

  pub fn get_variable(&self, name: &str) -> Option<&DataValue> {
    self.variables.get(name)
  }

  pub fn clone_variables(&self) -> GlobalVariables {
    self.variables.clone()
  }
  
}

pub struct ExpressionEvaluator {
    function_registry: Arc<FunctionRegistry>,
}

impl ExpressionEvaluator {

    pub fn new(function_registry: Arc<FunctionRegistry>) -> ExpressionEvaluator {
        ExpressionEvaluator {  
            function_registry,
        }
    }

    pub fn evaluate_expression(
        &self,
        context: &ExpressionEvaluationContext,
        expression: &ast::Expression,
    ) -> Result<DataValue, EvaluationError> {
        match expression {
            ast::Expression::UnaryExpression(expression) => {
                self.evaluate_unary_expression(context, expression)
            }
            ast::Expression::BinaryExpression(expression) => {
                self.evaluate_binary_expression(context, expression)
            }
            ast::Expression::FunctionExpression(func) => {
                self.evaluate_function_expression(context, func)
            },
            
        }
    }

    pub fn evaluate_predicate(
        &self,
        context: &ExpressionEvaluationContext,
        expression: &ast::Expression,
    ) -> Result<bool, EvaluationError> {
        let value = self.evaluate_expression(context, expression)?;
        match value {
            DataValue::Boolean(b) => Ok(b),
            _ => Ok(false),
        }
    }

    pub fn evaluate_projection_field(
        &self,
        context: &ExpressionEvaluationContext,
        expression: &ast::Expression,
    ) -> Result<(String, DataValue), EvaluationError> {
        let value = self.evaluate_expression(context, expression)?;
        let alias = match expression {
            ast::Expression::UnaryExpression(expression) => match expression {
                ast::UnaryExpression::Property { context: _, key } => key,
                ast::UnaryExpression::Parameter(p) => p,
                ast::UnaryExpression::Alias { source: _, alias } => alias,
                ast::UnaryExpression::Identifier(id) => id,
                _ => "expression",
            },
            ast::Expression::BinaryExpression(_) => "expression",
            ast::Expression::FunctionExpression(_) => "function",
        };

        Ok((alias.to_string(), value))
    }

    fn evaluate_unary_expression(
        &self,
        context: &ExpressionEvaluationContext,
        expression: &ast::UnaryExpression,
    ) -> Result<DataValue, EvaluationError> {
        let result = match expression {
            ast::UnaryExpression::Not(expression) => {
                DataValue::Boolean(!self.evaluate_predicate(context, expression)?)
            }
            ast::UnaryExpression::IsBlank(e) => DataValue::Boolean(self.evaluate_expression(context, e)?.is_blank()),
            ast::UnaryExpression::IsNotBlank(e) => DataValue::Boolean(!self.evaluate_expression(context, e)?.is_blank()),
            ast::UnaryExpression::Literal(l) => {
                match l {
                    Literal::Number(n) => DataValue::Number(*n),
                    Literal::Date(d) => DataValue::Date(*d),
                    Literal::Text(t) => DataValue::Text(t.clone()),
                    Literal::Hyperlink(h) => DataValue::Hyperlink(h.clone()),
                    Literal::Image(i) => DataValue::Image(i.clone()),
                    Literal::Media(m) => DataValue::Media(m.clone()),
                    Literal::Boolean(b) => DataValue::Boolean(*b),
                    Literal::Blank => DataValue::Blank,
                    Literal::Record(r) => {
                        let mut fields = BTreeMap::new();
                        for (key, value) in r.fields.iter() {
                            fields.insert(key.clone(), self.evaluate_expression(context, value)?);
                        }
                        DataValue::Record(models::Record{fields})
                    },
                    Literal::Table(t) => {
                        let mut records = Vec::new();
                        for record in t.iter() {
                            let mut fields = BTreeMap::new();
                            for (key, value) in record.fields.iter() {
                                fields.insert(key.clone(), self.evaluate_expression(context, value)?);
                            }
                            records.push(models::Record{fields});
                        }
                        DataValue::Table(records)
                    },
                    Literal::OptionSet(o) => {
                        let mut options = BTreeMap::new();
                        for (key, value) in o.options.iter() {
                            options.insert(*key, value.clone());
                        }
                        DataValue::OptionSet(models::OptionSet{options})
                    },
                }
            },
            ast::UnaryExpression::Property { context, key } => todo!(),            
            ast::UnaryExpression::Alias { source, alias: _ } => {
                self.evaluate_expression(context, source)?
            }
            ast::UnaryExpression::Identifier(ident) => match context.get_variable(ident) {
                Some(value) => value.clone(),
                None => return Err(EvaluationError::UnknownIdentifier(ident.to_string())),
            },
            ast::UnaryExpression::Parameter(name) => match context.get_variable(name) {
                Some(value) => value.clone(),
                None => return Err(EvaluationError::UnknownIdentifier(name.to_string())),
            },
        };
        Ok(result)
    }

    fn evaluate_binary_expression(
        &self,
        context: &ExpressionEvaluationContext,
        expression: &ast::BinaryExpression,
    ) -> Result<DataValue, EvaluationError> {
        let result = match expression {
            ast::BinaryExpression::And(c1, c2) => DataValue::Boolean(
                self.evaluate_predicate(context, c1)? && self.evaluate_predicate(context, c2)?,
            ),
            ast::BinaryExpression::Or(c1, c2) => DataValue::Boolean(
                self.evaluate_predicate(context, c1)? || self.evaluate_predicate(context, c2)?,
            ),
            ast::BinaryExpression::Eq(e1, e2) => match (
                self.evaluate_expression(context, e1)?,
                self.evaluate_expression(context, e2)?,
            ) {
                (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Boolean(n1 == n2),
                (DataValue::Date(n1), DataValue::Date(n2)) => DataValue::Boolean(n1 == n2),
                (DataValue::Text(s1), DataValue::Text(s2)) => DataValue::Boolean(s1 == s2),
                (DataValue::Hyperlink(s1), DataValue::Hyperlink(s2)) => DataValue::Boolean(s1 == s2),
                (DataValue::Image(s1), DataValue::Image(s2)) => DataValue::Boolean(s1 == s2),
                (DataValue::Media(s1), DataValue::Media(s2)) => DataValue::Boolean(s1 == s2),
                (DataValue::Boolean(b1), DataValue::Boolean(b2)) => DataValue::Boolean(b1 == b2),
                (DataValue::Blank, DataValue::Blank) => DataValue::Boolean(true),
                (DataValue::Record(r1), DataValue::Record(r2)) => DataValue::Boolean(r1 == r2),
                (DataValue::Table(t1), DataValue::Table(t2)) => DataValue::Boolean(t1 == t2),                
                _ => DataValue::Boolean(false),
            },
            ast::BinaryExpression::Ne(e1, e2) => match (
                self.evaluate_expression(context, e1)?,
                self.evaluate_expression(context, e2)?,
            ) {
                (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Boolean(n1 != n2),
                (DataValue::Date(n1), DataValue::Date(n2)) => DataValue::Boolean(n1 != n2),
                (DataValue::Text(s1), DataValue::Text(s2)) => DataValue::Boolean(s1 != s2),
                (DataValue::Hyperlink(s1), DataValue::Hyperlink(s2)) => DataValue::Boolean(s1 != s2),
                (DataValue::Image(s1), DataValue::Image(s2)) => DataValue::Boolean(s1 != s2),
                (DataValue::Media(s1), DataValue::Media(s2)) => DataValue::Boolean(s1 != s2),
                (DataValue::Boolean(b1), DataValue::Boolean(b2)) => DataValue::Boolean(b1 != b2),
                (DataValue::Blank, DataValue::Blank) => DataValue::Boolean(false),
                (DataValue::Record(r1), DataValue::Record(r2)) => DataValue::Boolean(r1 != r2),
                (DataValue::Table(t1), DataValue::Table(t2)) => DataValue::Boolean(t1 != t2),                
                _ => DataValue::Boolean(true),
            },
            ast::BinaryExpression::Lt(e1, e2) => match (
                self.evaluate_expression(context, e1)?,
                self.evaluate_expression(context, e2)?,
            ) {
                (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Boolean(n1 < n2),                
                (DataValue::Date(n1), DataValue::Date(n2)) => DataValue::Boolean(n1 < n2),
                _ => DataValue::Boolean(false),
            },
            ast::BinaryExpression::Le(e1, e2) => match (
                self.evaluate_expression(context, e1)?,
                self.evaluate_expression(context, e2)?,
            ) {
                (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Boolean(n1 <= n2),
                (DataValue::Date(n1), DataValue::Date(n2)) => DataValue::Boolean(n1 <= n2),
                _ => DataValue::Boolean(false),
            },
            ast::BinaryExpression::Gt(e1, e2) => match (
                self.evaluate_expression(context, e1)?,
                self.evaluate_expression(context, e2)?,
            ) {
                (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Boolean(n1 > n2),
                (DataValue::Date(n1), DataValue::Date(n2)) => DataValue::Boolean(n1 > n2),
                _ => DataValue::Boolean(false),
            },
            ast::BinaryExpression::Ge(e1, e2) => match (
                self.evaluate_expression(context, e1)?,
                self.evaluate_expression(context, e2)?,
            ) {
                (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Boolean(n1 >= n2),
                (DataValue::Date(n1), DataValue::Date(n2)) => DataValue::Boolean(n1 >= n2),
                _ => DataValue::Boolean(false),
            },
            ast::BinaryExpression::Add(e1, e2) => {
                let n1 = self.evaluate_expression(context, e1)?;
                let n2 = self.evaluate_expression(context, e2)?;
                match (n1, n2) {
                    (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Number(n1 + n2),
                    (DataValue::Number(n1), DataValue::Text(s2)) => DataValue::Text(Arc::from(n1.to_string() + &s2)),
                    (DataValue::Text(s1), DataValue::Boolean(b2)) => DataValue::Text(Arc::from(s1.to_string() + &b2.to_string())),
                    (DataValue::Text(s1), DataValue::Number(n2)) => DataValue::Text(Arc::from(s1.to_string() + &n2.to_string())),
                    (DataValue::Text(s1), DataValue::Text(s2)) => DataValue::Text(Arc::from(s1.to_string() + &s2)),
                    _ => DataValue::Blank,
                }
            }
            ast::BinaryExpression::Subtract(e1, e2) => {
                let n1 = self.evaluate_expression(context, e1)?;
                let n2 = self.evaluate_expression(context, e2)?;
                match (n1, n2) {
                    (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Number(n1 - n2),                    
                    _ => DataValue::Blank,
                }
            }
            ast::BinaryExpression::Multiply(e1, e2) => {
                let n1 = self.evaluate_expression(context, e1)?;
                let n2 = self.evaluate_expression(context, e2)?;
                match (n1, n2) {
                    (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Number(n1 * n2),                    
                    _ => DataValue::Blank,
                }
            }
            ast::BinaryExpression::Divide(e1, e2) => {
                let n1 = self.evaluate_expression(context, e1)?;
                let n2 = self.evaluate_expression(context, e2)?;
                match (n1, n2) {
                    (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Number(n1 / n2),                    
                    _ => DataValue::Blank,
                }
            }
            ast::BinaryExpression::In(e1, e2, exact) => {
                let e1 = self.evaluate_expression(context, e1)?;
                let e2 = self.evaluate_expression(context, e2)?;
                
                if *exact {
                    match (e1, e2) {
                        (DataValue::Text(s1), DataValue::Text(s2)) => DataValue::Boolean(s2.contains(s1.deref())),
                        
                        _ => return Err(EvaluationError::InvalidType),
                    }
                } else {
                    match (e1, e2) {
                        (DataValue::Text(s1), DataValue::Text(s2)) => {
                            let s1 = s1.to_lowercase();
                            let s2 = s2.to_lowercase();
                            DataValue::Boolean(s2.contains(s1.deref()))
                        },
                        _ => return Err(EvaluationError::InvalidType),
                    }
                }
                
            },
            ast::BinaryExpression::Modulo(e1, e2) => {
                let n1 = self.evaluate_expression(context, e1)?;
                let n2 = self.evaluate_expression(context, e2)?;
                match (n1, n2) {
                    (DataValue::Number(n1), DataValue::Number(n2)) => DataValue::Number(n1 % n2),
                    _ => DataValue::Blank,
                }
            },
            ast::BinaryExpression::Exponent(e1, e2) => {
                let n1 = self.evaluate_expression(context, e1)?;
                let n2 = self.evaluate_expression(context, e2)?;
                todo!()
            },
        };
        Ok(result)
    }

    fn evaluate_function_expression(
        &self,
        context: &ExpressionEvaluationContext,
        expression: &ast::FunctionExpression,
    ) -> Result<DataValue, EvaluationError> {
        
        let result = match self.function_registry.get_function(&expression.name) {
            Some(function) => match function.as_ref() {
                Function::Scalar(scalar) => scalar.call(context, &expression.args)?,
            },
            None => {
                return Err(EvaluationError::UnknownFunction(
                    expression.name.to_string(),
                ))
            }
        };

        Ok(result)
    }

}
