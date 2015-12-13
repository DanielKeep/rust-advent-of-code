use std::io::{Read, stdin};

fn main() {
    let floor = stdin().bytes()
        .filter_map(|b| match b {
            Ok(b'(') => Some(1),
            Ok(b')') => Some(-1),
            _ => None
        })
        .fold(0, |a, b| a + b);

    println!("floor: {}", floor);
}
