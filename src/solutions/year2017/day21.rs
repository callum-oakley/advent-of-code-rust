use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pattern {
    size: usize,
    data: Vec<bool>,
}

impl Pattern {
    fn rotate(&self) -> Pattern {
        let mut res = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                res.data[x + y * self.size] = self.data[self.size - 1 - y + x * self.size];
            }
        }
        res
    }

    fn reflect(&self) -> Pattern {
        let mut res = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                res.data[x + y * self.size] = self.data[self.size - 1 - x + y * self.size];
            }
        }
        res
    }

    fn symmetries(&self) -> [Pattern; 8] {
        [
            self.clone(),
            self.rotate(),
            self.rotate().rotate(),
            self.rotate().rotate().rotate(),
            self.reflect(),
            self.reflect().rotate(),
            self.reflect().rotate().rotate(),
            self.reflect().rotate().rotate().rotate(),
        ]
    }

    fn blocks(&self) -> Blocks {
        let (inner_size, outer_size) = if self.size % 2 == 0 {
            (2, self.size / 2)
        } else {
            (3, self.size / 3)
        };

        let mut data = vec![
            Pattern {
                size: inner_size,
                data: vec![false; inner_size * inner_size],
            };
            outer_size * outer_size
        ];

        for y in 0..self.size {
            let outer_y = y / inner_size;
            let inner_y = y % inner_size;
            for x in 0..self.size {
                let outer_x = x / inner_size;
                let inner_x = x % inner_size;
                data[outer_x + outer_y * outer_size].data[inner_x + inner_y * inner_size] =
                    self.data[x + y * self.size];
            }
        }

        Blocks {
            outer_size,
            inner_size,
            data,
        }
    }
}

impl From<&str> for Pattern {
    fn from(s: &str) -> Pattern {
        let size = match s.len() {
            5 => 2,
            11 => 3,
            19 => 4,
            _ => unreachable!(),
        };

        let mut data = vec![false; size * size];

        let mut x = 0;
        let mut y = 0;
        for c in s.chars() {
            match c {
                '#' => {
                    data[x + y * size] = true;
                }
                '/' => {
                    x = 0;
                    y += 1;
                    continue;
                }
                _ => (),
            }
            x += 1;
        }

        Pattern { size, data }
    }
}

struct Blocks {
    outer_size: usize,
    inner_size: usize,
    data: Vec<Pattern>,
}

impl Blocks {
    fn enhance(&mut self, rules: &HashMap<Pattern, Pattern>) {
        for block in &mut self.data {
            *block = rules[block].clone();
        }
        self.inner_size += 1;
    }

    fn collapse(&self) -> Pattern {
        let size = self.inner_size * self.outer_size;
        let mut data = vec![false; size * size];

        for y in 0..size {
            let outer_y = y / self.inner_size;
            let inner_y = y % self.inner_size;
            for x in 0..size {
                let outer_x = x / self.inner_size;
                let inner_x = x % self.inner_size;
                data[x + y * size] = self.data[outer_x + outer_y * self.outer_size].data
                    [inner_x + inner_y * self.inner_size];
            }
        }

        Pattern { size, data }
    }
}

fn parse(input: &str) -> HashMap<Pattern, Pattern> {
    let mut res = HashMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once(" => ").unwrap();
        let from = Pattern::from(from);
        let to = Pattern::from(to);
        for from in from.symmetries() {
            res.insert(from, to.clone());
        }
    }
    res
}

fn part_(iterations: usize, input: &str) -> usize {
    let rules = parse(input);
    let mut pattern = Pattern::from(".#./..#/###");

    for _ in 0..iterations {
        let mut blocks = pattern.blocks();
        blocks.enhance(&rules);
        pattern = blocks.collapse();
    }

    pattern.data.into_iter().filter(|p| *p).count()
}

pub fn part1(input: &str) -> usize {
    part_(5, input)
}

pub fn part2(input: &str) -> usize {
    part_(18, input)
}

pub fn tests() {
    let example = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";
    assert_eq!(part_(2, example), 12);
}
