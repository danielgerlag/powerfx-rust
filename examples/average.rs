extern crate powerfx;

use powerfx::{PowerFxEngine, Session};


fn main() {
    let engine = PowerFxEngine::new();
        
    let mut session = Session::new();

    //Set the `table1` variable to a table with three records
    _ = engine.evaluate("Set(table1, Table({ Name: 'Foo', Age: 30 }, { Name: 'Baz', Age: 25 }, { Name: 'Bar', Age: 43 }))", Some(&mut session));

    //Calculate the average of the `Age` column in the `table1` table, using the session that was modified by the above statement
    let result = engine.evaluate("Average(table1, Age)", Some(&mut session)).unwrap();
    
    println!("{:?}", result);
}


