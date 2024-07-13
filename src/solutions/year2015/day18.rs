use crate::{grid::Grid, part::Part};

fn step(lights: &mut Grid<bool>, scrap: &mut Grid<bool>) {
    for (pos, light) in &*lights {
        let neighbors_on = lights.adjacent8_values(pos).filter(|p| **p).count();
        scrap[pos] = neighbors_on == 3 || *light && neighbors_on == 2;
    }
    std::mem::swap(lights, scrap);
}

fn fix_corners(lights: &mut Grid<bool>) {
    for x in [0, lights.size.x - 1] {
        for y in [0, lights.size.y - 1] {
            lights[[x, y]] = true;
        }
    }
}

fn part_(part: Part, steps: u32, input: &str) -> usize {
    let mut lights = Grid::parse(input, |_, c| c == '#');
    let mut scrap = Grid::new(false, lights.size);
    if part == Part::Two {
        fix_corners(&mut lights);
    }
    for _ in 0..steps {
        step(&mut lights, &mut scrap);
        if part == Part::Two {
            fix_corners(&mut lights);
        }
    }
    lights.into_values().filter(|light| *light).count()
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
