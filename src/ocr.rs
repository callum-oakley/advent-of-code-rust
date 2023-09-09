const CFLELOYFCS: &str = "
    .##..####.#....####.#.....##..#...#####..##...###.
    #..#.#....#....#....#....#..#.#...##....#..#.#....
    #....###..#....###..#....#..#..#.#.###..#....#....
    #....#....#....#....#....#..#...#..#....#.....##..
    #..#.#....#....#....#....#..#...#..#....#..#....#.
    .##..#....####.####.####..##....#..#.....##..###..
";

const HI: &str = "
    #...#..###
    #...#...#.
    #...#...#.
    #####...#.
    #...#...#.
    #...#...#.
    #...#...#.
    #...#..###
";

const FBHKLEAG: &str = "
    ######..#####...#....#..#....#..#.......######....##.....####.
    #.......#....#..#....#..#...#...#.......#........#..#...#....#
    #.......#....#..#....#..#..#....#.......#.......#....#..#.....
    #.......#....#..#....#..#.#.....#.......#.......#....#..#.....
    #####...#####...######..##......#.......#####...#....#..#.....
    #.......#....#..#....#..##......#.......#.......######..#..###
    #.......#....#..#....#..#.#.....#.......#.......#....#..#....#
    #.......#....#..#....#..#..#....#.......#.......#....#..#....#
    #.......#....#..#....#..#...#...#.......#.......#....#..#...##
    #.......#####...#....#..#....#..######..######..#....#...###.#
";

pub fn parse(s: &str) -> &'static str {
    let s = s.trim();
    if s == CFLELOYFCS.trim().replace(' ', "") {
        "CFLELOYFCS"
    } else if s == HI.trim().replace(' ', "") {
        "HI"
    } else if s == FBHKLEAG.trim().replace(' ', "") {
        "FBHKLEAG"
    } else {
        panic!("failed to parse:\n{s}")
    }
}
