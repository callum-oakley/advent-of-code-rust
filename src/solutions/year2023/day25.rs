use std::collections::HashMap;

use rand::seq::SliceRandom;

// TODO this is slow. Try a different approach.
pub fn part1(input: &str) -> usize {
    // https://en.wikipedia.org/wiki/Karger%27s_algorithm
    let mut edges: Vec<(&str, &str)> = Vec::new();
    let mut nodes: HashMap<&str, usize> = HashMap::new();
    for line in input.trim().lines() {
        let (a, bs) = line.trim().split_once(": ").unwrap();
        nodes.insert(a, 1);
        for b in bs.split_whitespace() {
            nodes.insert(b, 1);
            edges.push((a, b));
        }
    }

    let mut rng = rand::thread_rng();

    loop {
        let mut edges = edges.clone();
        let mut nodes = nodes.clone();

        while nodes.len() > 2 {
            let &(a, b) = edges.choose(&mut rng).unwrap();

            // Relabel edges
            for edge in &mut edges {
                if edge.0 == b {
                    edge.0 = a;
                }
                if edge.1 == b {
                    edge.1 = a;
                }
            }

            // Remove self loops
            let mut i = 0;
            while i < edges.len() {
                if edges[i].0 == edges[i].1 {
                    edges.swap_remove(i);
                    continue;
                }
                i += 1;
            }

            // Update component counts
            let b_count = nodes.remove(b).unwrap();
            *nodes.get_mut(a).unwrap() += b_count;
        }

        if edges.len() == 3 {
            return nodes.values().product();
        }
    }
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
