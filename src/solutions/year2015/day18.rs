use crate::{
    grid::{Point, Rect},
    part::Part,
};

fn step(lights: &mut Rect<bool>, scrap: &mut Rect<bool>) {
    for (pos, light) in &*lights {
        let neighbors_on = pos
            .adjacent8()
            .into_iter()
            .filter(|&p| lights.get(p) == Some(&true))
            .count();
        scrap[pos] = neighbors_on == 3 || *light && neighbors_on == 2;
    }
    std::mem::swap(lights, scrap);
}

fn fix_corners(lights: &mut Rect<bool>) {
    for x in [0, lights.size.x - 1] {
        for y in [0, lights.size.y - 1] {
            lights[Point { x, y }] = true;
        }
    }
}

fn part_(part: Part, steps: u32, input: &str) -> usize {
    let mut lights = Rect::parse(input, |_, c| c == '#');
    let mut scrap = Rect::new(false, lights.size);
    if part == Part::Two {
        fix_corners(&mut lights);
    }
    for _ in 0..steps {
        step(&mut lights, &mut scrap);
        if part == Part::Two {
            fix_corners(&mut lights);
        }
    }
    lights.values().filter(|light| **light).count()
}

pub fn part1(input: &str) -> usize {
    part_(Part::One, 100, input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, 100, input)
}

pub fn tests() {
    let example = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
    assert_eq!(part_(Part::One, 4, example), 4);
    assert_eq!(part_(Part::Two, 5, example), 17);
}
