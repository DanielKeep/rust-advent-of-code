#[macro_use] extern crate lazy_static;
extern crate clap;
extern crate itertools;
extern crate permutohedron;
extern crate regex;

use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let add_myself = args();
    let input = read_stdin();
    let (names, table) = parse_input(&input, add_myself);
    let nns = names.len();

    println!("names: {:?}", names);
    println!("table: {:?}", table);

    let mut init_soln: Vec<_> = (0..nns).collect();
    let mut best_soln = (total_diff(&init_soln, nns, &table), init_soln.clone());
    let mut cand_solns = permutohedron::Heap::new(&mut init_soln[..]);

    // Skip one.
    let _ = cand_solns.next_permutation();

    while let Some(cand_soln) = cand_solns.next_permutation() {
        let diff = total_diff(cand_soln, nns, &table);

        if diff > best_soln.0 {
            let (_, mut soln) = best_soln;
            for (dst, src) in soln.iter_mut().zip(cand_soln.iter()) {
                *dst = *src;
            }
            best_soln = (diff, soln);
        }
    }

    let names_rev: HashMap<_, _> = names.iter()
        .map(|(&k, &v)| (v, k))
        .collect();

    let order = best_soln.1.iter().cloned()
        .map(|idx| names_rev[&idx])
        .join(", ");

    println!("best seating order: {}", order);
    println!("total change in happiness: {}", best_soln.0);
}

fn total_diff(soln: &[usize], num_names: usize, table: &[i32]) -> i32 {
    let mut total = 0;
    let shifted = soln.iter().skip(1).chain(soln.iter().take(1));
    for (&a, &b) in soln.iter().zip(shifted) {
        total += table[a + b * num_names] + table[b + a * num_names];
    }
    total
}

fn args() -> bool {
    let matches = clap::App::new("day13")
        .args_from_usage("\
            -m --myself 'Add `Myself` to the list of names'\
        ")
        .get_matches();

    let myself = matches.is_present("myself");

    myself
}

lazy_static! {
    static ref RE_LINE: Regex = Regex::new(r#"(?ix)
        ^ \s* (?P<name0> \w+)
        \s+ would \s+
        (?P<dir> gain | lose) \s+ (?P<diff> \d+)
        \s+ happiness \s+ units? \s+ by \s+ sitting \s+ next \s+ to \s+
        (?P<name1> \w+) [.] \s* $
    "#).unwrap();
}

fn parse_input(input: &str, add_myself: bool) -> (HashMap<&str, usize>, Vec<i32>) {
    let mut names = HashMap::new();
    let mut entries = vec![];

    for line in input.lines() {
        let cap = match RE_LINE.captures(line) {
            Some(cap) => cap,
            None => continue
        };

        let name0 = cap.name("name0").unwrap();
        let name1 = cap.name("name1").unwrap();
        let pos = cap.name("dir").unwrap() == "gain";
        let diff: i32 = cap.name("diff").map(str::parse).unwrap().unwrap();

        let name0 = { let n = names.len(); *names.entry(name0).or_insert(n) };
        let name1 = { let n = names.len(); *names.entry(name1).or_insert(n) };
        entries.push((name0, name1, if pos { diff } else { -diff }));
    }

    if add_myself {
        let n = names.len();
        names.insert("Myself", n);
    }

    let ns = names.len();
    let mut table = vec![0; ns * ns];
    for (n0, n1, d) in entries {
        table[n0 + n1*ns] = d;
    }

    (names, table)
}

fn read_stdin() -> String {
    use std::io::{Read, stdin};
    let mut s = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut s).unwrap();
    s
}
