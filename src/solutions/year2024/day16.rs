use std::collections::HashMap;

use crate::{
    grid::{Grid, Turn, Vector, E, LEFT, RIGHT},
    search,
    uniq::Uniq,
};

#[derive(Clone)]
struct State {
    pos: Vector,
    dir: Vector,
    score: usize,
    path: Vec<Vector>,
}

impl State {
    fn step(&mut self) {
        self.pos += self.dir;
        self.score += 1;
        self.path.push(self.pos);
    }

    fn run(&self, maze: &Grid<char>) -> Self {
        let mut state = self.clone();
        state.step();
        while maze[state.pos + state.dir] != '#'
            && maze[state.pos + LEFT * state.dir] == '#'
            && maze[state.pos + RIGHT * state.dir] == '#'
        {
            state.step();
        }
        state
    }

    fn turn(&self, turn: Turn) -> Self {
        let mut state = self.clone();
        state.dir = turn * state.dir;
        state.score += 1000;
        state
    }
}

// We can't use search::dijkstra and hash (pos, dir) because if we can reach the same tile in two
// different ways with the same minimal score we need to consider both. Instead, keep track of the
// lowest score that we see each (pos, dir) and only discard branches with a strictly higher score.
fn search(maze: &Grid<char>) -> impl Iterator<Item = State> + '_ {
    let start = maze.keys().find(|&v| maze[v] == 'S').unwrap();
    let mut first_seen: HashMap<(Vector, Vector), usize> = HashMap::new();
    search::dijkstra_nohash(
        State {
            pos: start,
            dir: E,
            score: 0,
            path: vec![start],
        },
        |state| state.score,
        move |state, push| {
            if maze[state.pos] == 'E'
                || first_seen
                    .get(&(state.pos, state.dir))
                    .is_some_and(|&score| score < state.score)
            {
                return;
            }
            first_seen.insert((state.pos, state.dir), state.score);

            if maze[state.pos + state.dir] != '#' {
                push(state.run(maze));
            }
            for turn in [LEFT, RIGHT] {
                if maze[state.pos + turn * state.dir] != '#' {
                    push(state.turn(turn).run(maze));
                }
            }
        },
    )
}

pub fn part1(input: &str) -> usize {
    let maze = Grid::parse(input, |_, c| c);
    let res = search(&maze)
        .find(|state| maze[state.pos] == 'E')
        .unwrap()
        .score;
    res
}

pub fn part2(input: &str) -> usize {
    let min_score = part1(input);
    let maze = Grid::parse(input, |_, c| c);
    search(&maze)
        .filter(|state| state.score == min_score && maze[state.pos] == 'E')
        .flat_map(|state| state.path)
        .uniq()
        .count()
}

pub fn tests() {
    let example0 = [
        "###############",
        "#.......#....E#",
        "#.#.###.#.###.#",
        "#.....#.#...#.#",
        "#.###.#####.#.#",
        "#.#.#.......#.#",
        "#.#.#####.###.#",
        "#...........#.#",
        "###.#.#####.#.#",
        "#...#.....#.#.#",
        "#.#.#.###.#.#.#",
        "#.....#...#.#.#",
        "#.###.#.#.#.#.#",
        "#S..#.....#...#",
        "###############",
    ]
    .join("\n");
    let example1 = [
        "#################",
        "#...#...#...#..E#",
        "#.#.#.#.#.#.#.#.#",
        "#.#.#.#...#...#.#",
        "#.#.#.#.###.#.#.#",
        "#...#.#.#.....#.#",
        "#.#.#.#.#.#####.#",
        "#.#...#.#.#.....#",
        "#.#.#####.#.###.#",
        "#.#.#.......#...#",
        "#.#.###.#####.###",
        "#.#.#...#.....#.#",
        "#.#.#.#####.###.#",
        "#.#.#.........#.#",
        "#.#.#.#########.#",
        "#S#.............#",
        "#################",
    ]
    .join("\n");
    assert_eq!(part1(&example0), 7036);
    assert_eq!(part1(&example1), 11048);
    assert_eq!(part2(&example0), 45);
    assert_eq!(part2(&example1), 64);
}
