use std::{cmp::Reverse, collections::HashMap, sync::LazyLock};

use regex::Regex;

struct Room<'a> {
    name: &'a str,
    sector: u32,
    checksum: &'a str,
}

impl<'a> Room<'a> {
    fn new(s: &'a str) -> Self {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"([a-z\-]+)-(\d+)\[([a-z]{5})\]").unwrap());
        let captures = RE.captures(s).unwrap();
        Self {
            name: captures.get(1).unwrap().as_str(),
            sector: captures[2].parse().unwrap(),
            checksum: captures.get(3).unwrap().as_str(),
        }
    }

    fn is_real(&self) -> bool {
        let mut freqs = HashMap::new();
        for c in self.name.chars().filter(|c| *c != '-') {
            *freqs.entry(c).or_insert(0) += 1;
        }
        let mut keys = freqs.keys().copied().collect::<Vec<_>>();
        keys.sort_by_key(|k| (Reverse(freqs[k]), *k));
        keys.into_iter().take(5).eq(self.checksum.chars())
    }

    fn decrypt(&self) -> String {
        self.name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    let n = (u32::from(c) - u32::from('a') + self.sector) % 26;
                    char::from_u32(n + u32::from('a')).unwrap()
                }
            })
            .collect()
    }
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Room::new)
        .filter(Room::is_real)
        .map(|r| r.sector)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(Room::new)
        .filter(Room::is_real)
        .find(|r| r.decrypt() == "northpole object storage")
        .unwrap()
        .sector
}

pub fn tests() {
    assert!(Room::new("aaaaa-bbb-z-y-x-123[abxyz]").is_real());
    assert!(Room::new("a-b-c-d-e-f-g-h-987[abcde]").is_real());
    assert!(Room::new("not-a-real-room-404[oarel]").is_real());
    assert!(!Room::new("totally-real-room-200[decoy]").is_real());
    assert_eq!(
        Room::new("qzmt-zixmtkozy-ivhz-343[xxxxx]").decrypt(),
        "very encrypted name"
    );
}
