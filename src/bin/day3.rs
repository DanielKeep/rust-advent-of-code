use std::collections::HashSet;
use std::io::{Read, stdin};

fn main() {
    let moves: Vec<_> = stdin().bytes()
        .filter_map(|b| match b {
            Ok(b'^') => Some((0, 1)),
            Ok(b'>') => Some((1, 0)),
            Ok(b'v') => Some((0, -1)),
            Ok(b'<') => Some((-1, 0)),
            _ => None
        })
        .collect();

    println!("houses (part 1): {}", part_1(&moves));
    println!("houses (part 2): {}", part_2(&moves));
}

fn part_1(moves: &[(i32, i32)]) -> usize {
    let mut field = HashSet::new();
    field.insert((0, 0));

    let mut pos = (0, 0);
    for &move_ in moves {
        pos = (pos.0+move_.0, pos.1+move_.1);
        field.insert(pos);
    }

    field.len()
}

fn part_2(moves: &[(i32, i32)]) -> usize {
    let mut field = HashSet::new();
    field.insert((0, 0));

    let mut pos_cur = (0, 0);
    let mut pos_oth = (0, 0);
    for &move_ in moves {
        pos_cur = (pos_cur.0+move_.0, pos_cur.1+move_.1);
        field.insert(pos_cur);
        std::mem::swap(&mut pos_cur, &mut pos_oth);
    }

    field.len()
}
