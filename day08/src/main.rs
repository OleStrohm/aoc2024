use itertools::Itertools;
use num::integer::gcd;

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let width = map[0].len();
    let height = map.len();

    let mut part1_antinodes = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            let freq = map[y][x];
            if freq == '.' {
                continue;
            }

            for (ay, row) in map.iter().enumerate() {
                for (ax, &other_freq) in row.iter().enumerate() {
                    if other_freq == freq && (ax, ay) != (x, y) {
                        let Some(antinode_x) = (2 * x).checked_sub(ax) else {
                            continue;
                        };
                        let Some(antinode_y) = (2 * y).checked_sub(ay) else {
                            continue;
                        };
                        if antinode_x < width && antinode_y < height {
                            part1_antinodes[antinode_y][antinode_x] = true;
                        }
                    }
                }
            }
        }
    }

    let part1 = part1_antinodes
        .into_iter()
        .flatten()
        .filter(|&is_antinode| is_antinode)
        .count();

    println!("Part 1: {part1}");

    let mut part2_antinodes = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            let freq = map[y][x];
            if freq == '.' {
                continue;
            }

            for (ay, row) in map.iter().enumerate() {
                for (ax, &other_freq) in row.iter().enumerate() {
                    if other_freq == freq && (ax, ay) != (x, y) {
                        let x_diff = (x as isize) - (ax as isize);
                        let y_diff = (y as isize) - (ay as isize);
                        let scale_factor = gcd(x_diff, y_diff);
                        let x_diff = x_diff / scale_factor;
                        let y_diff = y_diff / scale_factor;

                        let mut test_x = x as isize;
                        let mut test_y = y as isize;

                        while test_x >= 0
                            && test_y >= 0
                            && test_x < width as isize
                            && test_y < height as isize
                        {
                            part2_antinodes[test_y as usize][test_x as usize] = true;
                            test_x += x_diff;
                            test_y += y_diff;
                        }

                        let mut test_x = x as isize;
                        let mut test_y = y as isize;

                        while test_x >= 0
                            && test_y >= 0
                            && test_x < width as isize
                            && test_y < height as isize
                        {
                            part2_antinodes[test_y as usize][test_x as usize] = true;
                            test_x -= x_diff;
                            test_y -= y_diff;
                        }
                    }
                }
            }
        }
    }

    let part2 = part2_antinodes
        .into_iter()
        .flatten()
        .filter(|&is_antinode| is_antinode)
        .count();

    println!("Part 2: {part2}");
}
