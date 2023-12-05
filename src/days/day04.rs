use std::collections::HashMap;

use crate::problem::Problem;

pub struct DayFour {}

pub fn parse(line: String) -> (Vec<u32>, Vec<u32>) {
    let first_split = line
        .chars()
        .skip_while(|&c| c != ':')
        .skip(2)
        .collect::<String>();
    let mut second_split = first_split.split('|').map(|part| {
        part.split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });
    (second_split.next().unwrap(), second_split.next().unwrap())
}

fn get_matching(winning_nbrs: Vec<u32>, nbrs: Vec<u32>) -> u32 {
    let mut score = 0;
    for n in winning_nbrs {
        if nbrs.contains(&n) {
            score += 1
        }
    }
    score
}

impl Problem for DayFour {
    fn part_one(&self, input: String) {
        let sum: u32 = input
            .split('\n')
            .map(|line| {
                if line.is_empty() {
                    return 0;
                }
                let (winning_nbrs, nbrs) = parse(line.to_string());
                let mut score = 0;
                for n in winning_nbrs {
                    if nbrs.contains(&n) {
                        if score == 0 {
                            score = 1;
                        } else {
                            score *= 2;
                        }
                    }
                }
                score
            })
            .sum();
        println!("{}", sum);
    }

    fn part_two(&self, input: String) {
        let mut instance: HashMap<usize, u32> = HashMap::new();
        let lines = input.split('\n');
        let length = lines.clone().count() - 1;
        for (i, line) in lines.enumerate() {
            if line.is_empty() {
                continue;
            }
            // Parse line
            let (w_nbr, nbr) = parse(line.to_string());
            // Get number of copies you have
            let mut copies = 1;
            match instance.get(&i) {
                Some(n) => {
                    copies += n;
                    instance.insert(i, n + 1);
                }
                None => {
                    instance.insert(i, 1);
                }
            }
            // Get number of matching numbers
            let matchs = get_matching(w_nbr, nbr);
            // Add copies for the following games
            if i + matchs as usize > length {
                for k in (i + 1)..length {
                    match instance.get(&k) {
                        Some(n) => {
                            instance.insert(k, n + copies);
                        }
                        None => {
                            instance.insert(k, copies);
                        }
                    }
                }
            } else {
                for k in (i + 1)..=(i + matchs as usize) {
                    match instance.get(&k) {
                        Some(n) => {
                            instance.insert(k, n + copies);
                        }
                        None => {
                            instance.insert(k, copies);
                        }
                    }
                }
            }
        }
        println!("{:?}", instance.values().sum::<u32>());
    }
}
