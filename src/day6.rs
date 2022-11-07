use crate::inputs;

fn parse_inputs() -> Vec<i32> {
    let lines = inputs::read_lines("./src/inputs/day6.txt");
    lines[0]
        .split(',')
        .into_iter()
        .map(|e| e.parse::<i32>().unwrap())
        .collect()
}

fn simulate(input: Vec<i32>) -> Vec<i32> {
    let mut adds: Vec<i32> = vec![];

    for e in &input {
        if *e == 0 {
            adds.push(8);
        }
    }

    let mut numbers: Vec<i32> = input
        .into_iter()
        .map(|e| if e > 0 { e - 1 } else { 6 })
        .collect();

    numbers.append(&mut adds);

    numbers
}

fn simulate_fast(input: Vec<usize>, total_days: usize) -> u64 {
    let mut counts: Vec<u64> = vec![0; total_days + 9];
    for e in &input {
        counts[*e] += 1;
    }

    let mut count = input.len() as u64;

    for i in 0..total_days {
        let new_fish = counts[i];
        counts[i + 6 + 1] += new_fish;
        counts[i + 8 + 1] += new_fish;
        count += new_fish;
    }
    count
}

pub fn part1() {
    let mut days = parse_inputs();

    for _ in 0..80 {
        days = simulate(days);
    }

    println!("result : {}", days.len());
}

pub fn part2() {
    let days = parse_inputs();

    println!(
        "result : {}",
        simulate_fast(days.into_iter().map(|d| d as usize).collect(), 256)
    );
}

