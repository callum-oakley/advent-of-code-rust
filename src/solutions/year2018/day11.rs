use crate::{
    grid::{Point, Rect, NW},
    part::Part,
};

fn cell(serial: i32, pos: Point) -> i32 {
    ((pos.x + 10) * pos.y + serial) * (pos.x + 10) / 100 % 10 - 5
}

// https://en.wikipedia.org/wiki/Summed-area_table
fn summed_area_table(serial: i32) -> Rect<i32> {
    let mut res = Rect::new(0, Point { x: 301, y: 301 });
    for x in 1..=300 {
        for y in 1..=300 {
            res[Point { x, y }] = cell(serial, Point { x, y })
                + res[Point { x: x - 1, y }]
                + res[Point { x, y: y - 1 }]
                - res[Point { x: x - 1, y: y - 1 }];
        }
    }
    res
}

fn square(sat: &Rect<i32>, pos: Point, size: i32) -> i32 {
    let base = pos + NW;
    sat[base] + sat[base + Point { x: size, y: size }]
        - sat[base + Point { x: size, y: 0 }]
        - sat[base + Point { x: 0, y: size }]
}

fn part_(part: Part, input: &str) -> (Point, i32) {
    let sat = summed_area_table(input.parse().unwrap());
    (match part {
        Part::One => 3..=3,
        Part::Two => 1..=300,
    })
    .flat_map(|size| {
        (1..=301 - size).flat_map(move |x| (1..=301 - size).map(move |y| (Point { x, y }, size)))
    })
    .max_by_key(|&(pos, size)| square(&sat, pos, size))
    .unwrap()
}

pub fn part1(input: &str) -> String {
    let (Point { x, y }, _) = part_(Part::One, input);
    format!("{x},{y}")
}

pub fn part2(input: &str) -> String {
    let (Point { x, y }, size) = part_(Part::Two, input);
    format!("{x},{y},{size}")
}

pub fn tests() {
    assert_eq!(cell(8, Point { x: 3, y: 5 }), 4);
    assert_eq!(cell(57, Point { x: 122, y: 79 }), -5);
    assert_eq!(cell(39, Point { x: 217, y: 196 }), 0);
    assert_eq!(cell(71, Point { x: 101, y: 153 }), 4);

    assert_eq!(part1("18"), "33,45");
    assert_eq!(part1("42"), "21,61");

    assert_eq!(part2("18"), "90,269,16");
    assert_eq!(part2("42"), "232,251,12");
}
