use crate::{
    combinatorics::permute,
    intcode::{State, VM},
};

fn amplify1(vm: &VM, phases: &[i64]) -> i64 {
    let mut signal = 0;
    for &phase in phases {
        let mut amp = vm.clone();
        amp.input(phase);
        amp.input(signal);
        signal = amp.output();
        amp.halt();
    }
    signal
}

fn amplify2(vm: &VM, phases: &[i64]) -> i64 {
    let mut amps = [vm.clone(), vm.clone(), vm.clone(), vm.clone(), vm.clone()];
    for i in 0..5 {
        amps[i].input(phases[i]);
    }
    amps[0].input(0);

    let mut i = 0;
    loop {
        if amps[i].state() == State::Output {
            match amps[(i + 1) % 5].state() {
                State::Input => {
                    let signal = amps[i].output();
                    amps[(i + 1) % 5].input(signal);
                }
                State::Output => {}
                State::Halt => {
                    assert_eq!(i, 4, "expected final output to come from amp E");
                    let signal = amps[i].output();
                    for amp in &mut amps {
                        amp.halt();
                    }
                    return signal;
                }
            }
        }
        i = (i + 1) % 5;
    }
}

pub fn part1(input: &str) -> i64 {
    let vm = VM::new(input);
    let mut phases = [0, 1, 2, 3, 4];
    let mut max_signal = amplify1(&vm, &phases);
    while permute(&mut phases) {
        max_signal = max_signal.max(amplify1(&vm, &phases));
    }
    max_signal
}

pub fn part2(input: &str) -> i64 {
    let vm = VM::new(input);
    let mut phases = [5, 6, 7, 8, 9];
    let mut max_signal = amplify2(&vm, &phases);
    while permute(&mut phases) {
        max_signal = max_signal.max(amplify2(&vm, &phases));
    }
    max_signal
}

pub fn tests() {
    assert_eq!(
        part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        43210,
    );
    assert_eq!(
        part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        54321,
    );
    assert_eq!(
        part1(
            &[
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,",
                "1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
            ]
            .concat(),
        ),
        65210,
    );

    assert_eq!(
        part2(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        ),
        139_629_729,
    );
    assert_eq!(
        part2(
            &[
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,",
                "-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,",
                "53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            ]
            .concat(),
        ),
        18216,
    );
}
