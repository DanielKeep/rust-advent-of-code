#[macro_use] extern crate lazy_static;
extern crate conv;
extern crate regex;

use std::collections::HashMap;
use std::io::{BufRead, stdin};
use conv::prelude::*;
use regex::Regex;

type Name = u16;

#[derive(Copy, Clone, Debug)]
struct Wire {
    name: Name,
    signal: Signal,
}

#[derive(Copy, Clone, Debug)]
enum Signal {
    Literal(Source),
    And(Source, Source),
    Or(Source, Source),
    LShift(Source, Source),
    RShift(Source, Source),
    Not(Source),
}

impl Signal {
    fn eval(self, values: &[Option<u16>]) -> Option<u16> {
        use self::Signal::*;
        macro_rules! eval_two {
            ($a0:ident, $a1:ident, $values:expr; $map:expr) => {
                $a0.eval($values)
                    .and_then(|$a0| $a1.eval($values)
                        .map(|$a1| $map))
            };
        }
        match self {
            Literal(a0) => a0.eval(values),
            And(a0, a1) => eval_two!(a0, a1, values; a0 & a1),
            Or(a0, a1) => eval_two!(a0, a1, values; a0 | a1),
            LShift(a0, a1) => eval_two!(a0, a1, values; a0 << a1),
            RShift(a0, a1) => eval_two!(a0, a1, values; a0 >> a1),
            Not(a0) => a0.eval(values).map(|a0| !a0),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Source {
    Literal(u16),
    Wire(Name),
}

impl Source {
    fn eval(self, values: &[Option<u16>]) -> Option<u16> {
        use self::Source::*;
        match self {
            Literal(lit) => Some(lit),
            Wire(name) => values[name as usize],
        }
    }
}

lazy_static! {
    static ref RE_ASSIGN: Regex = Regex::new(r#"(?xi)
        ^\s*
        (?P<a0> (?P<l0> \d+) | (?P<n0> \w+))
        \s* -> \s*
        (?P<out> \w+)
        \s*$
    "#).unwrap();
    static ref RE_NOT: Regex = Regex::new(r#"(?xi)
        ^\s*
        NOT \s*
        (?P<a0> (?P<l0> \d+) | (?P<n0> \w+))
        \s* -> \s*
        (?P<out> \w+)
        \s*$
    "#).unwrap();
    static ref RE_AND: Regex = Regex::new(r#"(?xi)
        ^\s*
        (?P<a0> (?P<l0> \d+) | (?P<n0> \w+))
        \s* AND \s*
        (?P<a1> (?P<l1> \d+) | (?P<n1> \w+))
        \s* -> \s*
        (?P<out> \w+)
        \s*$
    "#).unwrap();
    static ref RE_OR: Regex = Regex::new(r#"(?xi)
        ^\s*
        (?P<a0> (?P<l0> \d+) | (?P<n0> \w+))
        \s* OR \s*
        (?P<a1> (?P<l1> \d+) | (?P<n1> \w+))
        \s* -> \s*
        (?P<out> \w+)
        \s*$
    "#).unwrap();
    static ref RE_LSHIFT: Regex = Regex::new(r#"(?xi)
        ^\s*
        (?P<a0> (?P<l0> \d+) | (?P<n0> \w+))
        \s* LSHIFT \s*
        (?P<a1> (?P<l1> \d+) | (?P<n1> \w+))
        \s* -> \s*
        (?P<out> \w+)
        \s*$
    "#).unwrap();
    static ref RE_RSHIFT: Regex = Regex::new(r#"(?xi)
        ^\s*
        (?P<a0> (?P<l0> \d+) | (?P<n0> \w+))
        \s* RSHIFT \s*
        (?P<a1> (?P<l1> \d+) | (?P<n1> \w+))
        \s* -> \s*
        (?P<out> \w+)
        \s*$
    "#).unwrap();
}

fn main() {
    let (wires, _name_to_strs, str_to_names) = parse_input();
    let mut values = vec![None; wires.len()];
    let mut remaining = values.len();
    let mut passes = 0;

    while remaining > 0 {
        passes += 1;
        let remaining_at_start = remaining;
        for wire in wires.iter() {
            let wire_idx = wire.name as usize;
            if values[wire_idx].is_none() {
                if let Some(v) = wire.signal.eval(&values) {
                    values[wire_idx] = Some(v);
                    remaining -= 1;
                }
            }
        }
        if remaining == remaining_at_start {
            println!("Warning: could not solve; {} remaining", remaining);
        }
    }

    let mut names = str_to_names.into_iter().collect::<Vec<_>>();
    names.sort_by(|a, b| (a.0).cmp(&b.0));
    for (s, name) in names {
        let val = &values[name as usize];
        if let &Some(ref val) = val {
            println!("- {:>4}: {}", s, val);
        }
    }
    println!("(in {} pass{})", passes, if passes == 1 { "" } else { "es" });
}

fn parse_input() -> (Vec<Wire>, Vec<String>, HashMap<String, Name>) {
    let stdin = stdin();
    let lines = stdin.lock().lines()
        .filter_map(Result::ok);

    let mut wires = vec![];
    let mut name_to_strs = vec![];
    let mut str_to_names = HashMap::new();

    macro_rules! name_from {
        ($s:expr) => {
            {
                use std::collections::hash_map::Entry;
                let s = String::from($s);
                match str_to_names.entry(s.clone()) {
                    Entry::Occupied(e) => *e.get(),
                    Entry::Vacant(e) => {
                        let name = *e.insert(name_to_strs.len()
                            .value_into().unwrap());
                        name_to_strs.push(s);
                        name
                    },
                }
            }
        };
    }

    macro_rules! source_from {
        ($cap:expr, $arg_num:tt) => {
            {
                let lit = $cap.name(concat!("l", stringify!($arg_num)));
                let wir = $cap.name(concat!("n", stringify!($arg_num)));

                match (lit, wir) {
                    (Some(lit), None) => Source::Literal(lit.parse().unwrap()),
                    (None, Some(wir)) => Source::Wire(name_from!(wir)),
                    _ => panic!("invalid argument parse")
                }
            }
        };
    }

    for line in lines {
        let line = line.trim();
        if line == "" { continue; }

        let wire = if let Some(cap) = RE_ASSIGN.captures(line) {
            Wire {
                name: name_from!(cap.name("out").unwrap()),
                signal: Signal::Literal(source_from!(cap, 0)),
            }
        } else if let Some(cap) = RE_NOT.captures(line) {
            Wire {
                name: name_from!(cap.name("out").unwrap()),
                signal: Signal::Not(source_from!(cap, 0)),
            }
        } else if let Some(cap) = RE_AND.captures(line) {
            Wire {
                name: name_from!(cap.name("out").unwrap()),
                signal: Signal::And(source_from!(cap, 0), source_from!(cap, 1)),
            }
        } else if let Some(cap) = RE_OR.captures(line) {
            Wire {
                name: name_from!(cap.name("out").unwrap()),
                signal: Signal::Or(source_from!(cap, 0), source_from!(cap, 1)),
            }
        } else if let Some(cap) = RE_LSHIFT.captures(line) {
            Wire {
                name: name_from!(cap.name("out").unwrap()),
                signal: Signal::LShift(source_from!(cap, 0), source_from!(cap, 1)),
            }
        } else if let Some(cap) = RE_RSHIFT.captures(line) {
            Wire {
                name: name_from!(cap.name("out").unwrap()),
                signal: Signal::RShift(source_from!(cap, 0), source_from!(cap, 1)),
            }
        } else {
            continue;
        };

        wires.push(wire);
    }

    (wires, name_to_strs, str_to_names)
}
