extern crate clap;

const DEFAULT_ROUNDS: u32 = 40;

fn main() {
    let rounds = args();
    let mut num = read_stdin().lines().next().map(String::from).unwrap();

    for _ in 0..rounds {
        num = look_and_say(&num);
    }

    println!("result after {} rounds: {}", rounds, num);
    println!("length: {}", num.len());
}

fn args() -> u32 {
    let matches = clap::App::new("day10")
        .args_from_usage("\
            -r --rounds=[ROUNDS] 'Specifies the number of rounds'\
        ")
        .get_matches();

    let rounds = matches.value_of("ROUNDS")
        .map(|s| s.parse().unwrap())
        .unwrap_or(DEFAULT_ROUNDS);
    rounds
}

fn look_and_say(mut n: &str) -> String {
    use std::fmt::Write;

    assert!(n.len() > 0);
    assert!(n.bytes().all(|b| b'0' <= b && b <= b'9'));

    let mut r = String::new();

    while let Some(lead) = n.chars().next() {
        let count = n.chars().take_while(|c| *c == lead).count();
        write!(r, "{}{}", count, lead).unwrap();
        n = &n[count..];
    }

    r
}

fn read_stdin() -> String {
    use std::io::{Read, stdin};
    let mut s = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut s).unwrap();
    s
}
