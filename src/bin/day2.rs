use std::io::{BufRead, stdin};

fn main() {
    let stdin = stdin();
    let (area, ribbon) = stdin.lock().lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.trim() != "")
        .filter_map(|line| -> Option<(u32, u32, u32)> {
            let mut ns = line.split('x').map(|s| s.parse());
            match (ns.next(), ns.next(), ns.next()) {
                (Some(Ok(w)), Some(Ok(h)), Some(Ok(l))) => Some((w, h, l)),
                _ => {
                    println!("Skipping: {}", line);
                    None
                }
            }
        })
        .map(|(w, h, l)| {
            use std::cmp::min;
            let (a, b, c) = (w*h, h*l, l*w);
            let area = 2 * (a + b + c) + min(a, min(b, c));
            let perim = 2 * min(w+h, min(h+l, l+w));
            let bow = w*h*l;
            (area, perim+bow)
        })
        .fold((0, 0), |a, b| (a.0+b.0, a.1+b.1));
    println!("sq feet of paper: {}", area);
    println!("feet of ribbon:   {}", ribbon);
}
