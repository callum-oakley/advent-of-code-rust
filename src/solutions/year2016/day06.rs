use crate::freqs::Freqs;

#[derive(Clone, Copy)]
enum Part {
    One,
    Two,
}

fn part_(part: Part, input: &str) -> String {
    let messages: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();
    let mut res = String::new();
    for i in 0..messages[0].len() {
        let freqs = messages.iter().map(|m| m[i]).freqs();
        res.push(match part {
            Part::One => *freqs.keys().max_by_key(|c| freqs[c]).unwrap(),
            Part::Two => *freqs.keys().min_by_key(|c| freqs[c]).unwrap(),
        });
    }
    res
}

pub fn part1(input: &str) -> String {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> String {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = "eedadn drvtee eandsr raavrd atevrs tsrnev sdttsa rasrtv
                   nssdts ntnada svetve tesnvt vntsnd vrdear dvrsen enarar";
    assert_eq!(part1(example), "easter");
    assert_eq!(part2(example), "advent");
}
