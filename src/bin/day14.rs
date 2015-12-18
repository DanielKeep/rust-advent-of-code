#[macro_use] extern crate lazy_static;
extern crate regex;

use std::cmp;
use regex::Regex;

fn main() {
    let (new_rules, seconds) = args();
    let entries = parse_input();

    if !new_rules {
        let mut results: Vec<_> = entries.iter()
            .map(|e| (&*e.0, simulate(e, seconds)))
            .collect();
        results.sort_by(|a, b| a.1.cmp(&b.1).reverse());

        println!("results after {} s:", seconds);
        for (name, dist) in results {
            println!("- {} flew {} km", name, dist);
        }
    } else {
        let mut results = simulate_part_2(&entries, seconds);
        results.sort_by(|a, b| a.1.cmp(&b.1).reverse());

        println!("results after {} s:", seconds);
        for (name, pts, pos) in results {
            println!("- {} scored {} points (at {} km)", name, pts, pos);
        }
    }
}

fn simulate(&(_, fv, ft, rt): &(String, u32, u32, u32), seconds: u32) -> u32 {
    let mut t = 0;
    let mut pos = 0;

    while t < seconds {
        let fly_time = cmp::min(ft, seconds - t);
        pos += fly_time * fv;
        t += fly_time + rt;
    }

    pos
}

fn simulate_part_2(entries: &[(String, u32, u32, u32)], seconds: u32) -> Vec<(&str, u32, u32)> {
    #[derive(Clone, Default)]
    struct State {
        pts: u32,
        pos: u32,
        ft: u32,
        rt: u32,
    }

    let mut states: Vec<_> = entries.iter()
        .map(|e| State { pts: 0, pos: 0, ft: e.2, rt: e.3 })
        .collect();

    for _ in 0..seconds {
        for (en, st) in entries.iter().zip(states.iter_mut()) {
            if st.ft > 0 {
                st.pos += en.1;
                st.ft -= 1;
            } else if st.rt > 0 {
                st.rt -= 1;
            }

            if st.ft == 0 && st.rt == 0 {
                st.ft = en.2;
                st.rt = en.3;
            }
        }

        let max_pos = states.iter().map(|e| e.pos).max().unwrap();

        for st in states.iter_mut() {
            if st.pos == max_pos {
                st.pts += 1;
            }
        }
    }

    entries.iter().zip(states.iter())
        .map(|(en, st)| (&*en.0, st.pts, st.pos))
        .collect()
}

fn args() -> (bool, u32) {
    extern crate clap;

    let matches = clap::App::new("day14")
        .args_from_usage("\
            -n --new-rules 'Use new rules'
            <SECONDS> 'Number of seconds to simulate'\
        ")
        .get_matches();

    let new_rules = matches.is_present("new-rules");
    let seconds = matches.value_of("SECONDS")
        .map(|s| s.parse().unwrap())
        .unwrap();

    (new_rules, seconds)
}

lazy_static! {
    static ref RE_LINE: Regex = Regex::new(r#"(?ix)
        ^ \s* (?P<name> \w+) \s+ can \s+ fly \s+
        (?P<fly_v> \d+) \s+ km/s \s+ for \s+ (?P<fly_t> \d+) \s+ seconds,
        \s+ but \s+ then \s+ must \s+ rest \s+ for \s+
        (?P<rest> \d+) \s+ seconds [.] \s* $
    "#).unwrap();
}

fn parse_input() -> Vec<(String, u32, u32, u32)> {
    let input = read_stdin();

    let mut entries = vec![];

    for line in input.lines() {
        let cap = match RE_LINE.captures(line) {
            Some(cap) => cap,
            None => continue
        };

        let name = cap.name("name").unwrap().to_owned();
        let fly_v = cap.name("fly_v").map(str::parse).unwrap().unwrap();
        let fly_t = cap.name("fly_t").map(str::parse).unwrap().unwrap();
        let rest = cap.name("rest").map(str::parse).unwrap().unwrap();

        entries.push((name, fly_v, fly_t, rest));
    }

    entries
}

fn read_stdin() -> String {
    use std::io::{Read, stdin};
    let mut s = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut s).unwrap();
    s
}
