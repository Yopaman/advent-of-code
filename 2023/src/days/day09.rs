use crate::problem::Problem;

pub struct DayNine {}

fn get_answers(line: Vec<i64>) -> i64 {
    match line.last().unwrap() {
        _ if line.iter().all(|n| *n == 0) => 0,
        n => n + get_answers(line.iter().zip(line.iter().skip(1)).map(|(a, b)| b - a).collect::<Vec<i64>>()),
        
    }
}

impl Problem for DayNine {
    fn part_one(&self, input: String) {
        let lines = input.trim().split('\n');
        let numbers: Vec<i64> = lines.map(|line| {
            get_answers(line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        }).collect();
        println!("{:?}", numbers.into_iter().sum::<i64>()); 
    }

    fn part_two(&self, input: String) {
        let lines = input.trim().split('\n');
        let numbers: Vec<i64> = lines.map(|line| {
            get_answers(line.split(' ').map(|n| n.parse::<i64>().unwrap()).rev().collect::<Vec<i64>>())
        }).collect();
        println!("{:?}", numbers.into_iter().sum::<i64>()); 
    }
}