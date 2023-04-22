use crate::uniq::Uniq;

fn parse(input: &str) -> (Vec<(&str, &str)>, &str) {
    let (reactions, molecule) = input.split_once("\n\n").unwrap();
    (
        reactions
            .lines()
            .map(|line| line.split_once(" => ").unwrap())
            .collect(),
        molecule,
    )
}

fn step<'a>(
    reactions: &'a [(&'a str, &'a str)],
    molecule: &'a str,
) -> impl Iterator<Item = String> + 'a {
    reactions.iter().flat_map(|(reactant, product)| {
        molecule.match_indices(reactant).map(|(i, _)| {
            let mut res = String::new();
            res.push_str(&molecule[0..i]);
            res.push_str(product);
            res.push_str(&molecule[i + reactant.len()..]);
            res
        })
    })
}

pub fn part1(input: &str) -> usize {
    let (reactions, molecule) = parse(input);
    Uniq::new(step(&reactions, molecule)).count()
}

pub fn tests() {
    assert_eq!(part1("H => HO\nH => OH\nO => HH\n\nHOH"), 4);
    assert_eq!(part1("H => HO\nH => OH\nO => HH\n\nHOHOHO"), 7);
}
