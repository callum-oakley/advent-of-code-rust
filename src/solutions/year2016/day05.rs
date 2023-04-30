use std::io::{Cursor, Seek, Write};

use md5::{Digest, Md5};

pub fn part1(input: &str) -> String {
    let mut password = String::new();
    let mut hasher = Md5::new();
    let mut buffer = Cursor::new(Vec::new());
    for i in 0.. {
        write!(buffer, "{input}{i}").unwrap();
        hasher.update(buffer.get_ref());
        buffer.rewind().unwrap();
        let hash = hasher.finalize_reset();
        if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0 {
            password.push(char::from_digit((hash[2] & 0x0f).into(), 16).unwrap());
            if password.len() == 8 {
                break;
            }
        }
    }
    password
}

pub fn part2(input: &str) -> String {
    let mut password = vec![None; 8];
    let mut hasher = Md5::new();
    let mut buffer = Cursor::new(Vec::new());
    for i in 0.. {
        write!(buffer, "{input}{i}").unwrap();
        hasher.update(buffer.get_ref());
        buffer.rewind().unwrap();
        let hash = hasher.finalize_reset();
        if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xf0 == 0 {
            let i = usize::from(hash[2] & 0x0f);
            if i < password.len() && password[i].is_none() {
                password[i] = Some(char::from_digit((hash[3] >> 4).into(), 16).unwrap());
                if password.iter().all(Option::is_some) {
                    break;
                }
            }
        }
    }
    password.into_iter().collect::<Option<_>>().unwrap()
}

pub fn tests() {
    assert_eq!(part1("abc"), "18f47a30");
    assert_eq!(part2("abc"), "05ace8e3");
}
