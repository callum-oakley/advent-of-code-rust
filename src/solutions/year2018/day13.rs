use crate::{
    grid::{Point, Rect, Turn},
    part::Part,
};

enum Tile {
    Empty,
    Straight,
    TurnFSlash,
    TurnBSlash,
    Intersection,
}

struct Cart {
    pos: Point,
    dir: Point,
    turn_count: usize,
}

impl Cart {
    fn tick(&mut self, track: &Rect<Tile>) {
        self.pos += self.dir;
        match track[self.pos] {
            Tile::Empty => unreachable!(),
            Tile::Straight => {}
            Tile::TurnFSlash => {
                if self.dir.x == 0 {
                    self.dir = self.dir.turn(Turn::Right);
                } else {
                    self.dir = self.dir.turn(Turn::Left);
                }
            }
            Tile::TurnBSlash => {
                if self.dir.x == 0 {
                    self.dir = self.dir.turn(Turn::Left);
                } else {
                    self.dir = self.dir.turn(Turn::Right);
                }
            }
            Tile::Intersection => {
                match self.turn_count % 3 {
                    0 => {
                        self.dir = self.dir.turn(Turn::Left);
                    }
                    2 => {
                        self.dir = self.dir.turn(Turn::Right);
                    }
                    _ => {}
                }
                self.turn_count += 1;
            }
        }
    }
}

fn parse(input: &str) -> (Rect<Tile>, Vec<Cart>) {
    let mut carts = Vec::new();
    let track = Rect::parse(input, |pos, c| match c {
        ' ' => Tile::Empty,
        '/' => Tile::TurnFSlash,
        '\\' => Tile::TurnBSlash,
        '+' => Tile::Intersection,
        '^' | 'v' | '<' | '>' => {
            carts.push(Cart {
                pos,
                dir: c.into(),
                turn_count: 0,
            });
            Tile::Straight
        }
        _ => Tile::Straight,
    });
    (track, carts)
}

fn tick(part: Part, track: &Rect<Tile>, carts: &mut Vec<Cart>) -> Option<Point> {
    carts.sort_unstable_by_key(|c| c.pos);

    let mut i = 0;
    while i < carts.len() {
        let pos = carts[i].pos + carts[i].dir;
        if let Some(j) = carts.iter().position(|c| c.pos == pos) {
            match part {
                Part::One => {
                    return Some(pos);
                }
                Part::Two => {
                    carts.remove(j);
                    if j < i {
                        i -= 1;
                    }
                    carts.remove(i);
                    continue;
                }
            }
        }

        carts[i].tick(track);
        i += 1;
    }

    None
}

pub fn part1(input: &str) -> String {
    let (track, mut carts) = parse(input);
    loop {
        if let Some(crash) = tick(Part::One, &track, &mut carts) {
            return crash.to_string();
        }
    }
}

pub fn part2(input: &str) -> String {
    let (track, mut carts) = parse(input);
    while carts.len() >= 2 {
        tick(Part::Two, &track, &mut carts);
    }
    carts[0].pos.to_string()
}

pub fn tests() {
    assert_eq!(
        part1(
            &[
                r"/->-\        ",
                r"|   |  /----\",
                r"| /-+--+-\  |",
                r"| | |  | v  |",
                r"\-+-/  \-+--/",
                r"  \------/   ",
            ]
            .join("\n")
        ),
        "7,3",
    );
    assert_eq!(
        part2("/>-<\\  \n|   |  \n| /<+-\\\n| | | v\n\\>+</ |\n  |   ^\n  \\<->/"),
        "6,4",
    );
}
