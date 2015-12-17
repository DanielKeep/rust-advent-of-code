extern crate clap;
extern crate serde_json;

use serde_json::Value;

fn main() {
    let ignore_red = args();

    let input = read_stdin();
    let input: Value = serde_json::from_str(&input).unwrap();

    let mut sum = 0.0;
    for_each_number(&input, ignore_red, &mut |v| sum += v);

    println!("sum: {}", sum);
}

fn args() -> bool {
    let matches = clap::App::new("day12")
        .args_from_usage("\
            -i --ignore-red 'Ignore objects with `red` property'\
        ")
        .get_matches();

    let ignore_red = matches.is_present("ignore-red");

    ignore_red
}

fn for_each_number<F>(value: &Value, ignore_red: bool, f: &mut F)
where F: FnMut(f64) {
    match *value {
        Value::F64(v) => f(v),
        Value::I64(v) => f(v as f64),
        Value::U64(v) => f(v as f64),

        Value::Array(ref values) => {
            for value in values {
                for_each_number(value, ignore_red, f);
            }
        },

        Value::Object(ref fields) => {
            if {
                !fields.values()
                    .any(|v| match *v {
                        Value::String(ref s) => &**s == "red",
                        _ => false
                    })
            } {
                for value in fields.values() {
                    for_each_number(value, ignore_red, f);
                }
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
