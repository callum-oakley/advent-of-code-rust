use std::{collections::HashMap, sync::LazyLock};

use crate::{
    grid::{self, IntoVector, Vector},
    search,
};

fn parse_keypad(s: &str) -> HashMap<char, Vector> {
    let mut res = HashMap::new();
    grid::scan(s, |v, c| {
        if c != ' ' {
            res.insert(c, v);
        }
    });
    res
}

static NUM_KEYPAD: LazyLock<HashMap<char, Vector>> =
    LazyLock::new(|| parse_keypad("789\n456\n123\n 0A"));

static DIR_KEYPAD: LazyLock<HashMap<char, Vector>> = LazyLock::new(|| parse_keypad(" ^A\n<v>"));

fn key_cost(
    cache: &mut HashMap<(usize, char, char), usize>,
    keypad: &HashMap<char, Vector>,
    depth: usize,
    start: char,
    end: char,
) -> usize {
    fn go(
        cache: &mut HashMap<(usize, char, char), usize>,
        keypad: &HashMap<char, Vector>,
        depth: usize,
        start: char,
        end: char,
    ) -> usize {
        struct State {
            pos: Vector,
            code: String,
        }
        if depth == 0 {
            1
        } else {
            let code = search::dijkstra(
                State {
                    pos: keypad[&start],
                    code: String::new(),
                },
                |state, push| {
                    for dir in "^>v<".chars() {
                        let pos = state.pos + dir.into_vector();
                        if keypad.values().any(|&v| v == pos) {
                            let mut code = state.code.clone();
                            code.push(dir);
                            push(State { pos, code });
                        }
                    }
                },
                search::no_filter,
                |state| code_cost(cache, &DIR_KEYPAD, depth - 1, &state.code),
            )
            .find(|state| state.pos == keypad[&end])
            .unwrap()
            .code;
            code_cost(cache, &DIR_KEYPAD, depth - 1, &code)
        }
    }

    if let Some(&res) = cache.get(&(depth, start, end)) {
        res
    } else {
        let res = go(cache, keypad, depth, start, end);
        cache.insert((depth, start, end), res);
        res
    }
}

fn code_cost(
    cache: &mut HashMap<(usize, char, char), usize>,
    keypad: &HashMap<char, Vector>,
    depth: usize,
    code: &str,
) -> usize {
    let mut res = 0;
    let mut start = 'A';
    for c in code.chars() {
        res += key_cost(cache, keypad, depth, start, c);
        start = c;
    }
    res + key_cost(cache, keypad, depth, start, 'A')
}

pub fn part1(input: &str) -> usize {
    input
        .split_whitespace()
        .map(|code| {
            let code = &code[..code.len() - 1];
            code_cost(&mut HashMap::new(), &NUM_KEYPAD, 3, code) * code.parse::<usize>().unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .split_whitespace()
        .map(|code| {
            let code = &code[..code.len() - 1];
            code_cost(&mut HashMap::new(), &NUM_KEYPAD, 26, code) * code.parse::<usize>().unwrap()
        })
        .sum()
}

pub fn tests() {
    let example = "029A 980A 179A 456A 379A";
    assert_eq!(part1(example), 126_384);
}
