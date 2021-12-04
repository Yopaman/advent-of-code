use crate::inputs;

pub fn run() {
    let lines = inputs::read_lines("./src/inputs/1_1.txt");
    let mut sum: u32 = 0;
    lines.windows(2)
    .inspect(|w| {
        if w[0].parse::<i32>().unwrap() < w[1].parse::<i32>().unwrap() {
            sum += 1;
        }
    }).for_each(drop);
    println!("{}", sum);

}


