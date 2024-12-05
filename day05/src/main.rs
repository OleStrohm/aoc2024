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

    fn order_between(orders: &[(u64, u64)], left: u64, right: u64) -> Option<Ordering> {
        if orders.contains(&(left, right)) {
            return Some(Ordering::Less);
        }
        if orders.contains(&(right, left)) {
            return Some(Ordering::Greater);
        }

        for &(order_left, order_right) in orders {
            if left == order_left {
                if let Some(order) = order_between(orders, order_right, right) {
                    return Some(order);
                }
            }

            if right == order_right {
                if let Some(order) = order_between(orders, left, order_left) {
                    return Some(order);
                }
            }
        }

        None
    }

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
            update.sort_by(|&left, &right| order_between(&orders, left, right).unwrap());
            update
        })
        .map(|update| update[update.len() / 2])
        .sum::<u64>();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
