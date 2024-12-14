use std::time::Duration;

use itertools::Itertools;
use num::integer::Roots;

fn main() {
    let input = include_str!("input.txt");
    let width = 101;
    let height = 103;
    //let input = include_str!("example.txt");
    //let width = 11;
    //let height = 7;

    let robots = input
        .split(&['=', ',', ' ', '\n'])
        .filter_map(|n| n.parse::<i64>().ok())
        .tuples()
        .collect_vec();

    let (tl, tr, bl, br) = robots
        .iter()
        .map(|(x, y, vx, vy)| {
            (
                (x + 100 * vx).rem_euclid(width),
                (y + 100 * vy).rem_euclid(height),
            )
        })
        .counts()
        .into_iter()
        .fold((0, 0, 0, 0), |(tl, tr, bl, br), ((x, y), count)| {
            if x < width / 2 && y < height / 2 {
                (tl + count, tr, bl, br)
            } else if x > width / 2 && y < height / 2 {
                (tl, tr + count, bl, br)
            } else if x < width / 2 && y > height / 2 {
                (tl, tr, bl + count, br)
            } else if x > width / 2 && y > height / 2 {
                (tl, tr, bl, br + count)
            } else {
                (tl, tr, bl, br)
            }
        });

    let part1 = tl * tr * bl * br;
    println!("Part 1: {part1}");

    let mut min_unsymmetry = usize::MAX;
    for i in 0..width * height {
        let positions = robots
            .iter()
            .map(|&(x, y, vx, vy)| {
                (
                    (x + i * vx).rem_euclid(width),
                    (y + i * vy).rem_euclid(height),
                )
            })
            .counts();

        let average_x = positions.keys().map(|(x, _)| x).sum::<i64>() / positions.len() as i64;

        let unsymmetry_amount = positions
            .keys()
            .filter(|&&(x, y)| !positions.contains_key(&((2 * average_x - x).rem_euclid(width), y)))
            .count();

        min_unsymmetry = min_unsymmetry.min(unsymmetry_amount);
        if unsymmetry_amount == 386 {
            let mut output_str = String::new();
            for y in 0..height {
                for x in 0..width {
                    if let Some(&count) = positions.get(&(x, y)) {
                        output_str += &format!("{}", count.min(9));
                    } else {
                        output_str += ".";
                    }
                }
                output_str += "\n";
            }

            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{output_str}");
            println!("iteration: {i}");
            println!("average x: {average_x}");
        }
    }
    println!("{min_unsymmetry}");
}
