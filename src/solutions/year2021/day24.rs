use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Arg {
    Var(char),
    Num(i32),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Inp(char),
    Add(char, Arg),
    Mul(char, Arg),
    Div(char, Arg),
    Mod(char, Arg),
    Eql(char, Arg),
}

fn parse_arg(s: &str) -> Arg {
    match s {
        "w" | "x" | "y" | "z" => Arg::Var(s.as_bytes()[0].into()),
        _ => Arg::Num(s.parse().unwrap()),
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    Regex::new(r"(?m)^(\S+) (\S+)(?: (\S+))?$")
        .unwrap()
        .captures_iter(input)
        .map(|captures| match &captures[1] {
            "inp" => Instruction::Inp(captures[2].chars().next().unwrap()),
            "add" => Instruction::Add(captures[2].chars().next().unwrap(), parse_arg(&captures[3])),
            "mul" => Instruction::Mul(captures[2].chars().next().unwrap(), parse_arg(&captures[3])),
            "div" => Instruction::Div(captures[2].chars().next().unwrap(), parse_arg(&captures[3])),
            "mod" => Instruction::Mod(captures[2].chars().next().unwrap(), parse_arg(&captures[3])),
            "eql" => Instruction::Eql(captures[2].chars().next().unwrap(), parse_arg(&captures[3])),
            _ => unreachable!(),
        })
        .collect()
}

fn value(vars: &HashMap<char, i32>, b: Arg) -> i32 {
    match b {
        Arg::Var(v) => vars[&v],
        Arg::Num(n) => n,
    }
}

fn is_valid(instructions: &[Instruction], n: u64) -> bool {
    let n = n.to_string();
    let mut digits = n
        .chars()
        .map(|c| i32::try_from(c.to_digit(10).unwrap()).unwrap());
    let mut vars = HashMap::from([('w', 0), ('x', 0), ('y', 0), ('z', 0)]);
    for &instruction in instructions {
        match instruction {
            Instruction::Inp(a) => {
                vars.insert(a, digits.next().unwrap());
            }
            Instruction::Add(a, b) => {
                vars.insert(a, vars[&a] + value(&vars, b));
            }
            Instruction::Mul(a, b) => {
                vars.insert(a, vars[&a] * value(&vars, b));
            }
            Instruction::Div(a, b) => {
                vars.insert(a, vars[&a] / value(&vars, b));
            }
            Instruction::Mod(a, b) => {
                vars.insert(a, vars[&a].rem_euclid(value(&vars, b)));
            }
            Instruction::Eql(a, b) => {
                vars.insert(a, (vars[&a] == value(&vars, b)).into());
            }
        }
    }
    vars[&'z'] == 0
}

// Think of z in base 26, and 1 operations push a new digit on the right (the "condition" is
// impossible to satisfy), while 26 operations pop a digit off the right as long as the condition is
// satisfied (and every one must be, since there are exactly 7 pushes and 7 pops).
//
//                    maximizing         minimizing
//     [1 11 6]       9 [x]              9 [x]
//     [1 13 14]      9 [x]              2 [x]
//     [1 15 14]      3 [x]              1 [x]
//     [26 -8 10]     3 + 14 - 8 = 9     1 + 14 - 8 = 7
//     [1 13 9]       4 [x]              1 [x]
//     [1 15 12]      8 [x]              1 [x]
//     [26 -11 8]     8 + 12 - 11 = 9    1 + 12 - 11 = 2
//     [26 -4 13]     4 + 9 - 4 = 9      1 + 9 - 4 = 6
//     [26 -15 12]    9 + 14 - 15 = 8    2 + 14 - 15 = 1
//     [1 14 6]       9 [x]              3 [x]
//     [1 14 9]       1 [x]              1 [x]
//     [26 -1 15]     1 + 9 - 1 = 9      1 + 9 - 1 = 9
//     [26 -8 4]      9 + 6 - 8 = 7      3 + 6 - 8 = 1
//     [26 -14 10]    9 + 6 - 14 = 1     9 + 6 - 14 = 1

pub fn part1(input: &str) -> u64 {
    let n = 99_394_899_891_971;
    assert!(is_valid(&parse(input), n));
    n
}

pub fn part2(input: &str) -> u64 {
    let n = 92_171_126_131_911;
    assert!(is_valid(&parse(input), n));
    n
}
