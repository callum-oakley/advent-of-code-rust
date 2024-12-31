use crate::grid::Grid;

fn parse(input: &str) -> (Vec<u32>, Vec<Grid<u32>>) {
    let mut paragraphs = input.split("\n\n");
    let draw = paragraphs
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let boards = paragraphs
        .map(|board| Grid::from_iter([5, 5], board.split_whitespace().map(|s| s.parse().unwrap())))
        .collect();
    (draw, boards)
}

fn bingo(draw: &[u32], board: &Grid<u32>) -> bool {
    (0..board.size.x).any(|x| (0..board.size.y).all(|y| draw.contains(&board[[x, y]])))
        || (0..board.size.y).any(|y| (0..board.size.x).all(|x| draw.contains(&board[[x, y]])))
}

fn play(draw: &[u32], board: &Grid<u32>) -> usize {
    (0..draw.len()).find(|&i| bingo(&draw[..i], board)).unwrap()
}

fn score(draw: &[u32], board: &Grid<u32>) -> u32 {
    board.values().filter(|&n| !draw.contains(n)).sum::<u32>() * draw.last().unwrap()
}

pub fn part1(input: &str) -> u32 {
    let (draw, boards) = parse(input);
    boards
        .iter()
        .map(|board| (board, play(&draw, board)))
        .min_by_key(|&(_, i)| i)
        .map(|(board, i)| score(&draw[..i], board))
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let (draw, boards) = parse(input);
    boards
        .iter()
        .map(|board| (board, play(&draw, board)))
        .max_by_key(|&(_, i)| i)
        .map(|(board, i)| score(&draw[..i], board))
        .unwrap()
}

pub fn tests() {
    let example = [
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
        "",
        " 3 15  0  2 22",
        " 9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
    ]
    .join("\n");
    assert_eq!(part1(&example), 4512);
    assert_eq!(part2(&example), 1924);
}
