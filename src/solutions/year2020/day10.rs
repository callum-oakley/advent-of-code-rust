use std::collections::HashMap;

fn parse(input: &str) -> Vec<u64> {
    let mut res: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    res.push(0);
    res.push(*res.iter().max().unwrap() + 3);
    res.sort_unstable();
    res
}

fn count_arrangements(cache: &mut HashMap<usize, u64>, adapters: &[u64], i: usize) -> u64 {
    if let Some(&res) = cache.get(&i) {
        return res;
    }

    let res = if i + 1 == adapters.len() {
        1
    } else {
        let options = adapters[i + 1..]
            .iter()
            .take_while(|&adapter| adapter - adapters[i] <= 3)
            .count();
        (0..options)
            .map(|j| count_arrangements(cache, adapters, i + 1 + j))
            .sum()
    };

    cache.insert(i, res);
    res
}

pub fn part1(input: &str) -> u64 {
    let adapters = parse(input);
    let mut diffs: HashMap<u64, u64> = HashMap::new();
    for pair in adapters.windows(2) {
        *diffs.entry(pair[1] - pair[0]).or_default() += 1;
    }
    diffs[&1] * diffs[&3]
}

pub fn part2(input: &str) -> u64 {
    count_arrangements(&mut HashMap::new(), &parse(input), 0)
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
