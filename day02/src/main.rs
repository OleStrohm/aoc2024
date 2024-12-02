#![feature(array_windows)]
#![feature(iter_chain)]

use std::iter::chain;

use itertools::Itertools;

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let reports = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|n| n.parse::<u64>().ok())
                .collect_vec()
        })
        .collect_vec();

    let part1 = reports
        .iter()
        .filter(|r| {
            let r = r.array_windows::<2>();
            r.map(|[l, r]| l.partial_cmp(r)).all_equal()
                && r.clone().all(|&[l, r]| (1..=3).contains(&l.abs_diff(r)))
        })
        .count();

    println!("Part 1: {part1}");

    let part2 = reports
        .into_iter()
        .filter(|r| {
            (0..r.len()).any(|remove| {
                let r = chain(&r[..remove], &r[remove + 1..]).tuple_windows();
                r.clone().map(|(l, r)| l.partial_cmp(r)).all_equal()
                    && r.clone().all(|(l, &r)| (1..=3).contains(&l.abs_diff(r)))
            })
        })
        .count();

    println!("Part 2: {part2}");
}
