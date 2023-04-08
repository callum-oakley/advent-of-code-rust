use lazy_static::lazy_static;
use regex::Regex;

use crate::grid::Point;

enum Op {
    On,
    Off,
    Toggle,
}

struct Instruction {
    op: Op,
    from: Point,
    to: Point,
}

fn parse(s: &str) -> impl Iterator<Item = Instruction> + '_ {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(turn on|turn off|toggle) (\d+,\d+) through (\d+,\d+)").unwrap();
    }
    RE.captures_iter(s).map(|capture| Instruction {
        op: match &capture[1] {
            "turn on" => Op::On,
            "turn off" => Op::Off,
            "toggle" => Op::Toggle,
            _ => unreachable!(),
        },
        from: capture[2].into(),
        to: capture[3].into(),
    })
}

fn part_<F: Fn(&Op, &mut u8)>(update: F, input: &str) -> u32 {
    let mut lights = [[0u8; 1000]; 1000];
    for instruction in parse(input) {
        for y in instruction.from.y..=instruction.to.y {
            for x in instruction.from.x..=instruction.to.x {
                update(&instruction.op, &mut lights[y as usize][x as usize]);
            }
        }
    }
    lights
        .into_iter()
        .map(|row| row.into_iter().map(|light| light as u32).sum::<u32>())
        .sum()
}

pub fn part1(input: &str) -> u32 {
    part_(
        |op: &Op, light: &mut u8| match op {
            Op::On => *light = 1,
            Op::Off => *light = 0,
            Op::Toggle => {
                *light = match light {
                    0 => 1,
                    1 => 0,
                    _ => unreachable!(),
                }
            }
        },
        input,
    )
}

pub fn part2(input: &str) -> u32 {
    part_(
        |op: &Op, light: &mut u8| match op {
            Op::On => *light += 1,
            Op::Off => *light = light.saturating_sub(1),
            Op::Toggle => *light += 2,
        },
        input,
    )
}
