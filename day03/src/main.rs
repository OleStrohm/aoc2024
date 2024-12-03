fn main() {
    //let mut input = include_str!("example.txt");
    let mut input = include_str!("input.txt");

    let mut part1 = 0;
    let mut part2 = 0;

    let mut enabled = true;

    while let Some(mul) = input.find("mul(") {
        let Some(comma) = input.find(',') else {
            break;
        };
        let Some(parenthesis) = input.find(')') else {
            break;
        };
        let disable = input.find("don't()").unwrap_or(usize::MAX);
        let enable = input.find("do()").unwrap_or(usize::MAX);

        let current = &input[mul.min(comma).min(parenthesis).min(disable).min(enable)..];
        let next = &current[1..];

        'a: {
            if current.starts_with("do()") {
                enabled = true;
            } else if current.starts_with("don't()") {
                enabled = false;
            } else if current.starts_with("mul(") && comma < parenthesis {
                let Ok(left) = input[mul + 4..comma].parse::<u64>() else {
                    break 'a;
                };
                let Ok(right) = input[comma + 1..parenthesis].parse::<u64>() else {
                    break 'a;
                };

                part1 += left * right;
                if enabled {
                    part2 += left * right;
                }
            }
        }
        input = next;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
