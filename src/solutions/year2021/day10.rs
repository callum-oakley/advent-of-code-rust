enum Error {
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn parse(line: &str) -> Result<(), Error> {
    let mut pending = Vec::new();
    for c in line.trim().chars() {
        match c {
            '(' => pending.push(')'),
            '[' => pending.push(']'),
            '{' => pending.push('}'),
            '<' => pending.push('>'),
            _ => {
                if pending.pop().is_none_or(|p| p != c) {
                    return Err(Error::Corrupted(c));
                }
            }
        }
    }
    if !pending.is_empty() {
        return Err(Error::Incomplete(pending));
    }
    Ok(())
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| match parse(line) {
            Err(Error::Corrupted(')')) => Some(3),
            Err(Error::Corrupted(']')) => Some(57),
            Err(Error::Corrupted('}')) => Some(1197),
            Err(Error::Corrupted('>')) => Some(25137),
            _ => None,
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .lines()
        .filter_map(|line| {
            if let Err(Error::Incomplete(mut pending)) = parse(line) {
                let mut score = 0;
                while let Some(c) = pending.pop() {
                    match c {
                        ')' => score = score * 5 + 1,
                        ']' => score = score * 5 + 2,
                        '}' => score = score * 5 + 3,
                        '>' => score = score * 5 + 4,
                        _ => unreachable!(),
                    }
                }
                Some(score)
            } else {
                None
            }
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

pub fn tests() {
    let example = "
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";
    assert_eq!(part1(example), 26397);
    assert_eq!(part2(example), 288_957);
}
