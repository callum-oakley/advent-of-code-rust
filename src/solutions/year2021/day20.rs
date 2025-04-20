use std::collections::HashSet;

use crate::{
    grid::{Adjacent, Vector},
    uniq::Uniq,
};

struct Image {
    pixels: HashSet<Vector>,
    invert: bool,
}

fn parse(input: &str) -> (Vec<bool>, Image) {
    let (algorithm, pixels) = input.split_once("\n\n").unwrap();
    (
        algorithm.chars().map(|c| c == '#').collect(),
        Image {
            pixels: crate::grid::scan(pixels)
                .filter(|&(_, c)| c == '#')
                .map(|(v, _)| v)
                .collect(),
            invert: false,
        },
    )
}

fn index(image: &Image, v: Vector) -> usize {
    v.adjacent9()
        .enumerate()
        .filter(|&(_, v)| image.pixels.contains(&v) ^ image.invert)
        .map(|(i, _)| 2_usize.pow((8 - i).try_into().unwrap()))
        .sum()
}

fn step(algorithm: &[bool], image: &mut Image) {
    image.pixels = image
        .pixels
        .iter()
        .flat_map(|v| v.adjacent9())
        .uniq()
        .filter(|&v| algorithm[index(image, v)] ^ image.invert ^ algorithm[0])
        .collect();

    image.invert ^= algorithm[0];
}

fn part_(steps: usize, input: &str) -> usize {
    let (algorithm, mut image) = parse(input);
    for _ in 0..steps {
        step(&algorithm, &mut image);
    }
    image.pixels.len()
}

pub fn part1(input: &str) -> usize {
    part_(2, input)
}

pub fn part2(input: &str) -> usize {
    part_(50, input)
}

pub fn tests() {
    let example = concat!(
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##",
        "#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###",
        ".######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.",
        ".#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....",
        ".#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..",
        "...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....",
        "..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n",
        "\n",
        "#..#.\n",
        "#....\n",
        "##..#\n",
        "..#..\n",
        "..###\n",
    );
    assert_eq!(part1(example), 35);
    assert_eq!(part2(example), 3351);
}
