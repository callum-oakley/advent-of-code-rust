use crate::{combinatorics, grid::Grid};

fn parse(input: &str) -> Vec<Grid<bool>> {
    input
        .split("\n\n")
        .map(|schematic| Grid::parse(schematic, |_, c| c == '#'))
        .collect()
}

pub fn part1(input: &str) -> usize {
    combinatorics::combinations(2, &parse(input))
        .filter(|p| !p[0].values().zip(p[1].values()).any(|(&a, &b)| a && b))
        .count()
}

pub fn tests() {
    let example = [
        "#####", ".####", ".####", ".####", ".#.#.", ".#...", ".....", "", "#####", "##.##",
        ".#.##", "...##", "...#.", "...#.", ".....", "", ".....", "#....", "#....", "#...#",
        "#.#.#", "#.###", "#####", "", ".....", ".....", "#.#..", "###..", "###.#", "###.#",
        "#####", "", ".....", ".....", ".....", "#....", "#.#..", "#.#.#", "#####",
    ]
    .join("\n");
    assert_eq!(part1(&example), 3);
}
