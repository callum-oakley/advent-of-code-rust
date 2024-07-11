use lazy_static::lazy_static;
use nalgebra::Vector2;
use regex::Regex;

use crate::grid2::IntoVector;

enum Op {
    On,
    Off,
    Toggle,
}

struct Instruction {
    op: Op,
    from: Vector2<i32>,
    to: Vector2<i32>,
}

fn parse(s: &str) -> impl Iterator<Item = Instruction> + '_ {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(turn on|turn off|toggle) (\d+,\d+) through (\d+,\d+)").unwrap();
    }
    RE.captures_iter(s).map(|captures| Instruction {
        op: match &captures[1] {
            "turn on" => Op::On,
            "turn off" => Op::Off,
            "toggle" => Op::Toggle,
            _ => unreachable!(),
        },
        from: captures[2].into_vector(),
        to: captures[3].into_vector(),
    })
}

fn part_<F: Fn(&Op, &mut u8)>(update: F, input: &str) -> u32 {
    let mut lights = vec![[0u8; 1000]; 1000];
    for instruction in parse(input) {
        for y in instruction.from.y..=instruction.to.y {
            for x in instruction.from.x..=instruction.to.x {
                update(
                    &instruction.op,
                    &mut lights[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()],
                );
            }
        }
    }
    lights
        .into_iter()
        .map(|row| row.into_iter().map(u32::from).sum::<u32>())
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
