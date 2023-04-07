use regex::Regex;

fn parse(input: &str) -> Vec<[u32; 3]> {
    Regex::new(r"(\d+)x(\d+)x(\d+)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| {
            let mut sides: [u32; 3] = [
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ];
            sides.sort();
            sides
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|[l, w, h]| 3 * l * w + 2 * w * h + 2 * h * l)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|[l, w, h]| 2 * (l + w) + l * w * h)
        .sum()
}
