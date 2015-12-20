#[macro_use] extern crate lazy_static;
extern crate arrayvec;
extern crate conv;
extern crate regex;

use arrayvec::ArrayVec;
use conv::prelude::*;
use regex::Regex;

fn main() {
    let calories = args();
    let ingredients = parse_input();

    let (score, amts) = brute_force(&ingredients, calories);
    println!("best score: {}", score);
    println!("amounts: {:?}", amts);
}

fn score(ings: &[Ingredient], amts: &[u8], calories: Option<u32>) -> u32 {
    let (mut cap, mut dur, mut fla, mut tex, mut cal) = (0, 0, 0, 0, 0);

    for (ing, &amt) in ings.iter().zip(amts.iter()) {
        let amt: i32 = amt.value_into().unwrap_ok();
        cap += amt * ing.cap;
        dur += amt * ing.dur;
        fla += amt * ing.fla;
        tex += amt * ing.tex;
        cal += amt * ing.cal;
    }

    let cap = cap.value_as::<u32>().unwrap_or_saturate();
    let dur = dur.value_as::<u32>().unwrap_or_saturate();
    let fla = fla.value_as::<u32>().unwrap_or_saturate();
    let tex = tex.value_as::<u32>().unwrap_or_saturate();
    let cal = cal.value_as::<u32>().unwrap_or_saturate();

    if calories.is_none() || calories == Some(cal) {
        cap * dur * fla * tex
    } else {
        0
    }
}

fn brute_force(ingr: &[Ingredient], calories: Option<u32>) -> (u32, Vec<u8>) {
    let ingrs = ingr.len();
    let mut amts = vec![0; ingrs];
    amts[ingrs - 1] = 100;

    let mut best_soln = (score(ingr, &amts, calories), amts.clone());

    loop {
        next_soln(&mut amts);

        let cand_score = score(ingr, &amts, calories);

        if cand_score > best_soln.0 {
            best_soln.0 = cand_score;
            for (dst, src) in (best_soln.1).iter_mut().zip(amts.iter()) {
                *dst = *src;
            }
        }

        if amts[0] == 100 {
            break;
        }
    }

    best_soln
}

fn next_soln(amts: &mut [u8]) {
    let part: ArrayVec<[_; 16]> = amts.iter().cloned()
        .scan(0, |s, v| { let old = *s; *s += v; Some(old) })
        .collect();

    let last = amts.len() - 1;

    for (part, amt) in part.iter().zip(amts.iter_mut()).rev().skip(1) {
        *amt += 1;
        if (*amt + part) > 100 {
            *amt = 0;
        } else {
            break;
        }
    }

    amts[last] = amts[..last].iter().cloned().fold(100, |a, b| a - b);
}

fn args() -> Option<u32> {
    extern crate clap;

    let matches = clap::App::new("day15")
        .args_from_usage("\
            -c --calories=[CALORIES] 'Calories per biscuit'\
        ")
        .get_matches();

    let calories = matches.value_of("CALORIES")
        .map(|s| s.parse().unwrap());

    calories
}

#[derive(Clone, Debug)]
struct Ingredient {
    name: Box<str>,
    cap: i32,
    dur: i32,
    fla: i32,
    tex: i32,
    cal: i32,
}

lazy_static! {
    static ref RE_LINE: Regex = Regex::new(r#"(?ix)
        ^ \s*
        (?P<name> \w+): \s+
        capacity \s+ (?P<cap> -?\d+), \s*
        durability \s+ (?P<dur> -?\d+), \s*
        flavor \s+ (?P<fla> -?\d+), \s*
        texture \s+ (?P<tex> -?\d+), \s*
        calories \s+ (?P<cal> -?\d+)
        \s* $
    "#).unwrap();
}

fn parse_input() -> Vec<Ingredient> {
    let input = read_stdin();

    let mut entries = vec![];

    for line in input.lines() {
        let caps = match RE_LINE.captures(line) {
            Some(caps) => caps,
            None => continue
        };

        let name = caps.name("name").unwrap().to_owned();
        let cap = caps.name("cap").map(str::parse).unwrap().unwrap();
        let dur = caps.name("dur").map(str::parse).unwrap().unwrap();
        let fla = caps.name("fla").map(str::parse).unwrap().unwrap();
        let tex = caps.name("tex").map(str::parse).unwrap().unwrap();
        let cal = caps.name("cal").map(str::parse).unwrap().unwrap();

        entries.push(Ingredient {
            name: name.into_boxed_str(),
            cap: cap,
            dur: dur,
            fla: fla,
            tex: tex,
            cal: cal,
        });
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
