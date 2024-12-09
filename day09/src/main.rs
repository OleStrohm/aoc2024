use itertools::Itertools;

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let numbers = input
        .trim()
        .chars()
        .map(|n| n.to_digit(10).unwrap())
        .collect_vec();

    let mut blocks = vec![];

    let mut is_file = true;
    let mut file_id = 0;

    for &number in &numbers {
        if is_file {
            for _ in 0..number {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..number {
                blocks.push(None);
            }
        }

        is_file = !is_file;
    }

    for n in &blocks {
        if let Some(n) = n {
            print!("{n}");
        } else {
            print!(".");
        }
    }
    println!();

    {
        let mut blocks = blocks.clone();
        let mut next_read = blocks.len() - 1;
        let mut next_write = 0;
        while next_write < next_read {
            if blocks[next_write].is_none() {
                blocks[next_write] = blocks[next_read].take();

                while blocks[next_read].is_none() {
                    next_read -= 1;
                }
            }

            next_write += 1;
        }

        for n in &blocks {
            if let Some(n) = n {
                print!("{n}");
            } else {
                print!(".");
            }
        }
        println!();

        let part1 = blocks
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| Some(i * n?))
            .sum::<usize>();

        println!("Part 1: {part1}");
    }

    {
        let mut blocks = blocks.clone();

        for id in (0..=numbers.len() / 2).rev() {
            let len = numbers[2 * id] as usize;
            let first_of_id = blocks
                .iter()
                .enumerate()
                .find(|&(_, &ident)| ident == Some(id))
                .unwrap()
                .0;
            if let Some(free_start) = blocks[..first_of_id]
                .windows(len)
                .find_position(|m| m.iter().all(|e| e.is_none()))
                .map(|(free_start, _)| free_start)
            {
                for b in &mut blocks {
                    if *b == Some(id) {
                        *b = None;
                    }
                }

                for i in 0..len {
                    blocks[free_start + i] = Some(id);
                }
            }
        }

        for n in &blocks {
            if let Some(n) = n {
                print!("{n}");
            } else {
                print!(".");
            }
        }
        println!();

        let part2 = blocks
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| Some(i * n?))
            .sum::<usize>();

        println!("Part 2: {part2}");
    }
}
