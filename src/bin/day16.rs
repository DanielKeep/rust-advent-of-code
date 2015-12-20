#[macro_use] extern crate lazy_static;
extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;

#[derive(Default)]
struct Compounds<T> {
    children: T,
    cats: T,
    samoyeds: T,
    pomeranians: T,
    akitas: T,
    vizslas: T,
    goldfish: T,
    trees: T,
    cars: T,
    perfumes: T,
}

const NEEDLE: &'static Compounds<u8> = &Compounds {
    children: 3,
    cats: 7,
    samoyeds: 2,
    pomeranians: 3,
    akitas: 0,
    vizslas: 0,
    goldfish: 5,
    trees: 3,
    cars: 2,
    perfumes: 1,
};

fn main() {
    let new_rules = args();
    let mut sues = parse_input();

    if new_rules {
        sues.retain(|&(_, ref cand)| new_maybe_is(cand, NEEDLE));
    } else {
        sues.retain(|&(_, ref cand)| maybe_is(cand, NEEDLE));
    }

    println!("candidate Sues: {}",
        sues.iter().map(|&(sue, _)| sue).join(", "));
}

fn new_maybe_is<T>(cand: &Compounds<Option<T>>, reference: &Compounds<T>) -> bool
where T: Eq + Ord {
    macro_rules! as_expr { ($e:expr) => {$e} }
    macro_rules! check_rel {
        ($rel:tt: $($ns:ident),+) => {
            $(
                match cand.$ns.as_ref() {
                    Some(cand) => as_expr!(*cand $rel reference.$ns),
                    None => true
                }
            ) && +
        }
    }

    check_rel!(== : children, samoyeds, akitas, vizslas, cars, perfumes)
        && check_rel!(> : cats, trees)
        && check_rel!(< : pomeranians, goldfish)
}

fn maybe_is<T>(cand: &Compounds<Option<T>>, reference: &Compounds<T>) -> bool
where T: Eq {
    macro_rules! check {
        ($($ns:ident),+) => {
            $(
                match cand.$ns.as_ref() {
                    Some(cand) => cand.eq(&reference.$ns),
                    None => true
                }
            ) && +
        }
    }

    check!(children, cats, samoyeds, pomeranians, akitas,
        vizslas, goldfish, trees, cars, perfumes)
}

fn args() -> bool {
    extern crate clap;

    let matches = clap::App::new("day16")
        .args_from_usage("\
            -n --new-rules 'Use new rules'\
        ")
        .get_matches();

    let new_rules = matches.is_present("new-rules");

    new_rules
}

lazy_static! {
    static ref RE_LINE: Regex = Regex::new(r#"(?ix)
        ^ \s* sue \s+ (?P<sue> \d+): \s+ (?P<tail> .*) \s* $
    "#).unwrap();

    static ref RE_COMP: Regex = Regex::new(r#"(?ix)
        (?P<name> \w+) \s* : \s* (?P<value> \d+) \s* ,?
    "#).unwrap();
}

fn parse_input() -> Vec<(u16, Compounds<Option<u8>>)> {
    let input = read_stdin();

    let mut entries = vec![];

    for line in input.lines() {
        let cap = match RE_LINE.captures(line) {
            Some(cap) => cap,
            None => continue
        };

        let sue: u16 = cap.name("sue").map(str::parse).unwrap().unwrap();

        let comps = cap.name("tail").unwrap();

        let mut entry = Compounds::default();
        for comp_cap in RE_COMP.captures_iter(comps) {
            let name = comp_cap.name("name").unwrap();
            let value = comp_cap.name("value").map(str::parse).unwrap().unwrap();
            match name {
                "children" => entry.children = Some(value),
                "cats" => entry.cats = Some(value),
                "samoyeds" => entry.samoyeds = Some(value),
                "pomeranians" => entry.pomeranians = Some(value),
                "akitas" => entry.akitas = Some(value),
                "vizslas" => entry.vizslas = Some(value),
                "goldfish" => entry.goldfish = Some(value),
                "trees" => entry.trees = Some(value),
                "cars" => entry.cars = Some(value),
                "perfumes" => entry.perfumes = Some(value),
                name => panic!("invalid component: {:?}", name)
            }
        }

        entries.push((sue, entry));
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
