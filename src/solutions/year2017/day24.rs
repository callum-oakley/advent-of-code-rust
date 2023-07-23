fn parse(input: &str) -> Vec<Option<(u32, u32)>> {
    input
        .split_whitespace()
        .map(|component| {
            let (x, y) = component.split_once('/').unwrap();
            Some((x.parse().unwrap(), y.parse().unwrap()))
        })
        .collect()
}

fn strength(bridge: &[(u32, u32)]) -> u32 {
    bridge.iter().map(|&(a, b)| a + b).sum()
}

fn part_<F>(
    hook: &mut F,
    components: &mut Vec<Option<(u32, u32)>>,
    bridge: &mut Vec<(u32, u32)>,
    prev: u32,
) where
    F: FnMut(&[(u32, u32)]),
{
    hook(bridge);

    for i in 0..components.len() {
        if let Some((a, b)) = components[i] {
            if a == prev {
                components[i] = None;
                bridge.push((a, b));
                part_(hook, components, bridge, b);
                bridge.pop();
                components[i] = Some((a, b));
            } else if b == prev {
                components[i] = None;
                bridge.push((b, a));
                part_(hook, components, bridge, a);
                bridge.pop();
                components[i] = Some((a, b));
            }
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut res = 0;
    part_(
        &mut |bridge| {
            res = res.max(strength(bridge));
        },
        &mut parse(input),
        &mut Vec::new(),
        0,
    );
    res
}

pub fn part2(input: &str) -> u32 {
    let mut longest = 0;
    let mut strongest = 0;
    part_(
        &mut |bridge| {
            if bridge.len() >= longest {
                longest = bridge.len();
                strongest = strongest.max(strength(bridge));
            }
        },
        &mut parse(input),
        &mut Vec::new(),
        0,
    );
    strongest
}

pub fn tests() {
    assert_eq!(part1("0/2 2/2 2/3 3/4 3/5 0/1 10/1 9/10"), 31);
    assert_eq!(part2("0/2 2/2 2/3 3/4 3/5 0/1 10/1 9/10"), 19);
}
