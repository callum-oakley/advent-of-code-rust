use crate::part::Part;

fn valid(part: Part, pass: &str) -> bool {
    let pass = pass.as_bytes();
    let mut passes_adjacency_condition = false;
    for i in 0..pass.len() - 1 {
        if pass[i] > pass[i + 1] {
            return false;
        }
        if pass[i] == pass[i + 1] {
            if part == Part::Two
                && (pass.get(i - 1) == Some(&pass[i]) || pass.get(i + 2) == Some(&pass[i]))
            {
                continue;
            }
            passes_adjacency_condition = true;
        }
    }
    passes_adjacency_condition
}

pub fn part1(input: &str) -> usize {
    let (low, high) = input.split_once('-').unwrap();
    (low.parse::<u32>().unwrap()..=high.parse::<u32>().unwrap())
        .filter(|&pass| valid(Part::One, &pass.to_string()))
        .count()
}

pub fn part2(input: &str) -> usize {
    let (low, high) = input.split_once('-').unwrap();
    (low.parse::<u32>().unwrap()..=high.parse::<u32>().unwrap())
        .filter(|&pass| valid(Part::Two, &pass.to_string()))
        .count()
}

pub fn tests() {
    assert!(valid(Part::One, "111111"));
    assert!(!valid(Part::One, "223450"));
    assert!(!valid(Part::One, "123789"));

    assert!(valid(Part::Two, "112233"));
    assert!(!valid(Part::Two, "123444"));
    assert!(valid(Part::Two, "111122"));
}
