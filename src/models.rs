
use std::{sync::Arc, collections::BTreeMap, fmt::Display};

use chrono::NaiveDate;


#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
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

impl DataValue {
    pub fn is_blank(&self) -> bool {
        match self {
            DataValue::Blank => true,
            _ => false,
        }
    }

    pub fn is_not_blank(&self) -> bool {
        !self.is_blank()
    }
}

impl Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataValue::Date(d) => write!(f, "{}", d),
            DataValue::Number(n) => write!(f, "{}", n),
            DataValue::Boolean(b) => write!(f, "{}", b),
            DataValue::OptionSet(o) => write!(f, "{:?}", o),
            DataValue::Text(t) => write!(f, "{}", t),
            DataValue::Image(i) => write!(f, "{}", i),
            DataValue::Hyperlink(h) => write!(f, "{}", h),
            DataValue::Media(m) => write!(f, "{}", m),
            DataValue::Record(r) => write!(f, "{:?}", r),
            DataValue::Table(t) => write!(f, "{:?}", t),
            DataValue::Blank => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionSet {
    pub options: BTreeMap<i64, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub fields: BTreeMap<Arc<str>, DataValue>
}

impl Record {
    pub fn from(fields: Vec<(Arc<str>, DataValue)>) -> Record {
        Record { fields: fields.into_iter().collect::<BTreeMap<Arc<str>, DataValue>>() }
    }
}