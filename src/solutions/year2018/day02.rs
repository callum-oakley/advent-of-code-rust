fn has_repetitions(target: u32, id: &str) -> bool {
    for a in id.chars() {
        let mut repetitions = 0;
        for b in id.chars() {
            if a == b {
                repetitions += 1;
            }
        }
        if repetitions == target {
            return true;
        }
    }
    false
}

pub fn part1(input: &str) -> u32 {
    let mut pairs = 0;
    let mut triples = 0;
    for id in input.split_whitespace() {
        if has_repetitions(2, id) {
            pairs += 1;
        }
        if has_repetitions(3, id) {
            triples += 1;
        }
    }
    pairs * triples
}

fn strintersection(s_a: &str, s_b: &str) -> String {
    s_a.chars()
        .zip(s_b.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect()
}

pub fn part2(input: &str) -> String {
    let ids = input.split_whitespace().collect::<Vec<_>>();
    for a in &ids {
        for b in &ids {
            let c = strintersection(a, b);
            if c.len() == a.len() - 1 {
                return c;
            }
        }
    }
    unreachable!()
}

pub fn tests() {
    assert_eq!(
        part1("abcdef bababc abbcde abcccd aabcdd abcdee ababab"),
        12
    );
    assert_eq!(part2("abcde fghij klmno pqrst fguij axcye wvxyz"), "fgij");
}
