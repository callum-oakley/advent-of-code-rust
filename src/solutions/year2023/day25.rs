use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use rand::seq::IteratorRandom;

use crate::search2;

fn shortest_path<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    start: &'a str,
    goal: &'a str,
) -> Vec<&'a str> {
    struct State<'a> {
        pos: &'a str,
        path: Vec<&'a str>,
    }
    search2::breadth_first(
        State {
            pos: start,
            path: vec![start],
        },
        |state| state.pos,
        |state, push| {
            for &pos in &graph[state.pos] {
                let mut path = state.path.clone();
                path.push(pos);
                push(State { pos, path });
            }
        },
    )
    .find(|state| state.pos == goal)
    .unwrap()
    .path
}

fn component_size(graph: &HashMap<&str, HashSet<&str>>, start: &str) -> usize {
    search2::breadth_first(
        start,
        |&state| state,
        |&state, push| graph[state].iter().copied().for_each(push),
    )
    .count()
}

pub fn part1(input: &str) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.trim().lines() {
        let (a, bs) = line.trim().split_once(": ").unwrap();
        for b in bs.split_whitespace() {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        }
    }

    let mut heatmap: HashMap<[&str; 2], usize> = HashMap::new();

    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let a = *graph.keys().choose(&mut rng).unwrap();
        let b = *graph.keys().choose(&mut rng).unwrap();

        for edge in shortest_path(&graph, a, b).windows(2) {
            let mut edge = [edge[0], edge[1]];
            edge.sort_unstable();
            *heatmap.entry(edge).or_default() += 1;
        }
    }

    let mut edges: Vec<_> = heatmap.keys().copied().collect();
    edges.sort_by_key(|edge| Reverse(heatmap[edge]));

    for &[a, b] in &edges[0..3] {
        graph.get_mut(a).unwrap().remove(b);
        graph.get_mut(b).unwrap().remove(a);
    }

    component_size(&graph, edges[0][0]) * component_size(&graph, edges[0][1])
}

pub fn tests() {
    let example = "
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    ";
    assert_eq!(part1(example), 54);
}
