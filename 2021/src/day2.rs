use crate::inputs;


pub fn part1() {
    let lines = inputs::read_lines("./src/inputs/day2.txt");
    let mut depth: i32 = 0;
    let mut hor: i32 = 0;
    for l in lines {
        let split: Vec<&str> = l[..].split(' ').collect();
        let value: i32 = split[1].parse().unwrap();
        match split[0] {
            "up" => depth -= value,
            "down" => depth += value,
            "forward" => hor += value,
            _ => panic!("Error with input")
        }
    }
    println!("{}", hor * depth);
}


pub fn part2() {
    let lines = inputs::read_lines("./src/inputs/day2.txt");
    let mut depth: i32 = 0;
    let mut hor: i32 = 0;
    let mut aim: i32 = 0;
    for l in lines {
        let split: Vec<&str> = l[..].split(' ').collect();
        let value: i32 = split[1].parse().unwrap();
        match split[0] {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => {
                hor += value;
                depth += aim*value;
            }
            _ => panic!("Error with input")
        }
    }
    println!("{}", hor * depth);
}