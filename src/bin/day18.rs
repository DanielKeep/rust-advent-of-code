extern crate itertools;

use std::ops::{Index, IndexMut};
use itertools::Itertools;

fn main() {
    let (broken, show, steps) = args();
    let mut curr_bitmap = parse_input();
    let mut next_bitmap = curr_bitmap.clone();

    if broken {
        let x1 = curr_bitmap.width() - 1;
        let y1 = curr_bitmap.height() - 1;
        curr_bitmap[(0, 0)] = true;
        curr_bitmap[(x1, 0)] = true;
        curr_bitmap[(0, y1)] = true;
        curr_bitmap[(x1, y1)] = true;
    }

    for _ in 0..steps {
        for xy in curr_bitmap.iter_coords() {
            let ns = &mut 0;
            curr_bitmap.neighbours_of(xy, |n_xy| {
                if curr_bitmap[n_xy] {
                    *ns += 1;
                }
            });
            next_bitmap[xy] = match (curr_bitmap[xy], *ns) {
                (true, 2) | (true, 3) | (false, 3) => true,
                _ => false
            };
        }

        if broken {
            let x1 = next_bitmap.width() - 1;
            let y1 = next_bitmap.height() - 1;
            next_bitmap[(0, 0)] = true;
            next_bitmap[(x1, 0)] = true;
            next_bitmap[(0, y1)] = true;
            next_bitmap[(x1, y1)] = true;
        }

        std::mem::swap(&mut curr_bitmap, &mut next_bitmap);
    }

    let lights = curr_bitmap.iter().fold(0u32, |a, &b| a + if b { 1 } else { 0 });

    if show {
        println!("Bitmap:");
        println!("{}", curr_bitmap);
    }

    println!("lights on after {} steps: {}", steps, lights);
}

#[derive(Clone)]
struct Bitmap {
    cells: Vec<bool>,
    width: u16,
}

type Coords = itertools::Product<
    std::ops::Range<u16>,
    std::ops::Range<u16>
>;

impl Bitmap {
    fn iter(&self) -> std::slice::Iter<bool> {
        self.cells.iter()
    }

    fn iter_coords(&self) -> Coords {
        (0..self.height()).cartesian_product(0..self.width())
    }

    fn neighbours_of<F>(&self, (x, y): (u16, u16), mut f: F)
    where F: FnMut((u16, u16)) {
        let w = self.width();
        let h = self.height();
        if y > 0 {
            if x > 0 { f((x-1, y-1)); }
            f((x, y-1));
            if x+1 < w { f((x+1, y-1)); }
        }
        {
            if x > 0 { f((x-1, y)); }
            if x+1 < w { f((x+1, y)); }
        }
        if y+1 < h {
            if x > 0 { f((x-1, y+1)); }
            f((x, y+1));
            if x+1 < w { f((x+1, y+1)); }
        }
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        (self.cells.len() / (self.width as usize)) as u16
    }
}

impl std::fmt::Display for Bitmap {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.height() {
            for e in &self[(.., y)] {
                if *e {
                    try!("#".fmt(fmt));
                } else {
                    try!(".".fmt(fmt));
                }
            }
            try!("\n".fmt(fmt));
        }
        Ok(())
    }
}

impl Index<(u16, u16)> for Bitmap {
    type Output = bool;

    fn index(&self, index: (u16, u16)) -> &bool {
        if index.0 >= self.width() {
            panic!("width out of bounds: {} >= {}", index.0, self.width());
        }
        if index.1 >= self.height() {
            panic!("height out of bounds: {} >= {}", index.1, self.height());
        }
        let idx = index.0 as usize + index.1 as usize * self.width() as usize;
        &self.cells[idx]
    }
}

impl Index<(std::ops::RangeFull, u16)> for Bitmap {
    type Output = [bool];

    fn index(&self, index: (std::ops::RangeFull, u16)) -> &[bool] {
        if index.1 >= self.height() {
            panic!("height out of bounds: {} >= {}", index.1, self.height());
        }
        let beg = index.1 as usize * self.width() as usize;
        let end = (index.1 as usize + 1) * self.width() as usize;
        &self.cells[beg..end]
    }
}

impl IndexMut<(u16, u16)> for Bitmap {
    fn index_mut(&mut self, index: (u16, u16)) -> &mut bool {
        if index.0 >= self.width() {
            panic!("width out of bounds: {} >= {}", index.0, self.width());
        }
        if index.1 >= self.height() {
            panic!("height out of bounds: {} >= {}", index.1, self.height());
        }
        let idx = index.0 as usize + index.1 as usize * self.width() as usize;
        &mut self.cells[idx]
    }
}

fn args() -> (bool, bool, u32) {
    extern crate clap;

    let matches = clap::App::new("day18")
        .args_from_usage("\
            -b --broken 'Add broken corner lights'
            -s --show 'Show bitmap'
            <STEPS> 'Number of steps'\
        ")
        .get_matches();

    let broken = matches.is_present("broken");
    let show = matches.is_present("show");
    let steps = matches.value_of("STEPS")
        .map(|s| s.parse().unwrap())
        .unwrap();

    (broken, show, steps)
}

fn parse_input() -> Bitmap {
    use std::io::{Read, stdin};
    let mut width = None;
    let cells: Vec<_> = stdin().bytes()
        .flat_map(|br| br.ok())
        .filter(|&b| match b {
            b'#' | b'.' | b'\r' | b'\n' => true,
            _ => false
        })
        .enumerate()
        .flat_map(|(i, b)| match b {
            b'#' => Some(true),
            b'.' => Some(false),
            _ => {
                width = width.or(Some(i));
                None
            }
        })
        .collect();

    let width: usize = width.unwrap();
    assert_eq!((cells.len() / width) * width, cells.len());

    Bitmap {
        cells: cells,
        width: width as u16,
    }
}
