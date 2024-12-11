use std::collections::BTreeMap;

use itertools::{Either, Itertools};

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let stones = input
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect_vec();

    let part1 = (0..25)
        .fold(stones.clone(), |stones, _| {
            stones
                .into_iter()
                .flat_map(|n| {
                    let log10 = (n as f32).log10() as u32;
                    let pow10 = 10_u64.pow(log10 / 2 + 1);
                    match n {
                        0 => Either::Left([1]),
                        n if log10 % 2 == 1 => Either::Right([n / pow10, n % pow10]),
                        n => Either::Left([n * 2024]),
                    }
                    .into_iter()
                })
                .collect_vec()
        })
        .len();

    println!("Part 1: {part1}");

    let stones = stones
        .into_iter()
        .map(|n| (n, 1))
        .collect::<BTreeMap<u64, u64>>();

    let part2 = (0..75)
        .fold(stones, |stones, _| {
            stones
                .into_iter()
                .flat_map(|(n, count)| {
                    let log10 = (n as f32).log10() as u32;
                    let pow10 = 10_u64.pow(log10 / 2 + 1);
                    match n {
                        0 => Either::Left([(1, count)]),
                        n if log10 % 2 == 1 => {
                            Either::Right([(n / pow10, count), (n % pow10, count)])
                        }
                        n => Either::Left([(n * 2024, count)]),
                    }
                    .into_iter()
                })
                .sorted_by_key(|&(n, _)| n)
                .chunk_by(|&(n, _)| n)
                .into_iter()
                .map(|(key, chunk)| (key, chunk.map(|(_, count)| count).sum()))
                .collect()
        })
        .into_values()
        .sum::<u64>();

    println!("Part 2: {part2}");
}
