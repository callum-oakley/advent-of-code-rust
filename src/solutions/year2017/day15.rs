use regex::Regex;

struct Gen {
    factor: u64,
    prev: u64,
}

impl Iterator for Gen {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev = self.prev * self.factor % 2_147_483_647;
        Some(self.prev)
    }
}

fn part_<F, G>(criteria_a: F, criteria_b: G, sample: usize, input: &str) -> usize
where
    F: Fn(&u64) -> bool,
    G: Fn(&u64) -> bool,
{
    let re = Regex::new(r"\d+").unwrap();
    let mut matches = re.find_iter(input);
    let gen_a = Gen {
        factor: 16807,
        prev: matches.next().unwrap().as_str().parse().unwrap(),
    };
    let gen_b = Gen {
        factor: 48271,
        prev: matches.next().unwrap().as_str().parse().unwrap(),
    };

    gen_a
        .filter(criteria_a)
        .zip(gen_b.filter(criteria_b))
        .take(sample)
        .filter(|&(a, b)| a % (1 << 16) == b % (1 << 16))
        .count()
}

pub fn part1(input: &str) -> usize {
    part_(|_| true, |_| true, 40_000_000, input)
}

pub fn part2(input: &str) -> usize {
    part_(|&a| a % 4 == 0, |&b| b % 8 == 0, 5_000_000, input)
}

pub fn tests() {
    assert_eq!(part1("65 8921"), 588);
    assert_eq!(part2("65 8921"), 309);
}
