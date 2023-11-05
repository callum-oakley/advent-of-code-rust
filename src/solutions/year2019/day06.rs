use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<&str, &str> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(')').unwrap();
            (b, a)
        })
        .collect()
}

fn indirect_orbits<'a>(
    direct_orbits: &HashMap<&'a str, &'a str>,
    mut obj: &'a str,
) -> HashSet<&'a str> {
    let mut res = HashSet::new();
    while let Some(&orbit) = direct_orbits.get(obj) {
        res.insert(orbit);
        obj = orbit;
    }
    res
}

pub fn part1(input: &str) -> usize {
    let direct_orbits = parse(input);
    direct_orbits
        .keys()
        .map(|obj| indirect_orbits(&direct_orbits, obj).len())
        .sum()
}

// We have to move along a path consisting of those objects reachable from YOU but not from SAN,
// then one common object, then those reachable from SAN but not YOU. In other words: the symmetric
// difference between the indirect orbits of YOU and the indirect orbits of SAN, plus 1 for the
// common object, but then minus 1 because we're counting transfers not the objects themselves.
pub fn part2(input: &str) -> usize {
    let direct_orbits = parse(input);
    HashSet::symmetric_difference(
        &indirect_orbits(&direct_orbits, "YOU"),
        &indirect_orbits(&direct_orbits, "SAN"),
    )
    .count()
}

pub fn tests() {
    assert_eq!(
        part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
        42,
    );
    assert_eq!(
        part2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"),
        4,
    );
}
