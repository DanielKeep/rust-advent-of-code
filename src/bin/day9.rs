#[macro_use] extern crate lazy_static;
extern crate clap;
extern crate conv;
extern crate itertools;
extern crate permutohedron;
extern crate regex;

macro_rules! try_opt {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return None
        }
    };
}

use std::cmp::Ordering;
use std::collections::HashMap;
use std::marker::PhantomData;
use itertools::Itertools;
use regex::Regex;

type Cost = u16;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
struct Name<'a>(u8, PhantomData<&'a ()>);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Edge<'a> {
    src: Name<'a>,
    dst: Name<'a>,
    cost: Cost,
}

#[derive(Debug)]
struct Graph<'a> {
    edges: Vec<Edge<'a>>,
}

fn main() {
    let longest = args();

    let input = read_stdin();
    let mut names = Interner::new();
    let edges = parse_input(&input, &mut names);
    let graph = Graph::new(edges);

    let mut init_soln = names.names();
    let mut best_soln = (total_cost(&init_soln, &graph), init_soln.clone());
    let mut cand_solns = permutohedron::Heap::new(&mut init_soln[..]);

    fn first_cost_better(
        a: Option<&u32>,
        b: Option<&u32>,
        longest: bool,
    ) -> bool {
        match (a, b) {
            (None, _) => false,
            (Some(_), None) => true,
            (Some(a), Some(b)) => if longest { *a > *b } else { *a < *b }
        }
    }

    // Skip one.
    let _ = cand_solns.next_permutation();

    while let Some(cand_soln) = cand_solns.next_permutation() {
        let cost = total_cost(cand_soln, &graph);
        if first_cost_better(cost.as_ref(), (best_soln.0).as_ref(), longest) {
            let (_, mut soln) = best_soln;
            for (dst, src) in soln.iter_mut().zip(cand_soln.iter()) {
                *dst = *src;
            }
            best_soln = (cost, soln);
        }
    }

    match best_soln {
        (None, _) => println!("no solution found!"),
        (Some(cost), path) => {
            println!("best path: {} = {}",
                path.iter().cloned()
                    .map(|name| names.to_str(name))
                    .join(" -> "),
                cost);
        }
    }
}

fn args() -> bool {
    let matches = clap::App::new("day9")
        .args_from_usage("\
            -l --longest 'Return longest path instead of shortest'\
        ")
        .get_matches();

    let longest = matches.is_present("longest");

    longest
}

fn total_cost(path: &[Name], graph: &Graph) -> Option<u32> {
    let pairs = path.iter().zip(path.iter().skip(1));
    let mut total_cost = 0;
    for (&a, &b) in pairs {
        total_cost += match graph.lookup(a, b) {
            Some(cost) => cost as u32,
            None => return None
        };
    }
    Some(total_cost)
}

fn parse_input<'a>(
    s: &'a str,
    names: &mut Interner<'a>,
) -> Vec<Edge<'a>> {
    let mut edges: Vec<_> = s.lines()
        .filter_map(|line| Edge::from_str(line, names))
        .collect();
    edges.sort();
    edges
}

fn read_stdin() -> String {
    use std::io::{Read, stdin};
    let mut s = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut s).unwrap();
    s
}

lazy_static! {
    static ref RE_EDGE: Regex = Regex::new(r#"(?ix)
        ^ \s*
        (?P<src> \w+)
        \s* to \s*
        (?P<dst> \w+)
        \s* = \s*
        (?P<cost> \d+)
        \s* $
    "#).unwrap();
}

impl<'a> Name<'a> {
    fn ord_pair(a: Self, b: Self) -> (Self, Self) {
        if a <= b {
            (a, b)
        } else {
            (b, a)
        }
    }
}

impl<'a> Edge<'a> {
    fn from_str(s: &'a str, names: &mut Interner<'a>)
    -> Option<Edge<'a>> {
        let cap = try_opt!(RE_EDGE.captures(s));
        let src = try_opt!(cap.name("src"));
        let dst = try_opt!(cap.name("dst"));
        let cost = try_opt!(cap.name("cost"));

        let src = names.intern(src);
        let dst = names.intern(dst);
        let cost = try_opt!(cost.parse().ok());

        Some(Edge {
            src: src,
            dst: dst,
            cost: cost,
        })
    }

    fn cmp_endpoints(&self, other: &(Name<'a>, Name<'a>)) -> Ordering {
        let self_key = Name::ord_pair(self.src, self.dst);
        self_key.cmp(other)
    }
}

impl<'a> Ord for Edge<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_key = Name::ord_pair(self.src, self.dst);
        let other_key = Name::ord_pair(other.src, other.dst);
        self_key.cmp(&other_key)
    }
}

impl<'a> PartialOrd for Edge<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Graph<'a> {
    fn new(mut edges: Vec<Edge<'a>>) -> Self {
        edges.sort();
        Graph {
            edges: edges,
        }
    }

    fn lookup(&self, src: Name<'a>, dst: Name<'a>) -> Option<Cost> {
        let ends = Name::ord_pair(src, dst);
        match self.edges.binary_search_by(|probe| probe.cmp_endpoints(&ends)) {
            Ok(off) => Some(self.edges[off].cost),
            Err(_) => None
        }
    }
}

struct Interner<'a> {
    map: HashMap<&'a str, Name<'a>>,
    rev: HashMap<Name<'a>, &'a str>,
}

impl<'a> Interner<'a> {
    fn new() -> Self {
        Interner {
            map: HashMap::new(),
            rev: HashMap::new(),
        }
    }

    fn intern(&mut self, key: &'a str) -> Name<'a> {
        use conv::prelude::*;
        use std::collections::hash_map::Entry::*;

        let new_name = Name(self.map.len().value_into().unwrap(), PhantomData);
        match self.map.entry(key) {
            Occupied(e) => *e.get(),
            Vacant(e) => {
                e.insert(new_name);
                self.rev.insert(new_name, key);
                new_name
            }
        }
    }

    fn to_str(&self, name: Name<'a>) -> &'a str {
        self.rev.get(&name).unwrap()
    }

    fn names(&self) -> Vec<Name<'a>> {
        let mut names: Vec<_> = self.map.values().cloned().collect();
        names.sort();
        names
    }
}
