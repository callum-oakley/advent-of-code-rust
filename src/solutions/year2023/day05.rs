struct Interval {
    start: i64,
    len: i64,
}

impl Interval {
    fn end(&self) -> i64 {
        self.start + self.len - 1
    }
}

struct Map {
    src: Interval,
    dst: Interval,
}

fn parse(input: &str) -> (Vec<i64>, impl Iterator<Item = Vec<Map>> + '_) {
    fn parse_nums(s: &str) -> impl Iterator<Item = i64> + '_ {
        s.split_whitespace().map(|n| n.parse().unwrap())
    }
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    (
        parse_nums(seeds.trim().strip_prefix("seeds: ").unwrap()).collect::<Vec<_>>(),
        maps.split("\n\n").map(|block| {
            block
                .trim()
                .lines()
                .skip(1)
                .map(|line| {
                    let mut nums = parse_nums(line);
                    let dst_start = nums.next().unwrap();
                    let src_start = nums.next().unwrap();
                    let len = nums.next().unwrap();
                    Map {
                        src: Interval {
                            start: src_start,
                            len,
                        },
                        dst: Interval {
                            start: dst_start,
                            len,
                        },
                    }
                })
                .collect()
        }),
    )
}

pub fn part1(input: &str) -> i64 {
    let (seeds, stages) = parse(input);
    stages
        .fold(seeds, |seeds, maps| {
            seeds
                .into_iter()
                .map(|seed| {
                    for map in &maps {
                        if map.src.start <= seed && seed <= map.src.end() {
                            return seed - map.src.start + map.dst.start;
                        }
                    }
                    seed
                })
                .collect()
        })
        .into_iter()
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let (seeds, stages) = parse(input);
    stages
        .fold(
            seeds
                .chunks(2)
                .map(|pair| Interval {
                    start: pair[0],
                    len: pair[1],
                })
                .collect::<Vec<_>>(),
            |seeds, maps| {
                seeds
                    .into_iter()
                    .flat_map(|seed| {
                        // Assume a seed interval is affected by at most one map interval
                        for map in &maps {
                            if seed.end() < map.src.start || seed.start > map.src.end() {
                                // seed and map are disjoint
                            } else if seed.start >= map.src.start && seed.end() <= map.src.end() {
                                return vec![Interval {
                                    start: seed.start - map.src.start + map.dst.start,
                                    len: seed.len,
                                }];
                            } else if seed.start < map.src.start && seed.end() <= map.src.end() {
                                return vec![
                                    Interval {
                                        start: seed.start,
                                        len: map.src.start - seed.start,
                                    },
                                    Interval {
                                        start: map.dst.start,
                                        len: seed.len - map.src.start + seed.start,
                                    },
                                ];
                            } else if seed.start >= map.src.start && seed.end() > map.src.end() {
                                return vec![
                                    Interval {
                                        start: seed.start - map.src.start + map.dst.start,
                                        len: map.src.end() - seed.start,
                                    },
                                    Interval {
                                        start: map.src.start + map.src.len,
                                        len: seed.len - map.src.end() + seed.start,
                                    },
                                ];
                            } else if seed.start < map.src.start && seed.end() > map.src.end() {
                                return vec![
                                    Interval {
                                        start: seed.start,
                                        len: map.src.start - seed.start,
                                    },
                                    Interval {
                                        start: map.dst.start,
                                        len: map.src.len,
                                    },
                                    Interval {
                                        start: map.src.start + map.src.len,
                                        len: seed.end() - map.src.end(),
                                    },
                                ];
                            }
                        }
                        vec![seed]
                    })
                    .collect()
            },
        )
        .into_iter()
        .map(|seed| seed.start)
        .min()
        .unwrap()
}

pub fn tests() {
    let example = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    ";

    assert_eq!(part1(example), 35);
    assert_eq!(part2(example), 46);
}
