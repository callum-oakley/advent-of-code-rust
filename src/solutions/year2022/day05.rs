use std::sync::LazyLock;

use regex::Regex;

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Vec<Vec<char>>, impl Iterator<Item = Instruction> + '_) {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

    let (diagram, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = Vec::new();
    for (v, c) in crate::grid::scan(diagram) {
        if c.is_ascii_uppercase() {
            let col = usize::try_from((v.x - 1) / 4).unwrap();
            while stacks.len() < col + 1 {
                stacks.push(Vec::new());
            }
            stacks[col].insert(0, c);
        }
    }

    (
        stacks,
        RE.captures_iter(instructions).map(|captures| Instruction {
            count: captures[1].parse().unwrap(),
            from: captures[2].parse().unwrap(),
            to: captures[3].parse().unwrap(),
        }),
    )
}

pub fn part1(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);
    for instruction in instructions {
        let stack_len = stacks[instruction.from - 1].len();
        let moving = stacks[instruction.from - 1].split_off(stack_len - instruction.count);
        stacks[instruction.to - 1].extend(moving.iter().rev());
    }
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn part2(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);
    for instruction in instructions {
        let stack_len = stacks[instruction.from - 1].len();
        let moving = stacks[instruction.from - 1].split_off(stack_len - instruction.count);
        stacks[instruction.to - 1].extend(moving);
    }
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn tests() {
    let example = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n",
    );
    assert_eq!(part1(example), "CMZ");
    assert_eq!(part2(example), "MCD");
}
