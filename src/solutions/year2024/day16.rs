use crate::{
    grid::{Grid, Vector, E, LEFT, RIGHT},
    search,
};

fn search(maze: &Grid<char>) -> usize {
    struct State {
        pos: Vector,
        dir: Vector,
        score: usize,
    }
    search::dijkstra(
        State {
            pos: maze.keys().find(|&v| maze[v] == 'S').unwrap(),
            dir: E,
            score: 0,
        },
        |state| (state.pos, state.dir),
        |state| state.score,
        |state, push| {
            if maze[state.pos + state.dir] != '#' {
                push(State {
                    pos: state.pos + state.dir,
                    dir: state.dir,
                    score: state.score + 1,
                });
            }
            for turn in [LEFT, RIGHT] {
                push(State {
                    pos: state.pos,
                    dir: turn * state.dir,
                    score: state.score + 1000,
                });
            }
        },
    )
    .find(|state| maze[state.pos] == 'E')
    .unwrap()
    .score
}

pub fn part1(input: &str) -> usize {
    search(&Grid::parse(input, |_, c| c))
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
}
