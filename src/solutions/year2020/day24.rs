use std::{collections::HashSet, sync::LazyLock};

use regex::Regex;

use crate::{grid::Vector, uniq::Uniq};

// As described here (in the "pointy" orientation):
// <https://www.redblobgames.com/grids/hexagons/#coordinates-axial>
// Let x correspond to q and y correspond to r:
fn into_hex_vector(s: &str) -> Vector {
    match s {
        "ne" => Vector::new(1, -1),
        "e" => Vector::new(1, 0),
        "se" => Vector::new(0, 1),
        "sw" => Vector::new(-1, 1),
        "w" => Vector::new(-1, 0),
        "nw" => Vector::new(0, -1),
        _ => unreachable!(),
    }
}

fn adjacent(tile: Vector) -> impl Iterator<Item = Vector> {
    static DIRS: LazyLock<[Vector; 6]> =
        LazyLock::new(|| ["ne", "e", "se", "sw", "w", "nw"].map(into_hex_vector));
    DIRS.iter().map(move |dir| tile + dir)
}

fn parse(input: &str) -> impl Iterator<Item = Vector> + '_ {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("ne|e|se|sw|w|nw").unwrap());
    input.split_whitespace().map(|instruction| {
        RE.find_iter(instruction)
            .map(|m| into_hex_vector(m.as_str()))
            .sum()
    })
}

fn init(tiles: impl Iterator<Item = Vector>) -> HashSet<Vector> {
    let mut black = HashSet::new();
    for tile in tiles {
        if black.contains(&tile) {
            black.remove(&tile);
        } else {
            black.insert(tile);
        }
    }
    black
}

pub fn part1(input: &str) -> usize {
    init(parse(input)).len()
}

pub fn part2(input: &str) -> usize {
    let mut black = init(parse(input));
    for _ in 0..100 {
        black = black
            .iter()
            .flat_map(|&tile| adjacent(tile))
            .uniq()
            .filter(|&tile| {
                let adjacent_count = adjacent(tile).filter(|a| black.contains(a)).count();
                if black.contains(&tile) {
                    [1, 2].contains(&adjacent_count)
                } else {
                    adjacent_count == 2
                }
            })
            .collect();
    }
    black.len()
}

pub fn tests() {
    let example = "
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
    ";
    assert_eq!(part1(example), 10);
    assert_eq!(part2(example), 2208);
}
