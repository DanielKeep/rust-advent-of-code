#[macro_use] extern crate lazy_static;
extern crate regex;

macro_rules! try_opt {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => return None
        }
    };
}

use std::io::{BufRead, stdin};
use regex::Regex;

type Point = (u16, u16);
type Rect = (Point, Point);

const FIELD_SIZE: (usize, usize) = (1000, 1000);

lazy_static! {
    static ref RE_INSTRUCTION: Regex = Regex::new(r#"(?xi)
        ^\s*
        (?:
            turn\s+(?P<op_on>on)
            | turn\s+(?P<op_off>off)
            | (?P<op_toggle>toggle)
        )
        \s+
        (?P<pt0x>\d+)\s*,\s*(?P<pt0y>\d+)
        \s+
        through
        \s+
        (?P<pt1x>\d+)\s*,\s*(?P<pt1y>\d+)
        \s*$
    "#).unwrap();
}

#[derive(Copy, Clone, Debug)]
enum Instr {
    TurnOn(Rect),
    TurnOff(Rect),
    Toggle(Rect),
}

impl Instr {
    fn apply(self, field: &mut [bool]) {
        use self::Instr::*;
        match self {
            TurnOn(rect) => mutate_rect(field, rect, |_, c| *c = true),
            TurnOff(rect) => mutate_rect(field, rect, |_, c| *c = false),
            Toggle(rect) => mutate_rect(field, rect, |_, c| *c = !*c),
        }
    }
}

fn main() {
    let instrs = read_instrs();
    let mut field = vec![false; FIELD_SIZE.0 * FIELD_SIZE.1];

    for instr in instrs {
        instr.apply(&mut field);
    }

    let lights_on = {
        let mut lights_on = 0;
        let rect = (
            (0,0),
            ((FIELD_SIZE.0 - 1) as u16,
                (FIELD_SIZE.1 - 1) as u16)
        );
        mutate_rect(&mut field, rect, |_, on| if *on { lights_on += 1; });
        lights_on
    };

    println!("lights on: {}", lights_on);
}

fn read_instrs() -> Vec<Instr> {
    let stdin = stdin();
    let instrs = stdin.lock().lines()
        .filter_map(Result::ok)
        .flat_map(|line| {
            use self::Instr::*;

            let m = try_opt!(RE_INSTRUCTION.captures(&line));

            let op = try_opt!(m.name("op_on")
                .or(m.name("op_off"))
                .or(m.name("op_toggle"))
            );
            let pt0x = try_opt!(try_opt!(m.name("pt0x")).parse().ok());
            let pt0y = try_opt!(try_opt!(m.name("pt0y")).parse().ok());
            let pt1x = try_opt!(try_opt!(m.name("pt1x")).parse().ok());
            let pt1y = try_opt!(try_opt!(m.name("pt1y")).parse().ok());

            let rect = ((pt0x, pt0y), (pt1x, pt1y));

            match op {
                "on" => Some(TurnOn(rect)),
                "off" => Some(TurnOff(rect)),
                "toggle" => Some(Toggle(rect)),
                _ => None
            }
        })
        .collect();
    instrs
}

fn mutate_rect<F>(field: &mut [bool], rect: Rect, mut f: F)
where F: FnMut(Point, &mut bool) {
    for y in rect .0 .1 .. (rect .1 .1 + 1) {
        let x_beg = rect .0 .0;
        let x_end = rect .1 .0 + 1;

        let y_off = (y as usize) * FIELD_SIZE.0;
        let x_beg_off = y_off + x_beg as usize;
        let x_end_off = y_off + x_end as usize;

        let cells = (&mut field[x_beg_off..x_end_off]).into_iter();

        for (x, e) in (x_beg..x_end).zip(cells) {
            f((x, y), e);
        }
    }
}
