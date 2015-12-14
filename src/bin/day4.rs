extern crate crypto;
extern crate time;

use std::io::{BufRead, stdin};
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let stdin = stdin();
    let key = stdin.lock().lines()
        .filter_map(|line| line.ok())
        .next()
        .unwrap_or(String::new());
    let key = key.trim();

    let (secs, soln) = time_secs(|| {
        for n in 1.. {
            if try(key, n) {
                return n
            }
        }
        panic!("could not find solution for key {:?}", key);
    });

    println!("solution: {} (took {} secs)", soln, secs);
}

fn time_secs<F, R>(f: F) -> (f64, R)
where F: FnOnce() -> R {
    let start = time::precise_time_s();
    let r = f();
    let end = time::precise_time_s();
    (end - start, r)
}

fn try(key: &str, n: u32) -> bool {
    let mut md5 = Md5::new();
    md5.input_str(key);
    let n_str = format!("{}", n);
    md5.input_str(&n_str);

    let hash = md5.result_str();
    hash.bytes().take(5).all(|b| b == b'0')
}
