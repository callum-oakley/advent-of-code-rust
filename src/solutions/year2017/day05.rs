use crate::part::Part;

fn part_(part: Part, input: &str) -> usize {
    let mut mem = input
        .split_whitespace()
        .map(|offset| offset.parse().unwrap())
        .collect::<Vec<i32>>();
    let mut count = 0;
    let mut ip = 0i32;
    while ip >= 0 && (usize::try_from(ip).unwrap()) < mem.len() {
        let offset = mem[usize::try_from(ip).unwrap()];
        if part == Part::Two && offset >= 3 {
            mem[usize::try_from(ip).unwrap()] -= 1;
        } else {
            mem[usize::try_from(ip).unwrap()] += 1;
        }
        ip += offset;
        count += 1;
    }
    count
}

pub fn part1(input: &str) -> usize {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, input)
}

pub fn tests() {
    assert_eq!(part1("0 3 0 1 -3"), 5);
    assert_eq!(part2("0 3 0 1 -3"), 10);
}
