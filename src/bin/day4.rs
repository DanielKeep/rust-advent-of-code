extern crate clap;
extern crate crossbeam;
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

    let (zeroes, jobs) = args();

    let (secs, soln) = time_secs(|| {
        match jobs {
            Some(0) | None => search(key, zeroes),
            Some(jobs) => search_parallel(key, zeroes, jobs),
        }
    });

    println!("solution: {} (took {} secs)", soln, secs);
}

fn args() -> (u8, Option<u8>) {
    let matches = clap::App::new("day4")
        .args_from_usage("\
            -z --zeroes=[ZEROES] 'Specifies the number of leading zeroes'
            -j --jobs=[JOBS] 'Run in parallel with a given number of jobs'\
        ")
        .get_matches();

    let zeroes = matches.value_of("ZEROES")
        .map(|s| s.parse().unwrap())
        .unwrap_or(5);
    let jobs = matches.value_of("JOBS")
        .map(|s| s.parse().unwrap());
    (zeroes, jobs)
}

fn time_secs<F, R>(f: F) -> (f64, R)
where F: FnOnce() -> R {
    let start = time::precise_time_s();
    let r = f();
    let end = time::precise_time_s();
    (end - start, r)
}

fn search(key: &str, zeroes: u8) -> u32 {
    for n in 1.. {
        if try(key, n, zeroes) {
            return n;
        }
    }
    panic!("could not find solution for key {:?}", key);
}

fn search_parallel(key: &str, zeroes: u8, jobs: u8) -> u32 {
    use std::sync::RwLock;

    const BLOCK_SIZE: usize = 2048;

    println!("Running with {} jobs...", jobs);

    let result = RwLock::new(None);

    {
        let result = &result;

        crossbeam::scope(|scope| {
            for job in 0..jobs {

                scope.spawn(move || {
                    let mut it = StrideIter((1+job) as u32, jobs as u32);
                    while {
                        let res = *result.read().unwrap();
                        match res {
                            Some(res) => it.0 < res,
                            None => true
                        }
                    } {
                        for n in it.by_ref().take(BLOCK_SIZE) {
                            if try(key, n, zeroes) {
                                *result.write().unwrap() = Some(n);
                                return;
                            }
                        }
                    }
                });
            }
        });
    }

    let result = *result.read().unwrap();

    if let Some(result) = result {
        result
    } else {
        panic!("could not find solution for key {:?}", key);
    }
}

fn try(key: &str, n: u32, zeroes: u8) -> bool {
    let mut md5 = Md5::new();
    md5.input_str(key);
    let n_str = format!("{}", n);
    md5.input_str(&n_str);

    let hash = md5.result_str();
    hash.bytes().take(zeroes as usize).all(|b| b == b'0')
}

struct StrideIter(u32, u32);

impl Iterator for StrideIter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let r = self.0;
        match self.0.checked_add(self.1) {
            Some(v) => {
                self.0 = v;
                Some(r)
            },
            None => None
        }
    }
}
