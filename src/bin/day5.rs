#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::{BufRead, stdin};
use regex::Regex;

lazy_static! {
    static ref RE_VOWELS: Regex = Regex::new(r#"(?xi)
        # Contains at least three vowels.
        ^[^aeiou]*?[aeiou][^aeiou]*?[aeiou][^aeiou]*?[aeiou]
    "#).unwrap();

    static ref RE_NAUGHTY: Regex = Regex::new(r#"(?xi)
        ab | cd | pq | xy
    "#).unwrap();
}

fn main() {
    let stdin = stdin();
    let nice_strings = stdin.lock().lines()
        .filter_map(Result::ok)
        .map(|line| is_nice(&line))
        .fold(0, |count, is_nice| if is_nice { count + 1 } else { count });

    println!("nice strings: {}", nice_strings);
}

fn is_nice(s: &str) -> bool {
    let s = s.trim();

    if !RE_VOWELS.is_match(s) {
        return false;
    }

    if RE_NAUGHTY.is_match(s) {
        return false;
    }

    for (c0, c1) in s.chars().zip(s.chars().skip(1)) {
        if c0 == c1 {
            return true;
        }
    }

    false
}
