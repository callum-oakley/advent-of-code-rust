#[derive(PartialEq, Eq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

fn round(player: Shape, opponent: Shape) -> Outcome {
    match (player, opponent) {
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => Outcome::Win,
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => Outcome::Loss,
        _ => Outcome::Draw,
    }
}

fn round_score(player: Shape, opponent: Shape) -> u32 {
    let shape_score = match player {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
    let outcome_score = match round(player, opponent) {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    shape_score + outcome_score
}

fn shape_for_outcome(opponent: Shape, outcome: Outcome) -> Shape {
    [Shape::Rock, Shape::Paper, Shape::Scissors]
        .into_iter()
        .find(|&player| round(player, opponent) == outcome)
        .unwrap()
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (opponent, player) = line.split_once(' ').unwrap();
            let opponent = match opponent {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => unreachable!(),
            };
            let player = match player {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => unreachable!(),
            };
            round_score(player, opponent)
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (opponent, outcome) = line.split_once(' ').unwrap();
            let opponent = match opponent {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => unreachable!(),
            };
            let outcome = match outcome {
                "X" => Outcome::Loss,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => unreachable!(),
            };
            round_score(shape_for_outcome(opponent, outcome), opponent)
        })
        .sum()
}

pub fn tests() {
    let example = "A Y\nB X\nC Z";
    assert_eq!(part1(example), 15);
    assert_eq!(part2(example), 12);
}
