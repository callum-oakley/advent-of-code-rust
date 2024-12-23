use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use crate::uniq::Uniq;

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut res: HashMap<&str, HashSet<&str>> = HashMap::new();
    for edge in input.split_whitespace() {
        let (a, b) = edge.split_once('-').unwrap();
        res.entry(a).or_default().insert(b);
        res.entry(b).or_default().insert(a);
    }
    res
}

fn maximal_cliques<'a>(g: &HashMap<&'a str, HashSet<&'a str>>) -> Vec<HashSet<&'a str>> {
    // https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    fn bron_kerbosch<'a>(
        g: &HashMap<&'a str, HashSet<&'a str>>,
        r: HashSet<&'a str>,
        mut p: HashSet<&'a str>,
        mut x: HashSet<&'a str>,
    ) -> Vec<HashSet<&'a str>> {
        if p.is_empty() && x.is_empty() {
            vec![r]
        } else {
            let mut res = Vec::new();
            while let Some(&v) = p.iter().next() {
                res.extend(bron_kerbosch(
                    g,
                    r.union(&HashSet::from([v])).copied().collect(),
                    p.intersection(&g[v]).copied().collect(),
                    x.intersection(&g[v]).copied().collect(),
                ));
                p.remove(v);
                x.insert(v);
            }
            res
        }
    }
    bron_kerbosch(
        g,
        HashSet::new(),
        g.keys().copied().collect(),
        HashSet::new(),
    )
}

pub fn part1(input: &str) -> usize {
    let g = parse(input);
    g.keys()
        .filter(|&a| a.starts_with('t'))
        .flat_map(|&a| {
            g[a].iter()
                .filter(|&b| g[b].contains(a))
                .map(move |&b| (a, b))
        })
        .flat_map(|(a, b)| {
            g[a].intersection(&g[b]).map(move |&c| {
                let mut tri = [a, b, c];
                tri.sort_unstable();
                tri
            })
        })
        .uniq()
        .count()
}

pub fn part2(input: &str) -> String {
    let mut cliques = maximal_cliques(&parse(input));
    cliques.sort_unstable_by_key(|c| Reverse(c.len()));
    let mut clique: Vec<&str> = cliques[0].iter().copied().collect();
    clique.sort_unstable();
    clique.join(",")
}

pub fn tests() {
    let example = "
        kh-tc qp-kh de-cg ka-co yn-aq qp-ub cg-tb vc-aq tb-ka wh-tc yn-cg kh-ub ta-co de-co tc-td
        tb-wq wh-td ta-ka td-qp aq-cg wq-ub ub-vc de-ta wq-aq wq-vc wh-yn ka-de kh-ta co-tc wh-qp
        tb-vc td-yn
    ";
    assert_eq!(part1(example), 7);
    assert_eq!(part2(example), "co,de,ka,ta");
}
