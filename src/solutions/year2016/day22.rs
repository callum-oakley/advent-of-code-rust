use std::collections::HashSet;

use regex::Regex;

use crate::{
    grid::{Point, Rect, Z},
    search,
};

#[derive(Copy, Clone)]
struct Node {
    size: u32,
    used: u32,
}

fn parse(size: Point, input: &str) -> Rect<Node> {
    let mut grid = Rect::new(Node { size: 0, used: 0 }, size);
    let re = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T").unwrap();
    for captures in re.captures_iter(input) {
        grid[Point {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
        }] = Node {
            size: captures[3].parse().unwrap(),
            used: captures[4].parse().unwrap(),
        };
    }
    grid
}

pub fn part1(input: &str) -> usize {
    let grid = parse(Point { x: 30, y: 32 }, input);
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
struct State<'a> {
    viable: &'a HashSet<Point>,
    hole: Point,
    goal: Point,
    steps: usize,
}

impl<'a, 'b> search::State for &'b State<'a> {
    type Adjacent = Vec<State<'a>>;

    type HashKey = (Point, Point);

    fn adjacent(self) -> Self::Adjacent {
        self.hole
            .adjacent4()
            .into_iter()
            .filter(|p| self.viable.contains(p))
            .map(|p| {
                let mut state = self.clone();
                if state.goal == p {
                    state.goal = state.hole;
                }
                state.hole = p;
                state.steps += 1;
                state
            })
            .collect()
    }

    fn hash_key(self) -> Self::HashKey {
        (self.hole, self.goal)
    }
}

impl<'a, 'b> search::OrdKey for &'b State<'a> {
    type OrdKey = usize;

    fn ord_key(self) -> Self::OrdKey {
        self.steps
            // First we have to move the hole next to the goal.
            + usize::try_from((self.goal - self.hole).manhattan()).unwrap()
            // Then it takes 5 steps to shuffle the goal along one space.
            + 5 * usize::try_from(self.goal.manhattan()).unwrap()
    }
}

fn part2_(size: Point, input: &str) -> usize {
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

    search::min_first(State {
        viable: &viable,
        hole: hole.unwrap(),
        goal: Point {
            x: size.x - 1,
            y: 0,
        },
        steps: 0,
    })
    .find(|state| state.goal == Z)
    .unwrap()
    .steps
}

pub fn part2(input: &str) -> usize {
    part2_(Point { x: 30, y: 32 }, input)
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
    assert_eq!(part2_(Point { x: 3, y: 3 }, example), 7);
}
