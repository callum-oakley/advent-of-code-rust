use std::collections::HashMap;

fn parse(input: &str) -> Vec<i64> {
    let mut res: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    res.push(0);
    res.push(*res.iter().max().unwrap() + 3);
    res.sort_unstable();
    res
}

pub fn part1(input: &str) -> i64 {
    let adapters = parse(input);
    let mut diffs: HashMap<i64, i64> = HashMap::new();
    for pair in adapters.windows(2) {
        *diffs.entry(pair[1] - pair[0]).or_default() += 1;
    }
    diffs[&1] * diffs[&3]
}

pub fn part2(input: &str) -> i64 {
    // The number of routes to each adapter is the sum of the number of routes to each of the
    // possible previous adapters (it's like a messy Pascal's triangle).
    let adapters = parse(input);
    let mut routes: HashMap<i64, i64> = HashMap::from([(0, 1)]);
    for &a in &adapters[1..] {
        routes.insert(a, (a - 3..a).filter_map(|b| routes.get(&b)).sum());
    }
    routes[&adapters[adapters.len() - 1]]
}

pub fn tests() {
    let example1 = "16 10 15 5 1 11 7 19 6 12 4";
    let example2 =
        "28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3";
    assert_eq!(part1(example1), 35);
    assert_eq!(part1(example2), 220);
    assert_eq!(part2(example1), 8);
    assert_eq!(part2(example2), 19208);
}
