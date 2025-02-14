# Power Fx interpreter for Rust

This crate provides an embedded [Power Fx](https://learn.microsoft.com/en-us/power-platform/power-fx/overview) interpreter for Rust projects.

## Getting started

Install the package.

```shell
cargo add powerfx
```

## Status

This library is still in an alpha status.  The following functions have been implemented:

- Table
- First
- Last
- Index
- Filter
- Set
- If
- And
- Or
- Not
- Abs
- Sqrt
- Left
- Mid
- Right
- Upper
- Lower
- Average
- Sum
- Min
- Max

## Examples

The following example illustrates adding two constant numbers.

```rust
use powerfx::{DataValue, PowerFxEngine};

fn main() {
    let engine = PowerFxEngine::new();
    let result = engine.evaluate("2 + 3", None).unwrap();
    assert_eq!(result, DataValue::Number(5.0));
}
```

This can also be done with variables.

```rust
let engine = PowerFxEngine::new();
let mut session = Session::new();
session.set_variable("a", DataValue::Number(2.0));
session.set_variable("b", DataValue::Number(3.0));

let result = engine.evaluate("a + b", Some(&mut session)).unwrap();
assert_eq!(result, DataValue::Number(5.0));
```

For more examples, please see the [Examples Folder](./examples/)
