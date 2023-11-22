use std::collections::{BTreeSet, HashMap};

use crate::{
    grid::{Point, Rect, E, N, NE, NW, S, SE, SW, W, Z},
    search2::{self, Queue},
};

fn is_open(tile: char) -> bool {
    tile != '#'
}

fn is_key(tile: char) -> bool {
    ['@', '0', '1', '2', '3'].contains(&tile) || tile.is_lowercase()
}

fn is_door(tile: char) -> bool {
    tile.is_uppercase()
}

fn key(door: char) -> char {
    door.to_ascii_lowercase()
}

// Path represents a walk to another key, the steps and keys required.
struct Path {
    dest: char,
    steps: usize,
    keys: BTreeSet<char>,
}

fn reachable(map: &Rect<char>, pos: Point) -> Vec<Path> {
    #[derive(Clone)]
    struct State {
        pos: Point,
        steps: usize,
        keys: BTreeSet<char>,
        found_key: bool,
    }

    let mut res = Vec::new();

    let mut q = search2::breadth_first(
        State {
            pos,
            steps: 0,
            keys: BTreeSet::new(),
            found_key: false,
        },
        |state| state.pos,
    );

    while let Some(state) = q.pop() {
        if state.found_key {
            res.push(Path {
                dest: map[state.pos],
                steps: state.steps,
                keys: state.keys,
            });
            continue;
        }
        for pos in state
            .pos
            .adjacent4()
            .into_iter()
            .filter(|pos| is_open(map[pos]))
        {
            let mut state = state.clone();
            state.pos = pos;
            state.steps += 1;
            if is_door(map[pos]) {
                state.keys.insert(key(map[pos]));
            } else if is_key(map[pos]) {
                state.found_key = true;
            }
            q.push(state);
        }
    }

    res
}

fn key_graph(map: &Rect<char>) -> HashMap<char, Vec<Path>> {
    map.iter()
        .filter(|(_, &tile)| is_key(tile))
        .map(|(pos, &tile)| (tile, reachable(map, pos)))
        .collect()
}

fn part_(map: &Rect<char>, robots: Vec<char>) -> usize {
    #[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
    struct State {
        robots: Vec<char>,
        steps: usize,
        keys: BTreeSet<char>,
    }

    // First precompute the paths between keys (the number of steps and the keys required to
    // traverse them).
    let key_graph = key_graph(map);

    let mut initial_keys = BTreeSet::new();
    initial_keys.extend(&robots);

    let final_key_count = map.values().filter(|&&tile| is_key(tile)).count();
    let min_path_steps = key_graph
        .values()
        .flatten()
        .map(|path| path.steps)
        .min()
        .unwrap();

    // Then A* on this higher level graph is fast enough.
    let mut q = search2::a_star(
        State {
            robots,
            steps: 0,
            keys: initial_keys,
        },
        |state| (state.robots.clone(), state.keys.clone()),
        |state| state.steps,
        // For each key left to collect, we'll have to move at least min_path_steps.
        |state| (final_key_count - state.keys.len()) * min_path_steps,
    );

    while let Some(state) = q.pop() {
        if state.keys.len() == final_key_count {
            return state.steps;
        }
        for i in 0..state.robots.len() {
            for path in key_graph[&state.robots[i]]
                .iter()
                .filter(|path| path.keys.is_subset(&state.keys))
            {
                let mut state = state.clone();
                state.robots[i] = path.dest;
                state.steps += path.steps;
                state.keys.insert(path.dest);
                q.push(state);
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> usize {
    part_(&Rect::parse(input, |_, tile| tile), vec!['@'])
}

pub fn part2(input: &str) -> usize {
    let mut map = Rect::parse(input, |_, tile| tile);
    let (start, _) = map.iter().find(|(_, &tile)| tile == '@').unwrap();
    for dir in [Z, N, E, S, W] {
        map[start + dir] = '#';
    }
    map[start + NE] = '0';
    map[start + SE] = '1';
    map[start + SW] = '2';
    map[start + NW] = '3';
    part_(&map, vec!['0', '1', '2', '3'])
}
