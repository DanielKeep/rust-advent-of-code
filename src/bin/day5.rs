#[macro_use] extern crate lazy_static;
extern crate clap;
extern crate regex;
extern crate strcursor;

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
    let new_rules = args();

    let stdin = stdin();
    let nice_strings = stdin.lock().lines()
        .filter_map(Result::ok)
        .map(|line| is_nice(&line, new_rules))
        .fold(0, |count, is_nice| if is_nice { count + 1 } else { count });

    println!("nice strings: {}", nice_strings);
}

fn args() -> bool {
    let matches = clap::App::new("day5")
        .args_from_usage("\
            -n --new-rules 'Use new rules'\
        ")
        .get_matches();

    let new_rules = matches.is_present("new-rules");

    new_rules
}

fn is_nice(s: &str, new_rules: bool) -> bool {
    let s = s.trim();

    if new_rules {
        if !(|| {
            use strcursor::StrCursor;

            let mut s = &s[..];
            while s.len() >= 4 {
                let mut cur = StrCursor::new_at_start(s);
                cur.seek_next();
                let s_next = cur.slice_after();
                cur.seek_next();
                let head = cur.slice_before();
                if cur.slice_after().contains(head) {
                    return true;
                } else {
                    s = s_next;
                }
            }

            false
        })() {
            return false;
        }

        for (c0, c2) in s.chars().zip(s.chars().skip(2)) {
            if c0 == c2 {
                return true;
            }
        }

        false
    } else {
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
}
