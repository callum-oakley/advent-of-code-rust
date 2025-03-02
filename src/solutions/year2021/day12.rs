use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut res: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.trim().lines() {
        let (a, b) = line.trim().split_once('-').unwrap();
        res.entry(a).or_default().push(b);
        res.entry(b).or_default().push(a);
    }
    res
}

fn paths<'a>(
    cave: &HashMap<&'a str, Vec<&'a str>>,
    mut visited: HashSet<&'a str>,
    a: &'a str,
    b: &'a str,
    bonus_visit: bool,
) -> usize {
    if a == b {
        return 1;
    }
    visited.insert(a);
    cave[a]
        .iter()
        .filter_map(|c| {
            if !visited.contains(c) || c.chars().all(char::is_uppercase) {
                Some(paths(cave, visited.clone(), c, b, bonus_visit))
            } else if bonus_visit && *c != "start" {
                Some(paths(cave, visited.clone(), c, b, false))
            } else {
                None
            }
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    paths(&parse(input), HashSet::new(), "start", "end", false)
}

pub fn part2(input: &str) -> usize {
    paths(&parse(input), HashSet::new(), "start", "end", true)
}

pub fn tests() {
    let example0 = "
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ";
    let example1 = "
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    ";
    let example2 = "
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    ";
    assert_eq!(part1(example0), 10);
    assert_eq!(part1(example1), 19);
    assert_eq!(part1(example2), 226);
    assert_eq!(part2(example0), 36);
    assert_eq!(part2(example1), 103);
    assert_eq!(part2(example2), 3509);
}
