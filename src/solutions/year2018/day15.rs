use crate::{
    grid::{Point, Rect},
    part::Part,
    search,
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

fn parse(elf_ap: i32, input: &str) -> Rect<Square> {
    Rect::parse(input, |_, c| match c {
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

#[derive(Clone)]
struct State<'a> {
    dist: u32,
    pos: Point,
    first_step: Option<Point>,
    cave: &'a Rect<Square>,
}

impl<'a> search::State for State<'a> {
    type HashKey = Point;

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(self.pos.adjacent4().into_iter().filter_map(|step| {
            if let Some(Square::Empty) = self.cave.get(step) {
                Some(Self {
                    dist: self.dist + 1,
                    pos: step,
                    first_step: self.first_step.or(Some(step)),
                    ..*self
                })
            } else {
                None
            }
        }))
    }

    fn hash_key(&self) -> Self::HashKey {
        self.pos
    }
}

// The cost function is fiddly, here's everything it needs to cover:
// - To move, the unit first considers the squares that are in range and determines which of those
//   squares it could reach in the fewest steps.
// - If multiple squares are in range and tied for being reachable in the fewest steps, the square
//   which is first in reading order is chosen.
// - If multiple steps would put the unit equally closer to its destination, the unit chooses the
//   step which is first in reading order.
// Missing either of the last two steps produces correct results for all the examples, but fails on
// the puzzle proper...
impl<'a> search::OrdKey for State<'a> {
    type OrdKey = (u32, Point, Option<Point>);

    fn ord_key(&self) -> Self::OrdKey {
        (self.dist, self.pos, self.first_step)
    }
}

fn in_range(cave: &Rect<Square>, pos: Point, target_kind: Kind) -> Option<Point> {
    pos.adjacent4()
        .into_iter()
        .filter(|&pos| {
            cave.get(pos).map_or(false, Square::is_occupied) && cave[pos].unit().kind == target_kind
        })
        .min_by_key(|pos| cave[pos].unit().hp)
}

fn turn(part: Part, cave: &mut Rect<Square>, mut pos: Point, target_kind: Kind) -> Result<()> {
    let Some(state) = search::min_first(State {
        dist: 0,
        pos,
        first_step: None,
        cave,
    })
    .find(|state| in_range(cave, state.pos, target_kind).is_some()) else {
        return Ok(());
    };

    let unit = *cave[pos].unit();

    if let Some(step) = state.first_step {
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
