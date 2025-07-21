fn part_(window_len: usize, input: &str) -> usize {
    input
        .as_bytes()
        .windows(window_len)
        .take_while(|&window| window.iter().any(|&c| bytecount::count(window, c) > 1))
        .count()
        + window_len
}

pub fn part1(input: &str) -> usize {
    part_(4, input)
}

pub fn part2(input: &str) -> usize {
    part_(14, input)
}

pub fn tests() {
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}
