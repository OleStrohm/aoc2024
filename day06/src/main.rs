use std::collections::BTreeSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Obstruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn rotate(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn next(self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Dir::Up => (x, y - 1),
            Dir::Right => (x + 1, y),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
        }
    }
}

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Tile::Obstruction,
                    _ => Tile::Empty,
                })
                .collect_vec()
        })
        .collect_vec();

    let ((x, y), _) = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .find(|&(_, c)| c == '^')
        .unwrap();

    {
        let (mut x, mut y) = (x, y);
        let mut visited = BTreeSet::<(isize, isize)>::new();
        let mut dir = Dir::Up;

        let in_bounds = |x: isize, y: isize| {
            x >= 0 && y >= 0 && x < map[0].len() as isize && y < map.len() as isize
        };

        while in_bounds(x, y) {
            visited.insert((x, y));

            let (nx, ny) = dir.next(x, y);
            if in_bounds(nx, ny) && map[ny as usize][nx as usize] == Tile::Obstruction {
                dir = dir.rotate();
            } else {
                x = nx;
                y = ny;
            }
        }

        println!("Part 1: {}", visited.len());
    }

    let mut part2: u64 = 0;

    let in_bounds = |x: isize, y: isize| {
        x >= 0 && y >= 0 && x < map[0].len() as isize && y < map.len() as isize
    };

    for new_obs_y in 0..map.len() {
        for new_obs_x in 0..map[0].len() {
            let (mut x, mut y) = (x, y);

            let mut been_here = BTreeSet::<(isize, isize, Dir)>::new();
            let mut dir = Dir::Up;

            while in_bounds(x, y) {
                if been_here.contains(&(x, y, dir)) {
                    part2 += 1;
                    break;
                }
                been_here.insert((x, y, dir));

                let (nx, ny) = dir.next(x, y);

                if in_bounds(nx, ny) && map[ny as usize][nx as usize] == Tile::Obstruction
                    || (nx, ny) == (new_obs_x as isize, new_obs_y as isize)
                {
                    dir = dir.rotate();
                } else {
                    x = nx;
                    y = ny;
                }
            }
        }
    }

    println!("Part 2: {}", part2);
}
