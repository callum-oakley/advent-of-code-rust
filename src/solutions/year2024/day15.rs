use std::collections::HashSet;

use crate::grid::{self, IntoVector, Vector};

struct State {
    walls: HashSet<Vector>,
    boxes: HashSet<Vector>,
    robot: Vector,
}

impl State {
    fn tick(&mut self, dir: Vector) {
        if let Some(space) = self.space(dir) {
            self.robot += dir;
            if self.robot != space {
                // Then we need to shift some boxes.
                self.boxes.remove(&self.robot);
                self.boxes.insert(space);
            }
        }
    }

    fn space(&self, dir: Vector) -> Option<Vector> {
        let mut v = self.robot + dir;
        while self.boxes.contains(&v) {
            v += dir;
        }
        if self.walls.contains(&v) {
            None
        } else {
            Some(v)
        }
    }

    fn score(&self) -> i32 {
        self.boxes.iter().map(|v| v.x + 100 * v.y).sum()
    }
}

fn parse(input: &str) -> (State, impl Iterator<Item = Vector> + '_) {
    let (warehouse, dirs) = input.split_once("\n\n").unwrap();

    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = None;
    grid::scan(warehouse, |v, c| match c {
        '#' => {
            walls.insert(v);
        }
        'O' => {
            boxes.insert(v);
        }
        '@' => {
            robot = Some(v);
        }
        _ => (),
    });

    (
        State {
            walls,
            boxes,
            robot: robot.unwrap(),
        },
        dirs.chars()
            .filter(|&c| "^>v<".contains(c))
            .map(IntoVector::into_vector),
    )
}

pub fn part1(input: &str) -> i32 {
    let (mut state, dirs) = parse(input);

    for dir in dirs {
        state.tick(dir);
    }

    state.score()
}

pub fn tests() {
    let small_example = [
        "########",
        "#..O.O.#",
        "##@.O..#",
        "#...O..#",
        "#.#.O..#",
        "#...O..#",
        "#......#",
        "########",
        "",
        "<^^>>>vv<v>>v<<",
    ]
    .join("\n");
    let big_example = [
        "##########",
        "#..O..O.O#",
        "#......O.#",
        "#.OO..O.O#",
        "#..O@..O.#",
        "#O#..O...#",
        "#O..O..O.#",
        "#.OO.O.OO#",
        "#....O...#",
        "##########",
        "",
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
        "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
        "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
        "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
        "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
        "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
        ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
        "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
        "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
        "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    ]
    .join("\n");
    assert_eq!(part1(&small_example), 2028);
    assert_eq!(part1(&big_example), 10092);
    assert_eq!(part2(&big_example), 9021);
}
