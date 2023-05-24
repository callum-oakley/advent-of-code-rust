use lazy_static::lazy_static;
use regex::Regex;

fn part_(recursive: bool, input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([A-Z]*)\((\d+)x(\d+)\)+(.*)$").unwrap();
    }
    if let Some(captures) = RE.captures(input) {
        let scope: usize = captures[2].parse().unwrap();
        let reps: usize = captures[3].parse().unwrap();
        captures[1].len()
            + reps
                * (if recursive {
                    part_(recursive, &captures[4][..scope])
                } else {
                    scope
                })
            + part_(recursive, &captures[4][scope..])
    } else {
        input.len()
    }
}

pub fn part1(input: &str) -> usize {
    part_(false, input)
}

pub fn part2(input: &str) -> usize {
    part_(true, input)
}

pub fn tests() {
    assert_eq!(part1("ADVENT"), 6);
    assert_eq!(part1("A(1x5)BC"), 7);
    assert_eq!(part1("(3x3)XYZ"), 9);
    assert_eq!(part1("A(2x2)BCD(2x2)EFG"), 11);
    assert_eq!(part1("(6x1)(1x3)A"), 6);
    assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);

    assert_eq!(part2("(3x3)XYZ"), 9);
    assert_eq!(part2("X(8x2)(3x3)ABCY"), 20);
    assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241_920);
    assert_eq!(
        part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
        445
    );
}
