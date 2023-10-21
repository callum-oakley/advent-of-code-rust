use crate::vm_2018;

pub fn part1(input: &str) -> usize {
    let (ip, prog) = vm_2018::parse(input);
    let mut regs = [0; 6];
    while let Some(&instruction) = prog.get(regs[ip]) {
        vm_2018::apply(instruction, &mut regs);
        regs[ip] += 1;
    }
    regs[0]
}

// - lines 01 to 16 loop through every pair of numbers r1 and r3 <= r2, test if r1 * r3 = r2, and if
//   they do, increment r0 by r1. in other words: they find the sum of the divisors of r2
// - lines 17 to 35 initialise r2 (to 898 in part 1 and 10551298 in part 2)
pub fn part2(_: &str) -> usize {
    let r2 = 10_551_298;
    (1..=r2).filter(|d| r2 % d == 0).sum()
}

pub fn tests() {
    let example = [
        "#ip 0",
        "seti 5 0 1",
        "seti 6 0 2",
        "addi 0 1 0",
        "addr 1 2 3",
        "setr 1 0 0",
        "seti 8 0 4",
        "seti 9 0 5",
    ]
    .join("\n");
    assert_eq!(part1(&example), 7);
}

//        #ip 4
// 00     addi 4 16 4     jump to 17
// 01     seti 1 5 1      r1 = 1
// 02     seti 1 7 3      r3 = 1
// 03     mulr 1 3 5      r5 = r1 * r3
// 04     eqrr 5 2 5
// 05     addr 5 4 4
// 06     addi 4 1 4
// 07     addr 1 0 0
// 08     addi 3 1 3      if r5 = r2, r0 += r1, else inc r3
// 09     gtrr 3 2 5
// 10     addr 4 5 4
// 11     seti 2 4 4
// 12     addi 1 1 1      if r3 > r2, inc r1, else jump to 3
// 13     gtrr 1 2 5
// 14     addr 5 4 4
// 15     seti 1 5 4
// 16     mulr 4 4 4      if r1 > r2, halt, else jump to 2
// 17     addi 2 2 2      r2 = 2
// 18     mulr 2 2 2      r2 = r2 * r2 (2 * 2 = 4)
// 19     mulr 4 2 2      r2 = r2 * r4 (4 * 19 = 76)
// 20     muli 2 11 2     r2 = r2 * 11 (76 * 11 = 836)
// 21     addi 5 2 5      r5 = 2
// 22     mulr 5 4 5      r5 = r5 * r4 (2 * 22 = 44)
// 23     addi 5 18 5     r5 = r5 + 18 (44 + 18 = 62)
// 24     addr 2 5 2      r2 = r2 + r5 (836 + 62 = 898)
// 25     addr 4 0 4
// 26     seti 0 6 4      when part 1, jump to 1
// 27     setr 4 3 5      r5 = r4 (27)
// 28     mulr 5 4 5      r5 = r5 * r4 (27 * 28 = 756)
// 29     addr 4 5 5      r5 = r4 + r5 (29 + 756 = 785)
// 30     mulr 4 5 5      r5 = r4 * r5 (30 * 785 = 23550)
// 31     muli 5 14 5     r5 = r5 * 14 (23550 * 14 = 329700)
// 32     mulr 5 4 5      r5 = r5 * r4 (329700 * 32 = 10550400)
// 33     addr 2 5 2      r2 = r2 + r5 (898 + 10550400 = 10551298)
// 34     seti 0 2 0      r0 = 0
// 35     seti 0 6 4      jump to 1
