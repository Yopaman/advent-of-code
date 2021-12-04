use crate::inputs;

pub fn run() {
    let lines = inputs::read_lines("./src/inputs/1_1.txt");
    let mut sum: u32 = 0;
    lines.iter()
    .map(|w| w.parse::<i16>().unwrap())
    .collect::<Vec<i16>>()
    .windows(4)
    .inspect(|w| {
        if w[..3].iter().sum::<i16>() < w[1..].iter().sum::<i16>() {
            sum += 1;
        }
    }).for_each(drop);
    println!("{}", sum);

}