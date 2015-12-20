extern crate itertools;

#[cfg(feature="winapi")] extern crate winapi;
#[cfg(feature="kernel32-sys")] extern crate kernel32;

use std::ops::{Index, IndexMut};
use std::time::Duration;
use itertools::Itertools;

#[cfg(feature="day18-animation")]
const SLEEP_TIME: u64 = 1000/10;

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
        if cfg!(feature = "day18-animation") {
            clear_screen();
            print!("{}", curr_bitmap);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            std::thread::sleep(Duration::from_millis(SLEEP_TIME));
        }

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
        if cfg!(not(feature="day18-animation")) {
            println!("Bitmap:");
        } else {
            clear_screen();
        }
        println!("{}\n", curr_bitmap);
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

#[cfg(feature="day18-braille")]
const BRAILLE_OFFSET: u32 = 0x2800;

#[cfg(feature="day18-braille")]
const BRAILLE_BITS: &'static [u8; 8] = &[
    0x01, 0x08,
    0x02, 0x10,
    0x04, 0x20,
    0x40, 0x80,
];

#[cfg(feature="day18-braille")]
impl std::fmt::Display for Bitmap {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let w = self.width();
        let h = self.height();
        let mut first = true;

        for by in 0..((self.height() + 3)/4) {
            if !first { try!("\n".fmt(fmt)); }
            first = false;

            for bx in 0..((self.width() + 1)/2) {
                let x_off = bx*2;
                let y_off = by*4;
                let mut bits = 0;

                let coords = (y_off..y_off+4)
                    .cartesian_product(x_off..x_off+2)
                    .zip(BRAILLE_BITS)
                    .filter(|&((y, x), _)| (x < w) && (y < h));

                for ((y, x), bb) in coords {
                    if self[(x, y)] { bits |= *bb; }
                }

                let c = BRAILLE_OFFSET | bits as u32;
                let c = std::char::from_u32(c).unwrap();
                try!(c.fmt(fmt));
            }
        }
        Ok(())
    }
}

#[cfg(not(feature="day18-braille"))]
impl std::fmt::Display for Bitmap {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        for y in 0..self.height() {
            if !first { try!("\n".fmt(fmt)); }
            first = false;

            for e in &self[(.., y)] {
                if *e {
                    try!("#".fmt(fmt));
                } else {
                    try!(".".fmt(fmt));
                }
            }
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

#[cfg(feature="day18-animation")]
fn clear_screen() {
    use std::mem::zeroed;
    use winapi::*;
    use kernel32::*;

    unsafe {
        let fill = b' ' as u16;
        let tl = COORD { X: 0, Y: 0 };
        let mut s: CONSOLE_SCREEN_BUFFER_INFO = zeroed();
        let console: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        GetConsoleScreenBufferInfo(console, &mut s);

        let mut written: DWORD = 0;
        let cells: DWORD = s.dwSize.X as DWORD * s.dwSize.Y as DWORD;

        FillConsoleOutputCharacterW(console, fill, cells, tl, &mut written);
        FillConsoleOutputAttribute(console, s.wAttributes, cells, tl, &mut written);
        SetConsoleCursorPosition(console, tl);
    }
}
