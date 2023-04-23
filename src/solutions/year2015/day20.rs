// It's the sieve of Eratosthenes in disguise.
fn deliver(limit: usize, multiplier: usize, stamina: usize) -> Vec<usize> {
    let mut houses = vec![0; limit];
    for elf in 1..limit {
        for i in (elf..limit).step_by(elf).take(stamina) {
            houses[i] += elf * multiplier;
        }
    }
    houses
}

pub fn part1(input: &str) -> usize {
    let input = input.parse::<usize>().unwrap();
    deliver(1_000_000, 10, 1_000_000)
        .into_iter()
        .enumerate()
        .find_map(|(i, presents)| if input <= presents { Some(i) } else { None })
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let input = input.parse::<usize>().unwrap();
    deliver(1_000_000, 11, 50)
        .into_iter()
        .enumerate()
        .find_map(|(i, presents)| if input <= presents { Some(i) } else { None })
        .unwrap()
}

pub fn tests() {
    assert_eq!(
        deliver(10, 10, 10),
        vec![0, 10, 30, 40, 70, 60, 120, 80, 150, 130]
    );
}
