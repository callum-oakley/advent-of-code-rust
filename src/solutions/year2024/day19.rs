use std::collections::HashMap;

fn parse(input: &str) -> (Vec<&str>, impl Iterator<Item = &str> + '_) {
    let (towels, designs) = input.trim().split_once("\n\n").unwrap();
    (towels.split(", ").collect(), designs.split_whitespace())
}

fn arrangements<'a>(
    cache: &mut HashMap<&'a str, usize>,
    towels: &[&'a str],
    design: &'a str,
) -> usize {
    fn go<'a>(cache: &mut HashMap<&'a str, usize>, towels: &[&'a str], design: &'a str) -> usize {
        if design.is_empty() {
            1
        } else {
            towels
                .iter()
                .filter(|&towel| design.starts_with(towel))
                .map(|towel| arrangements(cache, towels, &design[towel.len()..]))
                .sum()
        }
    }

    if let Some(&res) = cache.get(design) {
        res
    } else {
        let res = go(cache, towels, design);
        cache.insert(design, res);
        res
    }
}

pub fn part1(input: &str) -> usize {
    let mut cache = HashMap::new();
    let (towels, designs) = parse(input);
    designs
        .filter(|design| arrangements(&mut cache, &towels, design) > 0)
        .count()
}

pub fn part2(input: &str) -> usize {
    let mut cache = HashMap::new();
    let (towels, designs) = parse(input);
    designs
        .map(|design| arrangements(&mut cache, &towels, design))
        .sum()
}

pub fn tests() {
    let example = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr bggr gbbr rrbgbr ubwu bwurrg brgr bbrgwb
    ";
    assert_eq!(part1(example), 6);
    assert_eq!(part2(example), 16);
}
