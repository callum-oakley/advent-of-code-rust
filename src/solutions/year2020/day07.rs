use std::collections::HashMap;

use regex::Regex;

fn parse(input: &str) -> HashMap<&str, Vec<(usize, &str)>> {
    let bag_re: Regex = Regex::new(r"(\w+ \w+) bags contain (.+)").unwrap();
    let contents_re: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    bag_re
        .captures_iter(input)
        .map(|x| {
            (
                x.get(1).unwrap().as_str(),
                contents_re
                    .captures_iter(x.get(2).unwrap().as_str())
                    .map(|y| (y[1].parse().unwrap(), y.get(2).unwrap().as_str()))
                    .collect(),
            )
        })
        .collect()
}

fn contains_gold(bags: &HashMap<&str, Vec<(usize, &str)>>, bag: &str) -> bool {
    bags[bag]
        .iter()
        .any(|&(_, b)| b == "shiny gold" || contains_gold(bags, b))
}

fn count_bags(bags: &HashMap<&str, Vec<(usize, &str)>>, bag: &str) -> usize {
    bags[bag]
        .iter()
        .map(|&(n, b)| n * (1 + count_bags(bags, b)))
        .sum()
}

pub fn part1(input: &str) -> usize {
    let bags = parse(input);
    bags.keys().filter(|&bag| contains_gold(&bags, bag)).count()
}

pub fn part2(input: &str) -> usize {
    count_bags(&parse(input), "shiny gold")
}

pub fn tests() {
    let example1 = "
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
    ";
    let example2 = "
        shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.
    ";
    assert_eq!(part1(example1), 4);
    assert_eq!(part2(example1), 32);
    assert_eq!(part2(example2), 126);
}
