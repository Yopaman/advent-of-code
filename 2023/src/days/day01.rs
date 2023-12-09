use crate::problem::Problem;

pub struct DayOne {}

impl Problem for DayOne {
    fn part_one(&self, input: String) {
        let lines = input.split("\n");
        let mut sum: i32 = 0;
        for line in lines {
            let mut nbr: String = String::from("");
            for c in line.chars() {
                if c.is_numeric() {
                    nbr.push(c);
                    break;
                }
            }

            for c in line.chars().rev() {
                if c.is_numeric() {
                    nbr.push(c);
                    break;
                }
            }
            if nbr != "" {
                sum += nbr.parse::<i32>().unwrap();
            }
        }
        println!("{}", sum);
    }

    fn part_two(&self, input: String) {
        let lines = input.split("\n");
        let mut sum: i32 = 0;
        for line in lines {
            let mut number = String::from("");
            if let Some(left_nbr) = get_left_nbr(line.to_string()) {
                number.push(char::from_digit(left_nbr.1 as u32, 10).unwrap());
            }
            if let Some(right_nbr) = get_right_nbr(line.to_string()) {
                number.push(char::from_digit(right_nbr.1 as u32, 10).unwrap());
            }
            if number.len() > 1 {
                sum += number.parse::<i32>().unwrap();
            }
        }
        println!("{}", sum);
       
    }
}

fn get_left_nbr(str: String) -> Option<(i32, i32)> {
    let numbers_lit = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut digits: Vec<(i32, i32)> = vec![];
    for i in 0..numbers_lit.len() {
       if let Some(j) = str.find(numbers_lit[i]) {
            digits.push(((j as i32), (i + 1) as i32));
       }
    }
    for i in 0..str.len() {
        if str.chars().nth(i).unwrap().is_numeric() {
            digits.push((i as i32, str.chars().nth(i).unwrap().to_digit(10).unwrap() as i32))
        }
    }
    digits.into_iter().min()
}

fn get_right_nbr(str: String) -> Option<(i32, i32)> {
    let numbers_lit = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut digits: Vec<(i32, i32)> = vec![];
    for i in 0..numbers_lit.len() {
       if let Some(j) = str.rfind(numbers_lit[i]) {
            digits.push(((j as i32), (i + 1) as i32));
       }
    }
    for i in (0..str.len()).rev() {
        if str.chars().nth(i).unwrap().is_numeric() {
            digits.push((i as i32, str.chars().nth(i).unwrap().to_digit(10).unwrap() as i32))
        }
    }
    digits.into_iter().max()
}