use std::iter::zip;

use itertools::Itertools;

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|l| {
            l.split_whitespace()
                .filter_map(|n| n.parse::<u64>().ok())
                .collect_tuple()
        })
        .unzip();

    left.sort();
    right.sort();

    let part1 = zip(left.clone(), right.clone())
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u64>();
    println!("Part 1: {part1}");

    let part2 = left
        .into_iter()
        .map(|l| l * right.iter().filter(|&&r| l == r).count() as u64)
        .sum::<u64>();

    println!("Part 2: {part2}");
}
