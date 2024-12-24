use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    op: Op,
}

fn parse(input: &str) -> (HashMap<&str, bool>, HashMap<&str, Gate>) {
    let (init, gates) = input.trim().split_once("\n\n").unwrap();
    (
        init.lines()
            .map(|line| {
                let (wire, signal) = line.trim().split_once(": ").unwrap();
                let signal = match signal {
                    "0" => false,
                    "1" => true,
                    _ => unreachable!(),
                };
                (wire, signal)
            })
            .collect(),
        gates
            .lines()
            .map(|line| {
                let (input, output) = line.split_once(" -> ").unwrap();
                let mut input = input.split_whitespace();
                let a = input.next().unwrap();
                let op = input.next().unwrap();
                let b = input.next().unwrap();
                let op = match op {
                    "AND" => Op::And,
                    "OR" => Op::Or,
                    "XOR" => Op::Xor,
                    _ => unreachable!(),
                };
                (output, Gate { a, b, op })
            })
            .collect(),
    )
}

fn eval(init: &HashMap<&str, bool>, gates: &HashMap<&str, Gate>, wire: &str) -> bool {
    if let Some(&value) = init.get(wire) {
        value
    } else {
        let gate = &gates[wire];
        match gate.op {
            Op::And => eval(init, gates, gate.a) && eval(init, gates, gate.b),
            Op::Or => eval(init, gates, gate.a) || eval(init, gates, gate.b),
            Op::Xor => eval(init, gates, gate.a) ^ eval(init, gates, gate.b),
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let (init, gates) = parse(input);
    init.keys()
        .chain(gates.keys())
        .filter(|wire| wire.starts_with('z'))
        .map(|&wire| {
            if eval(&init, &gates, wire) {
                2u64.pow(wire.trim_start_matches('z').parse().unwrap())
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> String {
    let (init, gates) = parse(input);
    todo!()
}

pub fn tests() {
    let example0 = "
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    ";
    let example1 = "
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    ";
    assert_eq!(part1(example0), 4);
    assert_eq!(part1(example1), 2024);
}
