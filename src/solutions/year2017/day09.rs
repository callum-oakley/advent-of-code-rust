use crate::part::Part;

#[derive(Clone, Copy)]
enum State {
    Normal,
    Garbage,
    Escape,
}

fn part_(part: Part, input: &str) -> u32 {
    let mut score = 0;
    let mut garbage = 0;
    let mut depth = 0;
    let mut state = State::Normal;
    for c in input.chars() {
        match (state, c) {
            (State::Normal, '{') => {
                depth += 1;
            }
            (State::Normal, '}') => {
                score += depth;
                depth -= 1;
            }
            (State::Normal, '<') | (State::Escape, _) => {
                state = State::Garbage;
            }
            (State::Garbage, '>') => {
                state = State::Normal;
            }
            (State::Garbage, '!') => {
                state = State::Escape;
            }
            (State::Garbage, _) => {
                garbage += 1;
            }
            _ => (),
        }
    }
    match part {
        Part::One => score,
        Part::Two => garbage,
    }
}

pub fn part1(input: &str) -> u32 {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> u32 {
    part_(Part::Two, input)
}

pub fn tests() {
    assert_eq!(part1("{}"), 1);
    assert_eq!(part1("{{{}}}"), 6);
    assert_eq!(part1("{{},{}}"), 5);
    assert_eq!(part1("{{{},{},{{}}}}"), 16);
    assert_eq!(part1("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);

    assert_eq!(part2("<>"), 0);
    assert_eq!(part2("<random characters>"), 17);
    assert_eq!(part2("<<<<>"), 3);
    assert_eq!(part2("<{!>}>"), 2);
    assert_eq!(part2("<!!>"), 0);
    assert_eq!(part2("<!!!>>"), 0);
    assert_eq!(part2(r#"<{o"i!a,<{i<a>"#), 10);
}
