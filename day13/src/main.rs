use itertools::Itertools;

fn main() {
    //let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let part1 = input
        .split("\n\n")
        .map(|machine| {
            let (ax, ay, bx, by, tx, ty) = machine
                .split(&['+', ',', '\n', '='])
                .filter_map(|n| n.parse::<i64>().ok())
                .collect_tuple()
                .unwrap();

            let b = (tx * ay - ty * ax) / (bx * ay - by * ax);
            let tx = tx - bx * b;
            let ty = ty - by * b;
            if tx % ax == 0 && ty % ay == 0 && tx / ax == ty / ay {
                let a = tx / ax;
                3 * a + b
            } else {
                0
            }
        })
        .sum::<i64>();

    println!("Part 1: {part1}");

    let part2 = input
        .split("\n\n")
        .map(|machine| {
            let (ax, ay, bx, by, tx, ty) = machine
                .split(&['+', ',', '\n', '='])
                .filter_map(|n| n.parse::<i64>().ok())
                .collect_tuple()
                .unwrap();
            let tx = tx + 10000000000000;
            let ty = ty + 10000000000000;

            let b = (tx * ay - ty * ax) / (bx * ay - by * ax);
            let tx = tx - bx * b;
            let ty = ty - by * b;
            if tx % ax == 0 && ty % ay == 0 && tx / ax == ty / ay {
                let a = tx / ax;
                3 * a + b
            } else {
                0
            }
        })
        .sum::<i64>();

    println!("Part 2: {part2}");
}
