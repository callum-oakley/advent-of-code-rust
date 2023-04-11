fn increment(pass: &mut [u8]) {
    let mut i = pass.len() - 1;
    loop {
        match pass[i] {
            b'z' => {
                pass[i] = b'a';
                i -= 1;
                continue;
            }
            // skip i, o, and l entirely
            b'h' | b'n' | b'k' => {
                pass[i] += 2;
            }
            _ => {
                pass[i] += 1;
            }
        }
        break;
    }
}

fn has_two_different_pairs(pass: &[u8]) -> bool {
    for i in 0..pass.len() - 1 {
        if pass[i] == pass[i + 1] {
            for j in i + 2..pass.len() - 1 {
                if pass[j] == pass[j + 1] && pass[i] != pass[j] {
                    return true;
                }
            }
        }
    }
    false
}

fn is_valid(pass: &[u8]) -> bool {
    pass.windows(3)
        .any(|triple| triple[0] + 1 == triple[1] && triple[1] + 1 == triple[2])
        && has_two_different_pairs(pass)
}

pub fn part1(input: &str) -> String {
    let mut pass = Vec::from(input);
    while !is_valid(&pass) {
        increment(&mut pass);
    }
    String::from_utf8(pass).unwrap()
}

pub fn part2(input: &str) -> String {
    let mut pass = Vec::from(input);
    for _ in 0..2 {
        increment(&mut pass);
        while !is_valid(&pass) {
            increment(&mut pass);
        }
    }
    String::from_utf8(pass).unwrap()
}

pub fn tests() {
    assert_eq!(part1("abcdefgh"), "abcdffaa");
}
