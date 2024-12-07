use itertools::Itertools;

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let equations = input
        .lines()
        .map(|l| {
            let mut numbers = l.split(&[':', ' ']).filter_map(|n| n.parse::<u64>().ok());
            let test_value = numbers.next().unwrap();
            let numbers = numbers.collect_vec();
            (test_value, numbers)
        })
        .collect_vec();

    fn can_make_1(result: u64, accum: u64, numbers: &[u64]) -> bool {
        let Some((current, rest)) = numbers.split_first() else {
            return accum == result;
        };

        can_make_1(result, accum + current, rest) || can_make_1(result, accum * current, rest)
    }

    let part1 = equations
        .iter()
        .filter(|&(test_value, numbers)| can_make_1(*test_value, 0, numbers))
        .map(|(test_value, _)| test_value)
        .sum::<u64>();

    println!("Part 1: {part1}");

    let part2 = equations
        .iter()
        .filter(|&(test_value, numbers)| {
            for mut i in 0..3_u64.pow(numbers.len() as u32 - 1) {
                let mut result = numbers[0];
                let mut rest = &numbers[1..];
                while let Some(&next) = rest.first() {
                    result = match i % 3 {
                        0 => result + next,
                        1 => result * next,
                        _ => result * 10_u64.pow(1 + (next as f32).log10() as u32) + next,
                    };
                    if result > *test_value {
                        break;
                    }
                    rest = &rest[1..];
                    i /= 3;
                }

                if *test_value == result {
                    return true;
                }
            }

            false
        })
        .map(|(test_value, _)| test_value)
        .sum::<u64>();

    println!("Part 2: {part2}");
}
