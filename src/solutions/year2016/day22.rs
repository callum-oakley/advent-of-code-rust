use std::collections::HashSet;

use regex::Regex;

use crate::{
    grid::{self, Grid, Vector, Z},
    search::{self, Queue},
};

#[derive(Copy, Clone)]
struct Node {
    size: u32,
    used: u32,
}

fn parse(size: Vector, input: &str) -> Grid<Node> {
    let mut grid = Grid::new(Node { size: 0, used: 0 }, size);
    let re = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T").unwrap();
    for captures in re.captures_iter(input) {
        grid[[captures[1].parse().unwrap(), captures[2].parse().unwrap()]] = Node {
            size: captures[3].parse().unwrap(),
            used: captures[4].parse().unwrap(),
        };
    }
    grid
}

pub fn part1(input: &str) -> usize {
    let grid = parse(Vector::new(30, 32), input);
    let mut res = 0;
    for (i, a) in &grid {
        for (j, b) in &grid {
            if i != j && a.used > 0 && a.used + b.used <= b.size {
                res += 1;
            }
        }
    }
    res
}

#[derive(Clone)]
struct State {
    hole: Vector,
    goal: Vector,
    steps: usize,
}

fn part2_(size: Vector, input: &str) -> usize {
    let grid = parse(size, input);

    let mut viable = HashSet::new();
    let mut hole = None;

    for (i, a) in &grid {
        if a.used == 0 {
            hole = Some(i);
        }

        for (j, b) in &grid {
            if i != j && a.used > 0 && a.used + b.used <= b.size {
                viable.insert(i);
                viable.insert(j);
            }
        }
    }

    let mut q = search::a_star(
        State {
            hole: hole.unwrap(),
            goal: Vector::new(size.x - 1, 0),
            steps: 0,
        },
        |state| (state.hole, state.goal),
        |state| state.steps,
        // First we have to move the hole next to the goal. Then it takes 5 steps to shuffle the
        // goal along one space.
        |state| {
            usize::try_from((state.goal - state.hole).abs().sum()).unwrap()
                + 5 * usize::try_from(state.goal.abs().sum()).unwrap()
        },
    );

    while let Some(state) = q.pop() {
        if state.goal == Z {
            return state.steps;
        }
        for pos in grid::adjacent4(state.hole) {
            if viable.contains(&pos) {
                let mut state = state.clone();
                if state.goal == pos {
                    state.goal = state.hole;
                }
                state.hole = pos;
                state.steps += 1;
                q.push(state);
            }
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> usize {
    part2_(Vector::new(30, 32), input)
}

pub fn tests() {
    let example = "Filesystem            Size  Used  Avail  Use%
                   /dev/grid/node-x0-y0   10T    8T     2T   80%
                   /dev/grid/node-x0-y1   11T    6T     5T   54%
                   /dev/grid/node-x0-y2   32T   28T     4T   87%
                   /dev/grid/node-x1-y0    9T    7T     2T   77%
                   /dev/grid/node-x1-y1    8T    0T     8T    0%
                   /dev/grid/node-x1-y2   11T    7T     4T   63%
                   /dev/grid/node-x2-y0   10T    6T     4T   60%
                   /dev/grid/node-x2-y1    9T    8T     1T   88%
                   /dev/grid/node-x2-y2    9T    6T     3T   66%";
    assert_eq!(part2_(Vector::new(3, 3), example), 7);
}
