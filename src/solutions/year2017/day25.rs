use std::collections::HashMap;

use regex::Regex;

type State = u8;

#[derive(Clone, Copy)]
enum Dir {
    Left,
    Right,
}

struct Transition {
    write: bool,
    dir: Dir,
    next_state: State,
}

fn parse(input: &str) -> (State, usize, HashMap<(State, bool), Transition>) {
    let final_word = Regex::new(r"(?m)(\w+)[.:]$").unwrap();
    let steps_re = Regex::new(r"Perform a diagnostic checksum after ([0-9]+) steps\.").unwrap();

    let mut paragraphs = input.split("\n\n");

    let header = paragraphs.next().unwrap();

    let mut transitions = HashMap::new();

    for paragraph in paragraphs {
        let mut final_words = final_word.captures_iter(paragraph);
        let state_from = final_words.next().unwrap()[1].as_bytes()[0];

        for value in [false, true] {
            final_words.next(); // If the current value is...
            transitions.insert(
                (state_from, value),
                Transition {
                    write: &final_words.next().unwrap()[1] == "1",
                    dir: match &final_words.next().unwrap()[1] {
                        "left" => Dir::Left,
                        "right" => Dir::Right,
                        _ => unreachable!(),
                    },
                    next_state: final_words.next().unwrap()[1].as_bytes()[0],
                },
            );
        }
    }

    (
        final_word.captures(header).unwrap()[1].as_bytes()[0],
        steps_re.captures(header).unwrap()[1].parse().unwrap(),
        transitions,
    )
}

pub fn part1(input: &str) -> usize {
    let (mut state, steps, transitions) = parse(input);
    let mut tape = vec![false; 10_000];
    let mut cursor = tape.len() / 2;

    for _ in 0..steps {
        let transition = &transitions[&(state, tape[cursor])];
        tape[cursor] = transition.write;
        cursor = match transition.dir {
            Dir::Left => cursor - 1,
            Dir::Right => cursor + 1,
        };
        state = transition.next_state;
    }

    tape.into_iter().filter(|&x| x).count()
}

pub fn tests() {
    let example = "Begin in state A.
                   Perform a diagnostic checksum after 6 steps.

                   In state A:
                     If the current value is 0:
                       - Write the value 1.
                       - Move one slot to the right.
                       - Continue with state B.
                     If the current value is 1:
                       - Write the value 0.
                       - Move one slot to the left.
                       - Continue with state B.

                   In state B:
                     If the current value is 0:
                       - Write the value 1.
                       - Move one slot to the left.
                       - Continue with state A.
                     If the current value is 1:
                       - Write the value 1.
                       - Move one slot to the right.
                       - Continue with state A.";
    assert_eq!(part1(example), 3);
}
