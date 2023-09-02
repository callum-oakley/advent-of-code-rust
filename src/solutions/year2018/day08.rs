use std::iter;

struct Tree {
    children: Vec<Tree>,
    metadata: Vec<u8>,
}

impl<I> From<&mut I> for Tree
where
    I: Iterator<Item = u8>,
{
    fn from(bytes: &mut I) -> Self {
        let child_count = bytes.next().unwrap() as usize;
        let metadata_count = bytes.next().unwrap() as usize;
        Tree {
            children: iter::repeat_with(|| bytes.into())
                .take(child_count)
                .collect(),
            metadata: bytes.take(metadata_count).collect(),
        }
    }
}

impl Tree {
    fn metadata_sum(&self) -> u32 {
        self.children
            .iter()
            .map(Self::metadata_sum)
            .chain(self.metadata.iter().copied().map(u32::from))
            .sum()
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().copied().map(u32::from).sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|&i| self.children.get(i as usize - 1).map(Self::value))
                .sum()
        }
    }
}

pub fn part1(input: &str) -> u32 {
    Tree::from(&mut input.split_whitespace().map(|w| w.parse::<u8>().unwrap())).metadata_sum()
}

pub fn part2(input: &str) -> u32 {
    Tree::from(&mut input.split_whitespace().map(|w| w.parse::<u8>().unwrap())).value()
}

pub fn tests() {
    assert_eq!(part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
    assert_eq!(part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 66);
}
