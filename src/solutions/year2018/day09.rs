use regex::Regex;

fn parse(input: &str) -> (usize, usize) {
    let re = Regex::new(r"\d+").unwrap();
    let mut numbers = re.find_iter(input).map(|s| s.as_str().parse().unwrap());
    (numbers.next().unwrap(), numbers.next().unwrap())
}

fn play(players: usize, last_marble: usize) -> usize {
    let mut left = vec![0; last_marble + 1];
    let mut right = vec![0; last_marble + 1];
    let mut current = 0;
    let mut scores = vec![0; players];

    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            for _ in 0..6 {
                current = left[current];
            }
            let b = left[current];
            let a = left[b];
            right[a] = current;
            left[current] = a;
            scores[marble % players] += marble + b;
        } else {
            let a = right[current];
            let b = right[a];
            right[a] = marble;
            left[marble] = a;
            right[marble] = b;
            left[b] = marble;
            current = marble;
        }
    }

    scores.into_iter().max().unwrap()
}

pub fn part1(input: &str) -> usize {
    let (players, last_marble) = parse(input);
    play(players, last_marble)
}

pub fn part2(input: &str) -> usize {
    let (players, last_marble) = parse(input);
    play(players, last_marble * 100)
}

pub fn tests() {
    assert_eq!(part1("9 players; last marble is 25"), 32);
    assert_eq!(part1("10 players; last marble is 1618"), 8317);
    assert_eq!(part1("13 players; last marble is 7999"), 146_373);
    assert_eq!(part1("17 players; last marble is 1104"), 2764);
    assert_eq!(part1("21 players; last marble is 6111"), 54718);
    assert_eq!(part1("30 players; last marble is 5807"), 37305);
}
