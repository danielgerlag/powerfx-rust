
use std::{sync::Arc, collections::BTreeMap, fmt::Display};

use chrono::NaiveDate;


#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Date(NaiveDate),
    Number(f64),
    Boolean(bool),
    OptionSet(OptionSet),
    Text(Arc<str>),
    Image(Arc<str>),
    Hyperlink(Arc<str>),
    Media(Arc<str>),
    Record(Record),
    Table(Vec<Record>),
    Blank,
}

impl Literal {
    pub fn is_blank(&self) -> bool {
        match self {
            Literal::Blank => true,
            _ => false,
        }
    }

    pub fn is_not_blank(&self) -> bool {
        !self.is_blank()
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Date(d) => write!(f, "{}", d),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::OptionSet(o) => write!(f, "{:?}", o),
            Literal::Text(t) => write!(f, "{}", t),
            Literal::Image(i) => write!(f, "{}", i),
            Literal::Hyperlink(h) => write!(f, "{}", h),
            Literal::Media(m) => write!(f, "{}", m),
            Literal::Record(r) => write!(f, "{:?}", r),
            Literal::Table(t) => write!(f, "{:?}", t),
            Literal::Blank => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionSet {
    pub options: BTreeMap<i64, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub fields: BTreeMap<Arc<str>, Expression>
}

impl Record {
    pub fn from(fields: Vec<(Arc<str>, Expression)>) -> Record {
        Record { fields: fields.into_iter().collect::<BTreeMap<Arc<str>, Expression>>() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Context {
    Parent,
    Self_,
    ThisItem,
    ThisRecord,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
    FunctionExpression(FunctionExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryExpression {
    Not(Box<Expression>),
    IsBlank(Box<Expression>),
    IsNotBlank(Box<Expression>),
    Literal(Literal),
    Property { context: Context, key: Arc<str> },
    Parameter(Arc<str>),
    Identifier(Arc<str>),    
    Alias { source: Box<Expression>, alias: Arc<str> },
}

impl UnaryExpression {
    pub fn literal(value: Literal) -> Expression {
        Expression::UnaryExpression(UnaryExpression::Literal(value))
    }

    pub fn alias(source: Expression, alias: Arc<str>) -> Expression {
        Expression::UnaryExpression(Self::Alias { source: Box::new(source), alias })
    }
    
    pub fn parameter(name: Arc<str>) -> Expression {
        Expression::UnaryExpression(UnaryExpression::Parameter(name))
    }
    
    pub fn property(context: Context, key: Arc<str>) -> Expression {
        Expression::UnaryExpression(UnaryExpression::Property { context, key })
    }

    pub fn not(cond: Expression) -> Expression {
        Expression::UnaryExpression(Self::Not(Box::new(cond)))
    }

    pub fn ident(ident: Arc<str>) -> Expression {
        Expression::UnaryExpression(Self::Identifier(ident))
    }

    pub fn is_blank(expr: Expression) -> Expression {
        Expression::UnaryExpression(Self::IsBlank(Box::new(expr)))
    }

    pub fn is_not_blank(expr: Expression) -> Expression {
        Expression::UnaryExpression(Self::IsNotBlank(Box::new(expr)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryExpression {
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    
    Eq(Box<Expression>, Box<Expression>),
    Ne(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
    Le(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Ge(Box<Expression>, Box<Expression>),
    In(Box<Expression>, Box<Expression>, bool),
    
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    Exponent(Box<Expression>, Box<Expression>),

}

impl BinaryExpression {
    pub fn and(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::And(Box::new(a), Box::new(b)))
    }

    pub fn or(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Or(Box::new(a), Box::new(b)))
    }

    pub fn eq(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Eq(Box::new(a), Box::new(b)))
    }

    pub fn ne(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Ne(Box::new(a), Box::new(b)))
    }

    pub fn lt(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Lt(Box::new(a), Box::new(b)))
    }

    pub fn le(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Le(Box::new(a), Box::new(b)))
    }

    pub fn gt(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Gt(Box::new(a), Box::new(b)))
    }

    pub fn in_(a: Expression, b: Expression, exact: bool) -> Expression {
        Expression::BinaryExpression(Self::In(Box::new(a), Box::new(b), exact))
    }

    pub fn ge(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Ge(Box::new(a), Box::new(b)))
    }

    pub fn add(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Add(Box::new(a), Box::new(b)))
    }

    pub fn subtract(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Subtract(Box::new(a), Box::new(b)))
    }

    pub fn multiply(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Multiply(Box::new(a), Box::new(b)))
    }

    pub fn divide(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Divide(Box::new(a), Box::new(b)))
    }

    pub fn modulo(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Modulo(Box::new(a), Box::new(b)))
    }

    pub fn exponent(a: Expression, b: Expression) -> Expression {
        Expression::BinaryExpression(Self::Exponent(Box::new(a), Box::new(b)))
    }

}


#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpression {
    pub name: Arc<str>, 
    pub args: Vec<Expression>,
}

impl FunctionExpression {
  pub fn function(name: Arc<str>, args: Vec<Expression>) -> Expression {
    Expression::FunctionExpression(FunctionExpression{ name, args })
  }
}