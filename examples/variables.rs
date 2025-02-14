extern crate powerfx;

use powerfx::{DataValue, PowerFxEngine, Session};


fn main() {
    let engine = PowerFxEngine::new();
    let mut session = Session::new();
    session.set_variable("a", DataValue::Number(2.0));
    session.set_variable("b", DataValue::Number(3.0));

    let result = engine.evaluate("a + b", Some(&mut session)).unwrap();
    println!("{:?}", result);
}
