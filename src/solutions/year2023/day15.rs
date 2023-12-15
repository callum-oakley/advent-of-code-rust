struct HashMap<'a> {
    boxes: [Vec<(&'a str, usize)>; 256],
}

impl<'a> HashMap<'a> {
    fn insert(&mut self, label: &'a str, lens: usize) {
        let slots = &mut self.boxes[hash(label)];
        for entry in &mut *slots {
            if entry.0 == label {
                entry.1 = lens;
                return;
            }
        }
        slots.push((label, lens));
    }

    fn remove(&mut self, label: &'a str) {
        let slots = &mut self.boxes[hash(label)];
        for j in 0..slots.len() {
            if slots[j].0 == label {
                slots.remove(j);
                return;
            }
        }
    }
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0u8, |v, &byte| v.wrapping_add(byte).wrapping_mul(17))
        .into()
}

pub fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

pub fn part2(input: &str) -> usize {
    const EMPTY: Vec<(&str, usize)> = Vec::new();
    let mut map = HashMap {
        boxes: [EMPTY; 256],
    };
    for s in input.split(',') {
        if &s[s.len() - 1..] == "-" {
            map.remove(&s[..s.len() - 1]);
        } else {
            let (label, lens) = s.split_once('=').unwrap();
            map.insert(label, lens.parse().unwrap());
        }
    }
    map.boxes
        .into_iter()
        .enumerate()
        .flat_map(|(i, slots)| {
            slots
                .into_iter()
                .enumerate()
                .map(move |(j, (_, lens))| (i + 1) * (j + 1) * lens)
        })
        .sum()
}

pub fn tests() {
    let example = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(hash("HASH"), 52);
    assert_eq!(part1(example), 1320);
    assert_eq!(part2(example), 145);
}
