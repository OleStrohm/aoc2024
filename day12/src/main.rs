use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let region_ids = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| x + y * row.len())
                .collect_vec()
        })
        .collect_vec();

    let mut equivalences = BTreeMap::new();

    for y in 0..region_ids.len() {
        for x in 0..region_ids[0].len() {
            let current = map[y][x];
            let up = y.checked_sub(1).map(|y| map[y][x]);
            let left = x.checked_sub(1).map(|x| map[y][x]);
            if Some((current, current)) == up.zip(left) {
                let mut up = region_ids[y - 1][x].min(region_ids[y][x - 1]);
                let mut left = region_ids[y - 1][x].max(region_ids[y][x - 1]);

                while let Some(&new_up) = equivalences.get(&up) {
                    up = new_up;
                }
                while let Some(&new_left) = equivalences.get(&left) {
                    left = new_left;
                }
                let min = left.min(up);
                let max = left.max(up);

                equivalences.insert(region_ids[y][x], min);
                if min != max {
                    equivalences.insert(max, min);
                }
            } else if Some(current) == left {
                equivalences.insert(region_ids[y][x], region_ids[y][x - 1]);
            } else if Some(current) == up {
                equivalences.insert(region_ids[y][x], region_ids[y - 1][x]);
            }
        }
    }

    //dbg!(&equivalences);

    let region_map = (0..region_ids.len())
        .map(|y| {
            (0..region_ids[0].len())
                .map(|x| {
                    let mut id = region_ids[y][x];
                    while let Some(&new_id) = equivalences.get(&id) {
                        id = new_id;
                    }
                    id
                })
                .collect_vec()
        })
        .collect_vec();

    let area = region_map.iter().flatten().counts();

    let mut perimeter = HashMap::<usize, usize>::new();

    for row in &region_map {
        for (l, r) in [usize::MAX]
            .iter()
            .chain(row)
            .chain([&usize::MAX])
            .tuple_windows()
        {
            if l != r {
                *perimeter.entry(*l).or_default() += 1;
                *perimeter.entry(*r).or_default() += 1;
            }
        }
    }
    for x in 0..region_map[0].len() {
        for (l, r) in [usize::MAX]
            .into_iter()
            .chain((0..region_map.len()).map(|y| region_map[y][x]))
            .chain([usize::MAX])
            .tuple_windows()
        {
            if l != r {
                *perimeter.entry(l).or_default() += 1;
                *perimeter.entry(r).or_default() += 1;
            }
        }
    }

    let part1 = area
        .iter()
        .map(|(id, area)| area * perimeter[id])
        .sum::<usize>();

    println!("Part 1: {part1}");

    let mut sides = HashMap::<usize, usize>::new();

    for &&id in area.keys() {
        for y in 0..region_map.len() {
            *sides.entry(id).or_default() += (0..region_map[0].len())
                .map(|x| {
                    region_map[y][x] == id
                        && region_map
                            .get(y + 1)
                            .and_then(|row| row.get(x))
                            .is_none_or(|&v| v != id)
                })
                .dedup()
                .filter(|&wall| wall)
                .count();
            *sides.entry(id).or_default() += (0..region_map[0].len())
                .map(|x| {
                    region_map[y][x] == id
                        && y.checked_sub(1)
                            .and_then(|y| region_map.get(y))
                            .and_then(|row| row.get(x))
                            .is_none_or(|&v| v != id)
                })
                .dedup()
                .filter(|&wall| wall)
                .count();
        }
        for x in 0..region_map[0].len() {
            *sides.entry(id).or_default() += (0..region_map.len())
                .map(|y| {
                    region_map[y][x] == id && region_map[y].get(x + 1).is_none_or(|&v| v != id)
                })
                .dedup()
                .filter(|&wall| wall)
                .count();
            *sides.entry(id).or_default() += (0..region_map.len())
                .map(|y| {
                    region_map[y][x] == id
                        && x.checked_sub(1)
                            .and_then(|x| region_map[y].get(x))
                            .is_none_or(|&v| v != id)
                })
                .dedup()
                .filter(|&wall| wall)
                .count();
        }
    }
    
    let part2 = area
        .iter()
        .map(|(id, area)| area * sides[id])
        .sum::<usize>();

    println!("Part 2: {part2}");
}
