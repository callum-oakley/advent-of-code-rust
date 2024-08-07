use crate::{
    grid::{self, Grid, Vector},
    part::Part,
    search::{self, Queue},
};

#[derive(Debug)]
struct ElfDeath;
type Result<T> = std::result::Result<T, ElfDeath>;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Elf,
    Goblin,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Unit {
    kind: Kind,
    hp: i32,
    ap: i32,
    moved: bool,
}

impl Unit {
    fn enemy(self) -> Kind {
        match self.kind {
            Kind::Elf => Kind::Goblin,
            Kind::Goblin => Kind::Elf,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Square {
    Wall,
    Empty,
    Occupied(Unit),
}

impl Square {
    fn is_occupied(&self) -> bool {
        matches!(self, Square::Occupied(_))
    }

    fn unit(&self) -> &Unit {
        if let Square::Occupied(unit) = self {
            unit
        } else {
            panic!("unoccupied")
        }
    }

    fn unit_mut(&mut self) -> &mut Unit {
        if let Square::Occupied(unit) = self {
            unit
        } else {
            panic!("unoccupied")
        }
    }
}

fn parse(elf_ap: i32, input: &str) -> Grid<Square> {
    Grid::parse(input, |_, c| match c {
        '#' => Square::Wall,
        '.' => Square::Empty,
        'E' => Square::Occupied(Unit {
            kind: Kind::Elf,
            hp: 200,
            ap: elf_ap,
            moved: false,
        }),
        'G' => Square::Occupied(Unit {
            kind: Kind::Goblin,
            hp: 200,
            ap: 3,
            moved: false,
        }),
        _ => unreachable!(),
    })
}

fn in_range(cave: &Grid<Square>, pos: Vector, target_kind: Kind) -> Option<Vector> {
    cave.adjacent4(pos)
        .filter(|(_, square)| square.is_occupied() && square.unit().kind == target_kind)
        .min_by_key(|(_, square)| square.unit().hp)
        .map(|(pos, _)| pos)
}

fn first_step(cave: &mut Grid<Square>, pos: Vector, target_kind: Kind) -> Option<Vector> {
    struct State {
        dist: u32,
        pos: Vector,
        first_step: Option<Vector>,
    }

    let mut q = search::dijkstra(
        State {
            dist: 0,
            pos,
            first_step: None,
        },
        |state| state.pos,
        // The cost function is fiddly, here's everything it needs to cover:
        // - To move, the unit first considers the squares that are in range and determines which of
        //   those squares it could reach in the fewest steps.
        // - If multiple squares are in range and tied for being reachable in the fewest steps, the
        //   square which is first in reading order is chosen.
        // - If multiple steps would put the unit equally closer to its destination, the unit
        //   chooses the step which is first in reading order.
        // Missing either of the last two steps produces correct results for all the examples, but
        // fails on the puzzle proper...
        |state| {
            (
                state.dist,
                grid::reading_ord_key(state.pos),
                state.first_step.map(grid::reading_ord_key),
            )
        },
    );

    while let Some(state) = q.pop() {
        if in_range(cave, state.pos, target_kind).is_some() {
            return state.first_step;
        }

        for (pos, square) in cave.adjacent4(state.pos) {
            if let Square::Empty = square {
                q.push(State {
                    dist: state.dist + 1,
                    pos,
                    first_step: state.first_step.or(Some(pos)),
                });
            }
        }
    }

    None
}

fn turn(part: Part, cave: &mut Grid<Square>, mut pos: Vector, target_kind: Kind) -> Result<()> {
    let unit = *cave[pos].unit();

    if let Some(step) = first_step(cave, pos, target_kind) {
        cave[pos] = Square::Empty;
        pos = step;
        assert_eq!(cave[pos], Square::Empty);
        cave[pos] = Square::Occupied(unit);
    }

    if let Some(target_pos) = in_range(cave, pos, target_kind) {
        let target = cave[target_pos].unit_mut();
        target.hp -= unit.ap;
        if target.hp <= 0 {
            if part == Part::Two && target.kind == Kind::Elf {
                return Err(ElfDeath);
            }
            cave[target_pos] = Square::Empty;
        }
    }

    Ok(())
}

fn part_(part: Part, elf_ap: i32, input: &str) -> Result<i32> {
    let mut cave = parse(elf_ap, input);
    for round in 0.. {
        for pos in cave.keys() {
            if let Square::Occupied(unit) = &mut cave[pos] {
                unit.moved = false;
            }
        }

        for pos in cave.keys() {
            if let Square::Occupied(unit) = &mut cave[pos] {
                // Mark units as moved to make sure a unit moving later in the reading order doesn't
                // get to go twice.
                if unit.moved {
                    continue;
                }
                unit.moved = true;

                let target_kind = unit.enemy();
                if cave.values().any(|s| {
                    if let Square::Occupied(unit) = s {
                        unit.kind == target_kind
                    } else {
                        false
                    }
                }) {
                    turn(part, &mut cave, pos, target_kind)?;
                } else {
                    return Ok(round
                        * cave
                            .values()
                            .filter_map(|s| {
                                if let Square::Occupied(unit) = s {
                                    Some(unit.hp)
                                } else {
                                    None
                                }
                            })
                            .sum::<i32>());
                }
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> i32 {
    part_(Part::One, 3, input).unwrap()
}

pub fn part2(input: &str) -> i32 {
    for elf_ap in 4.. {
        if let Ok(res) = part_(Part::Two, elf_ap, input) {
            return res;
        }
    }
    unreachable!()
}

pub fn tests() {
    assert_eq!(part1(".G...\n...EG\n.#.#G\n..G#E\n....."), 27730);
    assert_eq!(part1("G..#E\nE#E.E\nG.##.\n...#E\n...E."), 36334);
    assert_eq!(part1("E..EG\n.#G.E\nE.##E\nG..#.\n..E#."), 39514);
    assert_eq!(part1("E.G#.\n.#G..\nG.#.G\nG..#.\n...E."), 27755);
    assert_eq!(part1(".E...\n.#..G\n.###.\nE#G#G\n...#G"), 28944);
    assert_eq!(
        part1("G......\n.E.#...\n..##..G\n...##..\n...#...\n.G...G.\n.....G."),
        18740,
    );

    assert_eq!(part2(".G...\n...EG\n.#.#G\n..G#E\n....."), 4988);
    assert_eq!(part2("E..EG\n.#G.E\nE.##E\nG..#.\n..E#."), 31284);
    assert_eq!(part2("E.G#.\n.#G..\nG.#.G\nG..#.\n...E."), 3478);
    assert_eq!(part2(".E...\n.#..G\n.###.\nE#G#G\n...#G"), 6474);
    assert_eq!(
        part2("G......\n.E.#...\n..##..G\n...##..\n...#...\n.G...G.\n.....G."),
        1140,
    );
}
