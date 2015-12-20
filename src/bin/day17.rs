fn main() {
    let (show_solutions, min_num, liters) = args();
    let containers: Vec<u32> = read_stdin().lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut solutions = vec![];
    search(&containers, liters, &mut solutions);

    if min_num {
        solutions.sort_by(|a, b| a.len().cmp(&b.len()));
        if let Some(min_len) = solutions.get(0).map(|e| e.len()) {
            solutions.retain(|e| e.len() == min_len);
        }
    }

    println!("{} solutions{}",
        solutions.len(),
        if show_solutions { ":" } else { "" });

    if show_solutions {
        for soln in solutions {
            println!("- {:?}", soln);
        }
    }
}

fn search(cont: &[u32], liters: u32, solns: &mut Vec<Vec<u32>>) {
    fn step(prefix: Vec<u32>, cont: &[u32], liters: u32, solns: &mut Vec<Vec<u32>>) {
        if cont.len() == 0 {
            return;
        }

        let head_cont = cont[0];
        let cont = &cont[1..];

        if liters >= head_cont {
            // Try *with* this container.
            let mut prefix = prefix.clone();
            prefix.push(head_cont);
            let liters = liters - head_cont;

            if liters == 0 {
                solns.push(prefix);
            } else {
                step(prefix, cont, liters, solns);
            }
        }

        if liters == 0 {
            solns.push(prefix.clone());
        } else {
            step(prefix, cont, liters, solns);
        }
    }

    step(vec![], cont, liters, solns);
}

fn args() -> (bool, bool, u32) {
    extern crate clap;

    let matches = clap::App::new("day17")
        .args_from_usage("\
            -s --solutions 'Show solutions'
            -m --min-num 'How many ways can be minimum number of containers be filled'
            <LITERS> 'Liters of eggnog'\
        ")
        .get_matches();

    let solutions = matches.is_present("solutions");
    let min_num = matches.is_present("min-num");
    let liters = matches.value_of("LITERS")
        .map(|s| s.parse().unwrap())
        .unwrap();

    (solutions, min_num, liters)
}

fn read_stdin() -> String {
    use std::io::{Read, stdin};
    let mut s = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut s).unwrap();
    s
}
