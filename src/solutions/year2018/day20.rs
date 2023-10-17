use std::collections::{HashMap, HashSet};

use crate::{
    grid::{Point, Z},
    search,
};

// Construct a map of the base as a graph by stepping through the regex keeping track of all the
// possible positions we could be in while matching the regex up to this point
fn expand(input: &str) -> HashMap<Point, HashSet<Point>> {
    let mut graph: HashMap<_, HashSet<_>> = HashMap::new();
    let mut active = HashSet::from([Z]);
    let mut forks = Vec::new();
    let mut alts = Vec::new();
    for c in input.chars() {
        match c {
            '^' | '$' => (),
            '(' => {
                // Save starting active positions so we can load them when we reach a '|'
                forks.push(active.clone());
                // To keep track of the different ending active positions we can reach in this group
                alts.push(Vec::new());
            }
            '|' => {
                // Amend the possible ending active positions for this group
                alts.last_mut().unwrap().extend(active);
                // Reset active back to what it was at the last '('
                active = forks.last().unwrap().clone();
            }
            ')' => {
                // Add all alternative positions we could have reached by the end of this group
                active.extend(alts.pop().unwrap());
                forks.pop().unwrap();
            }
            _ => {
                let dir = c.into();
                // Insert all the doors that must exist to move in this direction from the active
                // positions
                for &pos in &active {
                    graph.entry(pos).or_default().insert(pos + dir);
                    graph.entry(pos + dir).or_default().insert(pos);
                }
                // Move all the active positions in this direction
                active = active.into_iter().map(|pos| pos + dir).collect();
            }
        }
    }
    graph
}

#[derive(Clone)]
struct State<'a> {
    graph: &'a HashMap<Point, HashSet<Point>>,
    pos: Point,
    dist: usize,
}

impl<'a> search::State for State<'a> {
    type HashKey = Point;

    fn adjacent(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(self.graph[&self.pos].iter().map(|&pos| State {
            graph: self.graph,
            pos,
            dist: self.dist + 1,
        }))
    }

    fn hash_key(&self) -> Self::HashKey {
        self.pos
    }
}

pub fn part1(input: &str) -> usize {
    let graph = expand(input);
    search::breadth_first(State {
        graph: &graph,
        pos: Z,
        dist: 0,
    })
    .last()
    .unwrap()
    .dist
}

pub fn part2(input: &str) -> usize {
    let graph = expand(input);
    search::breadth_first(State {
        graph: &graph,
        pos: Z,
        dist: 0,
    })
    .filter(|s| s.dist >= 1000)
    .count()
}

pub fn tests() {
    assert_eq!(part1("^WNE$"), 3);
    assert_eq!(part1("^ENWWW(NEEE|SSE(EE|N))$"), 10);
    assert_eq!(part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18);
    assert_eq!(
        part1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"),
        23,
    );
    assert_eq!(
        part1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"),
        31,
    );
}
