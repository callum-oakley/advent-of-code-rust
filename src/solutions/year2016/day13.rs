use crate::{grid::Point, search};

fn is_open(seed: u32, Point { x, y }: Point) -> bool {
    (u32::try_from(x * x + 3 * x + 2 * x * y + y + y * y).unwrap() + seed).count_ones() % 2 == 0
}

#[derive(Clone)]
struct State {
    pos: Point,
    steps: u32,
    seed: u32,
}

impl search::State for &State {
    type Adjacent = Vec<State>;
    type HashKey = Point;

    fn adjacent(self) -> Self::Adjacent {
        self.pos
            .adjacent4()
            .into_iter()
            .filter(|pos| pos.x >= 0 && pos.y >= 0 && is_open(self.seed, *pos))
            .map(|pos| State {
                pos,
                steps: self.steps + 1,
                seed: self.seed,
            })
            .collect()
    }

    fn hash_key(self) -> Self::HashKey {
        self.pos
    }
}

fn traversal(input: &str) -> search::BreadthFirstTraversal<State> {
    search::breadth_first(State {
        pos: Point { x: 1, y: 1 },
        steps: 0,
        seed: input.parse().unwrap(),
    })
}

fn part1_(target: Point, input: &str) -> u32 {
    traversal(input)
        .find(|state| state.pos == target)
        .unwrap()
        .steps
}

pub fn part1(input: &str) -> u32 {
    part1_(Point { x: 31, y: 39 }, input)
}

pub fn part2(input: &str) -> usize {
    traversal(input)
        .take_while(|state| state.steps <= 50)
        .count()
}

pub fn tests() {
    assert_eq!(part1_(Point { x: 7, y: 4 }, "10"), 11);
}
