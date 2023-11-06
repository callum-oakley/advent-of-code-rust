use crate::intcode::VM;

pub fn part1(input: &str) -> i64 {
    let mut vm = VM::new(input);
    vm.input(1);
    loop {
        let output = vm.output();
        if output != 0 {
            vm.halt();
            return output;
        }
    }
}

pub fn part2(input: &str) -> i64 {
    let mut vm = VM::new(input);
    vm.input(5);
    let output = vm.output();
    vm.halt();
    output
}

pub fn tests() {
    fn assert_output(prog: &str, input: i64, output: i64) {
        let mut vm = VM::new(prog);
        vm.input(input);
        assert_eq!(vm.output(), output);
        vm.halt();
    }

    assert_output("3,9,8,9,10,9,4,9,99,-1,8", 7, 0);
    assert_output("3,9,8,9,10,9,4,9,99,-1,8", 8, 1);
    assert_output("3,9,8,9,10,9,4,9,99,-1,8", 9, 0);

    assert_output("3,9,7,9,10,9,4,9,99,-1,8", 7, 1);
    assert_output("3,9,7,9,10,9,4,9,99,-1,8", 8, 0);
    assert_output("3,9,7,9,10,9,4,9,99,-1,8", 9, 0);

    assert_output("3,3,1108,-1,8,3,4,3,99", 7, 0);
    assert_output("3,3,1108,-1,8,3,4,3,99", 8, 1);
    assert_output("3,3,1108,-1,8,3,4,3,99", 9, 0);

    assert_output("3,3,1107,-1,8,3,4,3,99", 7, 1);
    assert_output("3,3,1107,-1,8,3,4,3,99", 8, 0);
    assert_output("3,3,1107,-1,8,3,4,3,99", 9, 0);

    assert_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", -1, 1);
    assert_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0);
    assert_output("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 1, 1);

    assert_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", -1, 1);
    assert_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0);
    assert_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1, 1);

    let larger_example = [
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,",
        "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,",
        "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
    ]
    .concat();

    assert_output(&larger_example, 7, 999);
    assert_output(&larger_example, 8, 1000);
    assert_output(&larger_example, 9, 1001);
}
