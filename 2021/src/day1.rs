use crate::inputs;

pub fn part1() {
    let lines = inputs::read_lines("./src/inputs/day1.txt");
    let mut sum: u32 = 0;
    Iterator::inspect(lines.windows(2), |w| {
        if w[0].parse::<i32>().unwrap() < w[1].parse::<i32>().unwrap() {
            sum += 1;
        }
    })
    .for_each(drop);
    println!("{}", sum);
}

pub fn part2() {
    let lines = inputs::read_lines("./src/inputs/day1.txt");
    let mut sum: u32 = 0;
    Iterator::for_each(
        Iterator::inspect(
            lines
                .iter()
                .map(|w| w.parse::<i16>().unwrap())
                .collect::<Vec<i16>>()
                .windows(4),
            |w| {
                if w[..3].iter().sum::<i16>() < w[1..].iter().sum::<i16>() {
                    sum += 1;
                }
            },
        ),
        drop,
    );
    println!("{}", sum);
}

