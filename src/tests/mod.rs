use crate::{engine::PowerFxEngine, evaluator::Session, models::{DataValue, Record}};


#[test]
fn arithmetic() {
    let engine = PowerFxEngine::new();
    
    let mut session = Session::new();
    session.set_variable("a", DataValue::Number(2.0));
    session.set_variable("b", DataValue::Number(3.0));

    let result = engine.evaluate("a + b", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(5.0));

    let result = engine.evaluate("b - a", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(1.0));

    let result = engine.evaluate("a * b", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(6.0));

    let result = engine.evaluate("b / a", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(1.5));
}

#[test]
fn aggregation() {
    let engine = PowerFxEngine::new();
    
    let mut session = Session::new();
    session.set_variable("a", DataValue::Number(3.0));
    session.set_variable("b", DataValue::Number(7.0));
    session.set_variable("c", DataValue::Number(5.0));

    let result = engine.evaluate("Min(a, b, c)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(3.0));

    let result = engine.evaluate("Max(a, b, c)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(7.0));

    let result = engine.evaluate("Sum(a, b, c)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(15.0));

    let result = engine.evaluate("Average(a, b, c)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(5.0));    
}

#[test]
fn aggregation_with_tables() {
    let engine = PowerFxEngine::new();
    
    let mut session = Session::new();
    _ = engine.evaluate("Set(table1, Table({ Name: 'Foo', Age: 30 }, { Name: 'Bar', Age: 43 }))", Some(&mut session));

    let result = engine.evaluate("Min(table1, Age)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(30.0));

    let result = engine.evaluate("Max(table1, Age)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(43.0));

    let result = engine.evaluate("Sum(table1, Age)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(73.0));

    let result = engine.evaluate("Average(table1, Age)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(36.5));

    let result = engine.evaluate("Average(table1, Age + 10)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Number(46.5));
}


#[test]
fn tables() {
    let engine = PowerFxEngine::new();
    
    let mut session = Session::new();
    _ = engine.evaluate("Set(table1, Table({ Name: 'Foo', Age: 30 }, { Name: 'Baz', Age: 25 }, { Name: 'Bar', Age: 43 }))", Some(&mut session));

    let result = engine.evaluate("Filter(table1, Age >= 29)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Table(vec![
        Record::from(vec![
            ("Name".into(), DataValue::Text("Foo".into())),
            ("Age".into(), DataValue::Number(30.0)),
        ]),
        Record::from(vec![
            ("Name".into(), DataValue::Text("Bar".into())),
            ("Age".into(), DataValue::Number(43.0)),
        ]),
    ]));

    let result = engine.evaluate("First(table1)", Some(&mut session)).unwrap();
    assert_eq!(result, DataValue::Record(Record::from(vec![
        ("Name".into(), DataValue::Text("Foo".into())),
        ("Age".into(), DataValue::Number(30.0)),
    ])));
}


