use crate::number_theory::{self, Congruence};

struct Bus {
    id: i64,
    index: i64,
}

fn parse(input: &str) -> (i64, Vec<Bus>) {
    let (earliest_departure, busses) = input.split_once('\n').unwrap();
    (
        earliest_departure.parse().unwrap(),
        busses
            .split(',')
            .enumerate()
            .filter_map(|(index, id)| {
                id.parse().map_or(None, |id| {
                    Some(Bus {
                        id,
                        index: i64::try_from(index).unwrap(),
                    })
                })
            })
            .collect(),
    )
}

pub fn part1(input: &str) -> i64 {
    let (earliest_departure, busses) = parse(input);
    let wait = |bus: &Bus| (-earliest_departure).rem_euclid(bus.id);
    let bus = busses.into_iter().min_by_key(wait).unwrap();
    bus.id * wait(&bus)
}

pub fn part2(input: &str) -> i64 {
    number_theory::chinese_remainder(
        parse(input)
            .1
            .into_iter()
            .map(|bus| Congruence {
                a: -bus.index,
                n: bus.id,
            })
            .collect(),
    )
}

pub fn tests() {
    let example = "939\n7,13,x,x,59,x,31,19";
    assert_eq!(part1(example), 295);
    assert_eq!(part2(example), 1_068_781);
}
