use std::iter;

#[derive(Clone, Copy)]
struct Chunk {
    file: Option<usize>,
    len: usize,
}

fn parse(input: &str) -> Vec<Chunk> {
    let mut res = Vec::new();

    let mut lens = input
        .chars()
        .map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap());
    for i in 0.. {
        if let Some(len) = lens.next() {
            res.push(Chunk { file: Some(i), len });
        } else {
            break;
        }

        if let Some(len) = lens.next() {
            res.push(Chunk { file: None, len });
        } else {
            break;
        }
    }

    res
}

pub fn part1(input: &str) -> usize {
    let mut disk: Vec<Option<usize>> = parse(input)
        .into_iter()
        .flat_map(|chunk| iter::repeat_n(chunk.file, chunk.len))
        .collect();

    let mut block = 0;
    while block < disk.len() {
        if disk[block].is_none() {
            while disk.last().unwrap().is_none() {
                disk.pop().unwrap();
            }
            disk[block] = disk.pop().unwrap();
        }
        block += 1;
    }

    disk.into_iter()
        .enumerate()
        .map(|(block, file)| block * file.unwrap())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut chunks = parse(input);

    for i in (0..chunks.len()).rev() {
        if chunks[i].file.is_none() {
            continue;
        }
        if let Some(j) =
            (0..i).find(|&j| chunks[j].file.is_none() && chunks[i].len <= chunks[j].len)
        {
            let chunk = chunks[i];
            chunks[i].file = None;
            chunks[j].len -= chunk.len;
            chunks.insert(j, chunk);
        }
    }

    let mut res = 0;

    let mut block = 0;
    for chunk in chunks {
        for _ in 0..chunk.len {
            res += block * chunk.file.unwrap_or(0);
            block += 1;
        }
    }

    res
}

pub fn tests() {
    assert_eq!(part1("2333133121414131402"), 1928);
    assert_eq!(part2("2333133121414131402"), 2858);
}
