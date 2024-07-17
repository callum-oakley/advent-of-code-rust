use crate::grid::{Grid, Vector, Z};

fn toboggan(trees: &Grid<bool>, slope: Vector) -> usize {
    let mut pos = Z;
    let mut count = 0;
    while pos.y < trees.size.y {
        if trees[pos] {
            count += 1;
        }
        pos += slope;
        pos.x %= trees.size.x;
    }
    count
}

pub fn part1(input: &str) -> usize {
    toboggan(&Grid::parse(input, |_, c| c == '#'), Vector::new(3, 1))
}

pub fn part2(input: &str) -> usize {
    let trees = Grid::parse(input, |_, c| c == '#');
    [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
        .into_iter()
        .map(|slope| toboggan(&trees, slope.into()))
        .product()
}

pub fn tests() {
    let example = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ]
    .join("\n");
    assert_eq!(part1(&example), 7);
    assert_eq!(part2(&example), 336);
}
