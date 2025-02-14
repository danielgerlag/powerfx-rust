extern crate powerfx;

use powerfx::PowerFxEngine;


fn main() {
    let engine = PowerFxEngine::new();
    let result = engine.evaluate("2 + 3", None).unwrap();
    println!("{:?}", result);
}
