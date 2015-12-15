extern crate strcursor;

use strcursor::StrCursor;

fn main() {
    let input = read_stdin();
    let (code, text) = count(&input);

    println!("code: {}", code);
    println!("text: {}", text);
    println!("diff: {}", code - text);
}

fn count(s: &str) -> (usize, usize) {
    let mut cur = StrCursor::new_at_start(s);

    let mut code = 0;
    let mut text = 0;

    while let Some(cp) = pop_cp(&mut cur) {
        match cp {
            '"' => code += 1,
            '\\' => {
                code += 1;
                match pop_cp(&mut cur) {
                    Some('"') | Some('\\')
                    => { code += 1; text += 1; },
    
                    Some('x') => {
                        code += 3;
                        text += 1;

                        let after = cur.at_next_cp()
                            .and_then(|cur| cur.at_next_cp());
                        cur = match after {
                            Some(cur) => cur,
                            None => panic!("unfinished hex escape sequence")
                        };
                    },
                    Some(c) => panic!("invalid escape sequence: \\{}", c),
                    None => panic!("unfinished escape sequence")
                }
            },
            ' ' | '\t' | '\r' | '\n' => (),
            _ => { code += 1; text += 1; }
        }
    }

    (code, text)
}

fn read_stdin() -> String {
    use std::io::{Read, stdin};
    let mut s = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut s).unwrap();
    s
}

fn pop_cp(cur: &mut StrCursor) -> Option<char> {
    let cp = cur.cp_after();
    *cur = cur.at_next_cp().unwrap_or(*cur);
    cp
}
