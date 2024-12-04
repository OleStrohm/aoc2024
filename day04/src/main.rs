use std::ops::Index;

use itertools::iproduct;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn add(self, multiple: isize, x: isize, y: isize) -> Option<Pos> {
        Some(Pos {
            x: self.x.checked_add_signed(multiple * x)?,
            y: self.y.checked_add_signed(multiple * y)?,
        })
    }
}

impl<T> Index<Pos> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, Pos { x, y }: Pos) -> &Self::Output {
        &self[y][x]
    }
}

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let board: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let width = board[0].len();
    let height = board.len();

    let part1 = iproduct!(0..width, 0..height)
        .map(move |(x, y)| Pos { x, y })
        .flat_map(|p| {
            iproduct!(-1..=1, -1..=1).filter_map(move |(dx, dy)| {
                Some([
                    p.add(0, dx, dy)?,
                    p.add(1, dx, dy)?,
                    p.add(2, dx, dy)?,
                    p.add(3, dx, dy)?,
                ])
            })
        })
        .filter(|ps| ps.iter().all(|&Pos { x, y }| x < width && y < height))
        .filter(|&[p0, p1, p2, p3]| {
            (board[p0], board[p1], board[p2], board[p3]) == ('X', 'M', 'A', 'S')
        })
        .count();

    let part2 = (0..width)
        .flat_map(|x| (0..height).map(move |y| Pos { x, y }))
        .filter_map(|p| {
            Some([
                p.add(1, -1, -1)?,
                p.add(1, 1, 1)?,
                p.add(0, 0, 0)?,
                p.add(1, -1, 1)?,
                p.add(1, 1, -1)?,
            ])
        })
        .filter(|ps| ps.iter().all(|&Pos { x, y }| x < width && y < height))
        .filter(|&[top_left, bottom_right, mid, bottom_left, top_right]| {
            let diag1 = (board[top_left], board[mid], board[bottom_right]);
            let diag2 = (board[bottom_left], board[mid], board[top_right]);
            (diag1 == ('M', 'A', 'S') || diag1 == ('S', 'A', 'M'))
                && (diag2 == ('M', 'A', 'S') || diag2 == ('S', 'A', 'M'))
        })
        .count();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
