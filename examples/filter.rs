extern crate powerfx;

use powerfx::{PowerFxEngine, Session};


fn main() {
    let engine = PowerFxEngine::new();
        
    let mut session = Session::new();
    _ = engine.evaluate("Set(table1, Table({ Name: 'Foo', Age: 30 }, { Name: 'Baz', Age: 25 }, { Name: 'Bar', Age: 43 }))", Some(&mut session));

    let result = engine.evaluate("Filter(table1, Age >= 29)", Some(&mut session)).unwrap();
    println!("{:?}", result);
}


