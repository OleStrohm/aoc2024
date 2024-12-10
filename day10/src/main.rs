use std::collections::BTreeSet;

use itertools::Itertools;

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let map = input
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let width = map[0].len();
    let height = map.len();

    let start = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, height)| (x, y, height))
        })
        .filter(|&(.., &height)| height == 0)
        .map(|(x, y, _)| (x, y))
        .collect_vec();

    let part1 = start
        .iter()
        .map(|start| {
            let mut seen = BTreeSet::<(usize, usize)>::new();
            let mut next_steps = BTreeSet::from_iter([*start]);
            while let Some(next) = next_steps.pop_first() {
                if seen.contains(&next) {
                    continue;
                }
                seen.insert(next);

                let current_height = map[next.1][next.0];

                next_steps.extend(
                    [
                        (next.0.saturating_sub(1), next.1),
                        (next.0, next.1 + 1),
                        (next.0 + 1, next.1),
                        (next.0, next.1.saturating_sub(1)),
                    ]
                    .into_iter()
                    .filter(|&(x, y)| x < width && y < height && map[y][x] == current_height + 1),
                );
            }

            seen.into_iter()
                .map(|(x, y)| map[y][x])
                .filter(|&height| height == 9)
                .count()
        })
        .sum::<usize>();

    println!("Part 1: {part1}");

    let mut trails_going_through = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|&height| if height == 9 { 1 } else { 0 })
                .collect_vec()
        })
        .collect_vec();

    for h in (0..9).rev() {
        for y in 0..height {
            for x in 0..width {
                if map[y][x] == h {
                    for (next_x, next_y) in [
                        (x.saturating_sub(1), y),
                        (x, y + 1),
                        (x + 1, y),
                        (x, y.saturating_sub(1)),
                    ]
                    .into_iter()
                    .filter(|&(x, y)| x < width && y < height && map[y][x] == h + 1)
                    {
                        trails_going_through[y][x] += trails_going_through[next_y][next_x];
                    }
                }
            }
        }
    }

    let map = &map;

    let part2 = trails_going_through
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, rating)| (map[y][x], rating))
        })
        .filter(|&(height, _)| height == 0)
        .map(|(_, rating)| rating)
        .sum::<u32>();

    println!("Part 2: {part2}");
}
