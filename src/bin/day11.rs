fn main() {
    let password = read_stdin().lines().next().map(String::from).unwrap();
    let password = password.into_bytes();

    let next_pw = increment_password(password.clone());

    let password = String::from_utf8(password).unwrap();
    let next_pw = String::from_utf8(next_pw).unwrap();

    println!("next password after `{}`: `{}`", password, next_pw);
}

fn increment_password(mut bytes: Vec<u8>) -> Vec<u8> {
    loop {
        bytes = increment_string(bytes);
        if check_password(&bytes) {
            return bytes;
        }
    }
}

#[test]
fn test_increment_password() {
    use self::increment_password as incpas;
    assert_eq!(&*incpas(b"abcdefgh"[..].to_owned()), b"abcdffaa");
    assert_eq!(&*incpas(b"ghijklmn"[..].to_owned()), b"ghjaabcc");
}

fn check_password(mut bytes: &[u8]) -> bool {
    let mut found_straight = false;
    let mut first_double = None;
    let mut found_second_double = false;

    if bytes.iter().any(|b| FORBIDDEN_CHARACTERS.contains(b)) {
        return false;
    }

    while !(found_straight && found_second_double) && bytes.len() >= 2 {
        if !found_straight {
            if bytes.len() < 3 { return false; }
            found_straight = (bytes[1] == bytes[0] + 1)
                && (bytes[2] == bytes[1] + 1);
        }

        match (first_double, found_second_double) {
            (None, _) => {
                if bytes[0] == bytes[1] {
                    first_double = Some(bytes[0]);
                }
            },
            (Some(first_double), false) => {
                if bytes[0] == bytes[1] && bytes[0] != first_double {
                    found_second_double = true;
                }
            },
            _ => ()
        }

        bytes = &bytes[1..];
    }

    found_straight && found_second_double
}

#[test]
fn test_check_password() {
    assert_eq!(check_password(b"hijklmmn"), false);
    assert_eq!(check_password(b"abbceffg"), false);
    assert_eq!(check_password(b"abbcegjk"), false);
    assert_eq!(check_password(b"abcdefgh"), false);
    assert_eq!(check_password(b"abcdffaa"), true);
    assert_eq!(check_password(b"ghijklmn"), false);
    assert_eq!(check_password(b"ghjaabcc"), true);
}

const FORBIDDEN_CHARACTERS: &'static [u8] = b"iol";

fn increment_string(mut bytes: Vec<u8>) -> Vec<u8> {
    assert!(!FORBIDDEN_CHARACTERS.contains(&b'a'));
    assert!(bytes.iter().all(|&b| b'a' <= b && b <= b'z'));

    for b in bytes.iter_mut().rev() {
        *b += 1;
        while FORBIDDEN_CHARACTERS.contains(b) {
            *b += 1;
        }
        if *b > b'z' {
            *b = b'a';
        } else {
            break;
        }
    }

    bytes
}

#[test]
fn test_increment_string() {
    use self::increment_string as incstr;
    assert_eq!(&*incstr(b"a"[..].to_owned()), &b"b"[..]);
    assert_eq!(&*incstr(b"aaa"[..].to_owned()), &b"aab"[..]);
    assert_eq!(&*incstr(b"aaz"[..].to_owned()), &b"aba"[..]);
    assert_eq!(&*incstr(b"zzz"[..].to_owned()), &b"aaa"[..]);
    assert_eq!(&*incstr(b"abh"[..].to_owned()), &b"abj"[..]);
    assert_eq!(&*incstr(b"abn"[..].to_owned()), &b"abp"[..]);
    assert_eq!(&*incstr(b"abk"[..].to_owned()), &b"abm"[..]);
}

fn read_stdin() -> String {
    use std::io::{Read, stdin};
    let mut s = String::new();
    let stdin = stdin();
    stdin.lock().read_to_string(&mut s).unwrap();
    s
}
