use crate::part::Part;

fn part_(part: Part, input: &str) -> usize {
    let mut size = input.parse().unwrap();

    // Zero-index elves because it makes the maths nicer.
    // left[a] is the elf to the left of a.
    let mut left = Vec::with_capacity(size);
    for i in 0..size {
        left.push((i + 1) % size);
    }

    // The elf to the right of the next victim.
    let mut right_of_victim = match part {
        Part::One => 0,
        Part::Two => size / 2 - 1,
    };

    while size > 1 {
        left[right_of_victim] = left[left[right_of_victim]];
        if part == Part::One || size % 2 == 1 {
            right_of_victim = left[right_of_victim];
        }
        size -= 1;
    }

    // Convert back to one-indexing.
    right_of_victim + 1
}

pub fn part1(input: &str) -> usize {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, input)
}

pub fn tests() {
    assert_eq!(part1("5"), 3);
    assert_eq!(part2("5"), 2);
}
