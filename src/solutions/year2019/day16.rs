fn pattern(j: usize, i: usize) -> i32 {
    match (i + 1) / (j + 1) % 4 {
        0 | 2 => 0,
        1 => 1,
        3 => -1,
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> String {
    let mut signal: Vec<i32> = input
        .chars()
        .map(|c| i32::try_from(c.to_digit(10).unwrap()).unwrap())
        .collect();

    for _ in 0..100 {
        signal = (0..signal.len())
            .map(|j| {
                signal
                    .iter()
                    .enumerate()
                    .map(|(i, &d)| d * pattern(j, i))
                    .sum::<i32>()
                    .abs()
                    % 10
            })
            .collect();
    }

    signal[..8]
        .iter()
        .map(|&d| char::from_digit(u32::try_from(d).unwrap(), 10).unwrap())
        .collect()
}

// Relies on offset pointing to the latter half of the signal, in which case pattern is 0 for i < j
// and 1 for i >= j and we're just doing cumulative sums.
pub fn part2(input: &str) -> String {
    let offset: usize = input[..7].parse().unwrap();

    let mut signal: Vec<u8> = input
        .chars()
        .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
        .collect();
    // we can ignore the part of the signal before offset
    signal = (offset..signal.len() * 10_000)
        .map(|i| signal[i % signal.len()])
        .collect();

    for _ in 0..100 {
        for i in (0..(signal.len() - 1)).rev() {
            // pattern is always 1 this far through the signal
            signal[i] = (signal[i] + signal[i + 1]) % 10;
        }
    }

    signal[..8]
        .iter()
        .map(|&d| char::from_digit(u32::from(d), 10).unwrap())
        .collect()
}

pub fn tests() {
    assert_eq!(
        (0..15).map(|i| pattern(1, i)).collect::<Vec<_>>(),
        vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1],
    );

    assert_eq!(part1("80871224585914546619083218645595"), "24176176");
    assert_eq!(part1("19617804207202209144916044189917"), "73745418");
    assert_eq!(part1("69317163492948606335995924319873"), "52432133");

    assert_eq!(part2("03036732577212944063491565474664"), "84462026");
    assert_eq!(part2("02935109699940807407585447034323"), "78725270");
    assert_eq!(part2("03081770884921959731165446850517"), "53553731");
}
