use std::cmp::Ordering;

use itertools::Itertools;

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let (orders, updates) = input.split("\n\n").collect_tuple().unwrap();
    let orders: Vec<(_, _)> = orders
        .lines()
        .map(|l| {
            l.split('|')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let updates = updates
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let part1 = updates
        .iter()
        .filter(|update| {
            update.iter().enumerate().all(|(i, &page)| {
                orders
                    .iter()
                    .all(|&(first, second)| first != page || !update[..i].contains(&second))
            })
        })
        .map(|update| update[update.len() / 2])
        .sum::<u64>();

    let part2 = updates
        .iter()
        .filter(|update| {
            !update.iter().enumerate().all(|(i, &page)| {
                orders
                    .iter()
                    .all(|&(first, second)| first != page || !update[..i].contains(&second))
            })
        })
        .map(|update| {
            let mut update = update.clone();
            update.sort_by(|&left, &right| {
                if orders.contains(&(left, right)) {
                    Ordering::Less
                } else if orders.contains(&(right, left)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum::<u64>();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
