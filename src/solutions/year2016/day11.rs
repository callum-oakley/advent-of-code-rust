use regex::Regex;

use crate::{
    combinatorics,
    search2::{self, Queue},
};

#[derive(Clone, Copy)]
enum ItemKind {
    Microchip,
    Generator,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    microchip: u8,
    generator: u8,
}

fn parse(input: &str) -> Vec<Item> {
    let microchip_re = Regex::new(r"([a-z]+)-compatible microchip").unwrap();
    let generator_re = Regex::new(r"([a-z]+) generator").unwrap();

    let mut microchips: Vec<(&str, u8)> = Vec::new();
    let mut generators: Vec<(&str, u8)> = Vec::new();
    for (floor, line) in input.lines().enumerate() {
        for captures in microchip_re.captures_iter(line) {
            microchips.push((captures.get(1).unwrap().as_str(), floor.try_into().unwrap()));
        }
        for captures in generator_re.captures_iter(line) {
            generators.push((captures.get(1).unwrap().as_str(), floor.try_into().unwrap()));
        }
    }

    // It doesn't matter if we swap all instances of element A with element B, so we can
    // characterise each element's microchip-generator pair only by their locations. This vastly
    // reduces the search space.
    let mut res = Vec::new();
    for (microchip_element, microchip) in microchips {
        for &(generator_element, generator) in &generators {
            if microchip_element == generator_element {
                res.push(Item {
                    microchip,
                    generator,
                });
            }
        }
    }

    // The order of this representation is also unimportant, so sort it.
    res.sort_unstable();

    res
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct State {
    lift: u8,
    items: Vec<Item>,
    steps: u8,
}

impl State {
    fn is_safe(&self) -> bool {
        for item in &self.items {
            if item.microchip != item.generator {
                // microchip is separate from its generator, so it's unshielded
                for other in &self.items {
                    if item.microchip == other.generator {
                        // there's another generator on the same floor
                        return false;
                    }
                }
            }
        }
        true
    }

    fn items_on_floor(&self) -> Vec<(usize, ItemKind)> {
        let mut res = Vec::new();
        for (i, item) in self.items.iter().enumerate() {
            if item.microchip == self.lift {
                res.push((i, ItemKind::Microchip));
            }
            if item.generator == self.lift {
                res.push((i, ItemKind::Generator));
            }
        }
        res
    }

    fn adjacent_floors(&self) -> Vec<u8> {
        match self.lift {
            0 => vec![1],
            3 => vec![2],
            _ => vec![self.lift - 1, self.lift + 1],
        }
    }

    // The best we can hope for is to move two items up and one down each step. Annoyingly, the
    // heuristic alone gives the correct answer for both parts (but fails on the example).
    fn heuristic(&self) -> u8 {
        let mut res = 0;
        let mut count = 0;
        for floor in 0..=2 {
            for item in &self.items {
                if item.microchip == floor {
                    count += 1;
                }
                if item.generator == floor {
                    count += 1;
                }
            }
            // At this point count covers all items on this floor, plus all items from lower floors
            // that we've brought up with us. Add to res the number of steps to take them all to the
            // next floor.
            match count {
                0 => {}
                1 => res += 1,
                _ => res += 2 * count - 3,
            }
        }
        res
    }

    fn push_adjacent(&self, q: &mut impl Queue<Item = State>) {
        for lift in self.adjacent_floors() {
            let items_on_floor = self.items_on_floor();

            for items in combinatorics::combination(1, &items_on_floor)
                .chain(combinatorics::combination(2, &items_on_floor))
            {
                let mut state = self.clone();

                state.lift = lift;

                for &(i, item_kind) in items {
                    match item_kind {
                        ItemKind::Microchip => state.items[i].microchip = lift,
                        ItemKind::Generator => state.items[i].generator = lift,
                    }
                }

                if state.is_safe() {
                    state.items.sort_unstable();
                    state.steps += 1;
                    q.push(state);
                }
            }
        }
    }
}

fn part_(items: Vec<Item>) -> u8 {
    let mut q = search2::a_star(
        State {
            lift: 0,
            items,
            steps: 0,
        },
        |state| (state.lift, state.items.clone()),
        |state| state.steps,
        State::heuristic,
    );

    while let Some(state) = q.pop() {
        if state.items.iter().all(
            |Item {
                 microchip,
                 generator,
             }| *microchip == 3 && *generator == 3,
        ) {
            return state.steps;
        }

        state.push_adjacent(&mut q);
    }
    unreachable!()
}

pub fn part1(input: &str) -> u8 {
    part_(parse(input))
}

pub fn part2(input: &str) -> u8 {
    let mut items = parse(input);
    items.push(Item {
        microchip: 0,
        generator: 0,
    });
    items.push(Item {
        microchip: 0,
        generator: 0,
    });
    items.sort_unstable();
    part_(items)
}

pub fn tests() {
    let example = "
        The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
        The second floor contains a hydrogen generator.
        The third floor contains a lithium generator.
        The fourth floor contains nothing relevant.
    ";
    assert_eq!(part1(example.trim()), 11);
}
