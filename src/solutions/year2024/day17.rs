use regex::Regex;

fn run(prog: &[usize], mut a: usize, mut b: usize, mut c: usize) -> Vec<usize> {
    fn combo(a: usize, b: usize, c: usize, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("unknown operand: {operand}"),
        }
    }

    let mut out = Vec::new();

    let mut ip = 0;
    while ip < prog.len() {
        let op = prog[ip];
        let operand = prog[ip + 1];
        ip += 2;
        match op {
            0 => a >>= combo(a, b, c, operand),
            1 => b ^= operand,
            2 => b = combo(a, b, c, operand) % 8,
            3 => {
                if a != 0 {
                    ip = operand;
                }
            }
            4 => b ^= c,
            5 => out.push(combo(a, b, c, operand) % 8),
            6 => b = a >> combo(a, b, c, operand),
            7 => c = a >> combo(a, b, c, operand),
            _ => panic!("unknown op: {op}"),
        }
    }

    out
}

fn parse(input: &str) -> (Vec<usize>, usize, usize, usize) {
    let re = Regex::new(r"\d+").unwrap();
    let mut ints = re.find_iter(input);
    let a = ints.next().unwrap().as_str().parse().unwrap();
    let b = ints.next().unwrap().as_str().parse().unwrap();
    let c = ints.next().unwrap().as_str().parse().unwrap();
    let prog = ints.map(|m| m.as_str().parse().unwrap()).collect();
    (prog, a, b, c)
}

pub fn part1(input: &str) -> String {
    let (prog, a, b, c) = parse(input);
    run(&prog, a, b, c)
        .iter()
        .map(ToString::to_string)
        .reduce(|a, b| format!("{a},{b}"))
        .unwrap()
}

// Reverse engineering the program:
//
// 2,4,1,1,7,5,1,5,4,3,5,5,0,3,3,0
//
// bst a
// bxl 1
// cdv b
// bxl 5
// bxc _
// out b
// adv 3
// jnz 0
//
// while a != 0 {
//     b = a
//     b ^= 1
//     c = a >> b
//     b ^= 5
//     b ^= c
//     out(b)
//     a >>= 3
// }
//
// We can see that since we shift 3 each iteration we output one number for each octal digit of the
// input, and the last n digits of the output depend only on the first n digits of the input, so we
// can find the input by working from left to right, varying a digit at a time until we find a match
// on the suffix of the output, and backtracking if we reach a dead end.
pub fn part2(input: &str) -> usize {
    fn search(prog: &[usize], a: usize, b: usize, c: usize, fixed: usize) -> Option<usize> {
        if fixed == prog.len() {
            Some(a)
        } else {
            (0..8)
                .filter(|d| run(prog, (a << 3) + d, b, c) == prog[prog.len() - fixed - 1..])
                .find_map(|d| search(prog, (a << 3) + d, b, c, fixed + 1))
        }
    }
    let (prog, _, b, c) = parse(input);
    search(&prog, 0, b, c, 0).unwrap()
}

pub fn tests() {
    let example1 = "Register A: 729\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0";
    assert_eq!(part1(example1), "4,6,3,5,6,3,5,2,1,0");
}
