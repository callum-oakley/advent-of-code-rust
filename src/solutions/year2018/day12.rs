use std::collections::HashSet;

fn parse(input: &str) -> (HashSet<&[u8]>, String) {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    (
        bottom
            .lines()
            .filter_map(|line| {
                let (from, to) = line.split_once(" => ").unwrap();
                if to == "#" {
                    Some(from.trim().as_bytes())
                } else {
                    None
                }
            })
            .collect(),
        format!("....{}....", top.split_once(": ").unwrap().1),
    )
}

fn step(rules: &HashSet<&[u8]>, state: &str) -> String {
    let mut res = String::from("....");
    for window in state.as_bytes().windows(5) {
        if rules.contains(window) {
            res.push('#');
        } else {
            res.push('.');
        }
    }
    res.push_str("....");
    res
}

fn score(generation: i32, state: &str) -> i32 {
    let mut res = 0;
    let mut i = -4 - generation * 2;
    for c in state.chars() {
        if c == '#' {
            res += i;
        }
        i += 1;
    }
    res
}

pub fn part1(input: &str) -> i32 {
    let (rules, mut state) = parse(input);
    for _ in 0..20 {
        state = step(&rules, &state);
    }
    score(20, &state)
}

// Printing successive values, observe that the score appears to approach a
// constant factor of the generation. Wait for this factor to stabilise and then
// multiply out.
pub fn part2(input: &str) -> f64 {
    let (rules, mut state) = parse(input);
    let mut gen = 0;
    let mut prev_factor = -1f64;
    let mut factor = 0f64;
    while (factor - prev_factor).abs() >= f64::EPSILON {
        state = step(&rules, &state);
        gen += 1;
        prev_factor = factor;
        factor = f64::from(score(gen, &state)) / f64::from(gen);
    }
    factor * 50_000_000_000f64
}

pub fn tests() {
    let example = "initial state: #..#.#..##......###...###\n
        ...## => #\n..#.. => #\n.#... => #\n.#.#. => #\n.#.## => #\n.##.. => #\n.#### => #
        #.#.# => #\n#.### => #\n##.#. => #\n##.## => #\n###.. => #\n###.# => #\n####. => #";
    assert_eq!(part1(example), 325);
}
