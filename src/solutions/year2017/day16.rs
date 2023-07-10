enum Step {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn parse(input: &str) -> Vec<Step> {
    input
        .split(',')
        .map(|s| match &s[..1] {
            "s" => Step::Spin(s[1..].parse().unwrap()),
            "x" => {
                let (i, j) = s[1..].split_once('/').unwrap();
                Step::Exchange(i.parse().unwrap(), j.parse().unwrap())
            }
            "p" => {
                let (x, y) = s[1..].split_once('/').unwrap();
                Step::Partner(x.as_bytes()[0], y.as_bytes()[0])
            }
            _ => unreachable!(),
        })
        .collect()
}

fn dance(steps: &[Step], mut progs: Vec<u8>) -> Vec<u8> {
    for step in steps {
        match *step {
            Step::Spin(i) => progs.rotate_right(i),
            Step::Exchange(i, j) => progs.swap(i, j),
            Step::Partner(x, y) => {
                let i = progs.iter().position(|&z| z == x).unwrap();
                let j = progs.iter().position(|&z| z == y).unwrap();
                progs.swap(i, j);
            }
        }
    }
    progs
}

pub fn part1(input: &str) -> String {
    String::from_utf8(dance(&parse(input), Vec::from("abcdefghijklmnop"))).unwrap()
}

pub fn part2(input: &str) -> String {
    let steps = parse(input);

    let initial = Vec::from("abcdefghijklmnop");
    let mut progs = dance(&steps, initial.clone());
    let mut period = 1;
    while progs != initial {
        progs = dance(&steps, progs);
        period += 1;
    }

    for _ in 0..(1_000_000_000 % period) {
        progs = dance(&steps, progs);
    }

    String::from_utf8(progs).unwrap()
}

pub fn tests() {
    assert_eq!(
        String::from_utf8(dance(&parse("s1,x3/4,pe/b"), Vec::from("abcde"))).unwrap(),
        "baedc"
    );
}
