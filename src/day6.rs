use crate::inputs;

fn parse_inputs() -> Vec<i32> {
    let lines = inputs::read_lines("./src/inputs/day6.txt");
    lines[0].split(",").into_iter().map(|e| e.parse::<i32>().unwrap()).collect()   
}

fn simulate(input: Vec<i32>) -> Vec<i32> {
    let mut adds: Vec<i32> = vec!();
    let mut numbers: Vec<i32> = vec!();

    for e in &input {
        if *e == 0 {adds.push(8);}
    }

    numbers = input.into_iter().map(|e| {
        if e > 0 {e - 1}
        else {6}
    }).collect();

    numbers.append(&mut adds);

    numbers
}

pub fn part1() {
    let mut days = parse_inputs();
    
    for _ in 0..80 {
        days = simulate(days);
    }

    println!("result : {}", days.len());
}