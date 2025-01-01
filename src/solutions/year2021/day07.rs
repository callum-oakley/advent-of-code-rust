use crate::search;

fn part_<C: FnMut(&[u32], u32) -> u32>(input: &str, mut cost: C) -> u32 {
    let crabs: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let target = search::binary(min, max, |target| {
        cost(&crabs, target) < cost(&crabs, target + 1)
    });
    cost(&crabs, target)
}

pub fn part1(input: &str) -> u32 {
    part_(input, |crabs, target| {
        crabs.iter().map(|&crab| target.abs_diff(crab)).sum()
    })
}

pub fn part2(input: &str) -> u32 {
    part_(input, |crabs, target| {
        crabs
            .iter()
            .map(|&crab| {
                let dist = target.abs_diff(crab);
                dist * (dist + 1) / 2
            })
            .sum()
    })
}

pub fn tests() {
    let example = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(part1(example), 37);
    assert_eq!(part2(example), 168);
}
