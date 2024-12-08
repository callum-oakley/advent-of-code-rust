use std::collections::{HashMap, HashSet};

use crate::{
    grid::{Grid, Vector},
    part::Part,
};

fn parse(input: &str) -> (HashMap<char, Vec<Vector>>, Grid<()>) {
    let mut antennas: HashMap<char, Vec<Vector>> = HashMap::new();
    let grid = Grid::parse(input, |v, c| {
        if c != '.' {
            antennas.entry(c).or_default().push(v);
        }
    });
    (antennas, grid)
}

fn part_(part: Part, input: &str) -> usize {
    let (antennas, grid) = parse(input);

    let mut antinodes = HashSet::new();
    for nodes in antennas.values() {
        for &a in nodes {
            for &b in nodes {
                if a != b {
                    match part {
                        Part::One => {
                            if grid.contains_key(2 * a - b) {
                                antinodes.insert(2 * a - b);
                            }
                        }
                        Part::Two => {
                            let mut antinode = a;
                            while grid.contains_key(antinode) {
                                antinodes.insert(antinode);
                                antinode += a - b;
                            }
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

pub fn part1(input: &str) -> usize {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = [
        "............",
        "........0...",
        ".....0......",
        ".......0....",
        "....0.......",
        "......A.....",
        "............",
        "............",
        "........A...",
        ".........A..",
        "............",
        "............",
    ]
    .join("\n");
    assert_eq!(part1(&example), 14);
    assert_eq!(part2(&example), 34);
}
