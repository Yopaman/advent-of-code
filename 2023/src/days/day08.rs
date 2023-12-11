use std::collections::HashMap;

use crate::problem::Problem;

pub struct DayEight {}

fn parse(inputs: String) -> (String, HashMap<String, (String, String)>) {
    let mut blocks = inputs.split("\n\n");

    let instructions = blocks.next().unwrap().to_string();

    let mut maps: HashMap<String, (String, String)> = HashMap::new();
    for line in blocks.next().unwrap().trim().split('\n') {
        let mut line_split = line.split(" = ");
        let entry = line_split.next().unwrap().to_string();

        let nodes_string = line_split.next().unwrap();
        let left = &nodes_string[1..4].to_string();
        let right = &nodes_string[6..9].to_string();
        maps.insert(entry, (left.to_string(), right.to_string()));
    }
    (instructions, maps)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn get_number_of_jumps(
    instructions: String,
    map: &HashMap<String, (String, String)>,
    mut word: String,
) -> usize {
    let mut count = 0;
    while !word.ends_with('Z') {
        for c in instructions.chars() {
            match c {
                'L' => {
                    let possible_words = map.get(&word).unwrap();
                    word = possible_words.0.clone();
                }
                'R' => {
                    let possible_words = map.get(&word).unwrap();
                    word = possible_words.1.clone();
                }
                _ => {}
            }
            count += 1;
            if word.ends_with('Z') {
                break;
            }
        }
    }
    count
}

impl Problem for DayEight {
    fn part_one(&self, input: String) {
        let (instructions, maps) = parse(input);
        let mut count = 0;
        let mut word = String::from("AAA");
        while word != *"ZZZ".to_string() {
            for c in instructions.chars() {
                match c {
                    'L' => {
                        let possible_words = maps.get(&word).unwrap();
                        word = possible_words.0.clone();
                    }
                    'R' => {
                        let possible_words = maps.get(&word).unwrap();
                        word = possible_words.1.clone();
                    }
                    _ => {}
                }
                count += 1;
                if word == *"ZZZ".to_string() {
                    break;
                }
            }
        }
        println!("{}", count);
    }

    fn part_two(&self, input: String) {
        let (instructions, maps) = parse(input);
        let starters: Vec<&String> = maps.keys().filter(|s| s.ends_with('A')).collect();
        let numbers: Vec<usize> = starters
            .into_iter()
            .map(|element| get_number_of_jumps(instructions.clone(), &maps, element.to_string()))
            .collect();
        println!(
            "{:?}",
            numbers
                .into_iter()
                .reduce(|a, b| (a * b) / gcd(a, b))
                .unwrap()
        );
    }
}