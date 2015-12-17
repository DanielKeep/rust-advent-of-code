extern crate serde_json;

use serde_json::Value;

fn main() {
    let input = read_stdin();
    let input: Value = serde_json::from_str(&input).unwrap();

    let mut sum = 0.0;
    for_each_number(&input, &mut |v| sum += v);

    println!("sum: {}", sum);
}

fn for_each_number<F>(value: &Value, f: &mut F)
where F: FnMut(f64) {
    match *value {
        Value::F64(v) => f(v),
        Value::I64(v) => f(v as f64),
        Value::U64(v) => f(v as f64),

        Value::Array(ref values) => {
            for value in values {
                for_each_number(value, f);
            }
        },

        Value::Object(ref fields) => {
            for value in fields.values() {
                for_each_number(value, f);
            }
        },

        Value::Null
        | Value::Bool(_)
        | Value::String(_)
        => {
            // Do nothing.
        },
    }
}

fn read_stdin() -> String {
    use std::io::{Read, stdin};
    let mut s = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut s).unwrap();
    s
}
