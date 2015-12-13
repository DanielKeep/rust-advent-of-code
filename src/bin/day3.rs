use std::collections::HashSet;
use std::io::{Read, stdin};

fn main() {
    let moves = stdin().bytes()
        .filter_map(|b| match b {
            Ok(b'^') => Some((0, 1)),
            Ok(b'>') => Some((1, 0)),
            Ok(b'v') => Some((0, -1)),
            Ok(b'<') => Some((-1, 0)),
            _ => None
        });

    let mut field = HashSet::new();
    field.insert((0, 0));

    let mut pos = (0, 0);
    for move_ in moves {
        pos = (pos.0+move_.0, pos.1+move_.1);
        field.insert(pos);
    }

    let num_houses = field.len();
    println!("houses: {}", num_houses);
}
