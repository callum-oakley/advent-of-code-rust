use crate::grid::{Adjacent, Grid, Vector};

fn excite(octopuses: &mut Grid<u32>, v: Vector) -> usize {
    octopuses[v] += 1;
    if octopuses[v] == 10 {
        let mut flashes = 1;
        for a in v.adjacent8() {
            if octopuses.contains_key(a) {
                flashes += excite(octopuses, a);
            }
        }
        flashes
    } else {
        0
    }
}

fn step(octopuses: &mut Grid<u32>) -> usize {
    let mut flashes = 0;
    for v in octopuses.keys() {
        flashes += excite(octopuses, v);
    }
    for v in octopuses.keys() {
        if octopuses[v] > 9 {
            octopuses[v] = 0;
        }
    }
    flashes
}

pub fn part1(input: &str) -> usize {
    let mut octopuses = Grid::parse(input, |_, c| c.to_digit(10).unwrap());
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut octopuses);
    }
    flashes
}

pub fn part2(input: &str) -> usize {
    let mut octopuses = Grid::parse(input, |_, c| c.to_digit(10).unwrap());
    let mut steps = 1;
    while step(&mut octopuses) < octopuses.len() {
        steps += 1;
    }
    steps
}

pub fn tests() {
    let example = [
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ]
    .join("\n");
    assert_eq!(part1(&example), 1656);
    assert_eq!(part2(&example), 195);
}
