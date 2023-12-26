use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    grid::{Point, Rect},
    search::{self, Queue},
};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Point),
}

fn parse(input: &str) -> Rect<Tile> {
    Rect::parse(input, |_, c| match c {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '^' | '<' | '>' | 'v' => Tile::Slope(c.into()),
        _ => unreachable!(),
    })
}

fn reachable(map: &Rect<Tile>, nodes: &HashSet<Point>, start: Point) -> HashMap<Point, usize> {
    struct State {
        pos: Point,
        steps: usize,
    }
    let mut res = HashMap::new();
    let mut q = search::breadth_first(
        State {
            pos: start,
            steps: 0,
        },
        |state| state.pos,
    );
    while let Some(state) = q.pop() {
        if nodes.contains(&state.pos) && state.pos != start {
            res.insert(state.pos, state.steps);
        } else if let Tile::Slope(dir) = map[state.pos] {
            q.push(State {
                pos: state.pos + dir,
                steps: state.steps + 1,
            });
        } else {
            for pos in state.pos.adjacent4() {
                if map.get(pos).is_some_and(|&tile| tile != Tile::Forest) {
                    q.push(State {
                        pos,
                        steps: state.steps + 1,
                    });
                }
            }
        }
    }
    res
}

fn graph(map: &Rect<Tile>, start: Point, end: Point) -> HashMap<Point, HashMap<Point, usize>> {
    let mut nodes = HashSet::from([start, end]);
    nodes.extend(map.keys().filter(|pos| {
        pos.adjacent4()
            .into_iter()
            .filter(|&p| map.get(p).is_some_and(|&tile| tile != Tile::Forest))
            .count()
            >= 3
    }));
    nodes
        .iter()
        .map(|&node| (node, reachable(map, &nodes, node)))
        .collect()
}

fn steps(graph: &HashMap<Point, HashMap<Point, usize>>, path: &[Point]) -> usize {
    path.windows(2).map(|hop| graph[&hop[0]][&hop[1]]).sum()
}

fn part_(map: &Rect<Tile>) -> usize {
    let start = Point::new(0, 1);
    let end = map.size - Point::new(1, 2);
    let graph = graph(map, start, end);
    let mut q = VecDeque::from([vec![start]]);
    let mut max_steps = 0;
    while let Some(path) = q.pop() {
        let pos = *path.last().unwrap();
        if pos == end {
            max_steps = max_steps.max(steps(&graph, &path));
        } else {
            for &p in graph[&pos].keys() {
                if !path.contains(&p) {
                    let mut path = path.clone();
                    path.push(p);
                    q.push(path);
                }
            }
        }
    }
    max_steps
}

pub fn part1(input: &str) -> usize {
    part_(&parse(input))
}

pub fn part2(input: &str) -> usize {
    let mut map = parse(input);
    for pos in map.keys() {
        if let Tile::Slope(_) = map[pos] {
            map[pos] = Tile::Path;
        }
    }
    part_(&map)
}

pub fn tests() {
    let example = [
        "#.#####################",
        "#.......#########...###",
        "#######.#########.#.###",
        "###.....#.>.>.###.#.###",
        "###v#####.#v#.###.#.###",
        "###.>...#.#.#.....#...#",
        "###v###.#.#.#########.#",
        "###...#.#.#.......#...#",
        "#####.#.#.#######.#.###",
        "#.....#.#.#.......#...#",
        "#.#####.#.#.#########v#",
        "#.#...#...#...###...>.#",
        "#.#.#v#######v###.###v#",
        "#...#.>.#...>.>.#.###.#",
        "#####v#.#.###v#.#.###.#",
        "#.....#...#...#.#.#...#",
        "#.#########.###.#.#.###",
        "#...###...#...#...#.###",
        "###.###.#.###v#####v###",
        "#...#...#.#.>.>.#.>.###",
        "#.###.###.#.###.#.#v###",
        "#.....###...###...#...#",
        "#####################.#",
    ]
    .join("\n");
    assert_eq!(part1(&example), 94);
    assert_eq!(part2(&example), 154);
}
