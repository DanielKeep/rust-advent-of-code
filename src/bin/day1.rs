use std::io::{Read, stdin};

fn main() {
    let (floor, basement) = stdin().bytes()
        .filter_map(|b| match b {
            Ok(b'(') => Some(1),
            Ok(b')') => Some(-1),
            _ => None
        })
        .enumerate()
        .fold((0, None), |(a, p), (i, b)| {
            let f = a + b;
            (f, if f == -1 { p.or(Some(i+1)) } else { p })
        });

    println!("floor: {}", floor);
    println!("position: {:?}", basement);
}
