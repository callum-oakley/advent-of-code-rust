use std::{
    collections::{BTreeSet, HashMap},
    iter,
};

use crate::{
    grid::{Point, Rect, E, N, NE, NW, S, SE, SW, W, Z},
    search,
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

#[derive(Clone)]
struct LowState<'a> {
    map: &'a Rect<char>,
    pos: Point,
    steps: u32,
    keys: BTreeSet<char>,
    found_key: bool,
}

impl<'a> search::State for LowState<'a> {
    type HashKey = Point;

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        // Only go as far as the first key
        if self.found_key {
            return Box::new(iter::empty());
        }
        Box::new(
            self.pos
                .adjacent4()
                .into_iter()
                .filter(|pos| is_open(self.map[pos]))
                .map(|pos| {
                    let mut state = self.clone();
                    state.pos = pos;
                    state.steps += 1;
                    if is_door(state.map[pos]) {
                        state.keys.insert(key(state.map[pos]));
                    } else if is_key(state.map[pos]) {
                        state.found_key = true;
                    }
                    state
                }),
        )
    }

    fn hash_key(&self) -> Self::HashKey {
        self.pos
    }
}

struct MidState {
    pos: char,
    steps: u32,
    keys: BTreeSet<char>,
}

fn key_graph(map: &Rect<char>) -> HashMap<char, Vec<MidState>> {
    map.iter()
        .filter(|(_, &tile)| is_key(tile))
        .map(|(pos, &tile)| {
            (
                tile,
                search::breadth_first(LowState {
                    map,
                    pos,
                    steps: 0,
                    keys: BTreeSet::new(),
                    found_key: false,
                })
                .filter(|state| state.found_key)
                .map(|state| MidState {
                    pos: map[state.pos],
                    steps: state.steps,
                    keys: state.keys,
                })
                .collect(),
            )
        })
        .collect()
}

#[derive(Clone)]
struct HighState<'a> {
    key_graph: &'a HashMap<char, Vec<MidState>>,
    robots: Vec<char>,
    steps: u32,
    keys: BTreeSet<char>,
}

impl<'a> search::State for HighState<'a> {
    type HashKey = (Vec<char>, BTreeSet<char>);

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new((0..self.robots.len()).flat_map(move |i| {
            self.key_graph[&self.robots[i]]
                .iter()
                .filter(|ms| ms.keys.is_subset(&self.keys))
                .map(move |ms| {
                    let mut state = self.clone();
                    state.robots[i] = ms.pos;
                    state.steps += ms.steps;
                    state.keys.insert(ms.pos);
                    state
                })
        }))
    }

    fn hash_key(&self) -> Self::HashKey {
        (self.robots.clone(), self.keys.clone())
    }
}

impl<'a> search::OrdKey for HighState<'a> {
    type OrdKey = u32;

    fn ord_key(&self) -> Self::OrdKey {
        self.steps
    }
}

fn part_(map: &Rect<char>, robots: Vec<char>) -> u32 {
    let key_graph = key_graph(map);
    let mut initial_keys = BTreeSet::new();
    initial_keys.extend(&robots);
    let final_keys: BTreeSet<char> = map.values().copied().filter(|&tile| is_key(tile)).collect();
    search::min_first(HighState {
        key_graph: &key_graph,
        robots,
        steps: 0,
        keys: initial_keys,
    })
    .find(|state| state.keys == final_keys)
    .unwrap()
    .steps
}

pub fn part1(input: &str) -> u32 {
    part_(&Rect::parse(input, |_, tile| tile), vec!['@'])
}

pub fn part2(input: &str) -> u32 {
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
