use std::collections::HashMap;

use crate::problem::Problem;

pub struct DayThree {}

#[derive(Debug, Clone)]
struct Number {
    n: usize,
    x1: usize,
    x2: usize,
    y: usize,
}

fn parse(input: String) -> (HashMap<(usize, usize), char>, Vec<Number>) {
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();
    let mut numbers: Vec<Number> = vec![];
    let lines = input.split('\n');
    for (y, line) in lines.enumerate() {
        let mut number: String = String::new();
        let mut x1 = 0;
        let mut x2 = 0;
        let mut line_numbers = vec![];
        let _ = line
            .chars()
            .enumerate()
            .fold(false, |was_nbr, (x, c)| match c {
                '.' => {
                    if was_nbr {
                        line_numbers.push(Number {
                            n: number.parse::<usize>().unwrap(),
                            x1,
                            x2,
                            y,
                        });
                        number = "".to_string();
                    }
                    false
                }
                '0'..='9' => {
                    if !was_nbr {
                        x1 = x;
                    }
                    x2 = x;
                    number.push(c);
                    true
                }
                _ => {
                    symbols.insert((x, y), c);
                    if was_nbr {
                        line_numbers.push(Number {
                            n: number.parse::<usize>().unwrap(),
                            x1,
                            x2,
                            y,
                        });
                        number = "".to_string();
                    }
                    false
                }
            });
        if !number.is_empty() {
            line_numbers.push(Number {
                n: number.parse::<usize>().unwrap(),
                x1,
                x2,
                y,
            });
        }
        numbers.append(&mut line_numbers);
    }
    (symbols, numbers)
}

fn check_adjacency(number: &Number, symbols: &HashMap<(usize, usize), char>) -> bool {
    // x x x
    // . . .
    // . . .
    if number.y > 0 {
        for x in number.x1..=(number.x2 + 1) {
            if symbols.contains_key(&(x, number.y - 1)) {
                return true;
            }
        }
        if number.x1 > 0 && symbols.contains_key(&(number.x1 - 1, number.y - 1)) {
            return true;
        }
    }

    // . . .
    // x . .
    // x . .
    if number.x1 > 0
        && (symbols.contains_key(&(number.x1 - 1, number.y))
            || symbols.contains_key(&(number.x1 - 1, number.y + 1)))
    {
        return true;
    }

    // . . .
    // . . x
    // . . .
    if symbols.contains_key(&(number.x2 + 1, number.y)) {
        return true;
    }

    // . . .
    // . . .
    // . x x
    for x in number.x1..=(number.x2 + 1) {
        if symbols.contains_key(&(x, number.y + 1)) {
            return true;
        }
    }
    false
}

fn get_power(numbers: &[Number], x: usize, y: usize) -> usize {
    let filtered_numbers = numbers
        .iter()
        .cloned()
        .filter(|n| {
            (n.y == y || n.y == y + 1 || n.y == y - 1)
                && (((n.x1 >= x - 1) && (n.x1 <= x + 1)) || ((n.x2 >= x - 1) && (n.x2 <= x + 1)))
        })
        .collect::<Vec<Number>>();
    if filtered_numbers.len() == 2 {
        return filtered_numbers[0].n * filtered_numbers[1].n;
    }
    0
}

impl Problem for DayThree {
    fn part_one(&self, input: String) {
        let (symbols, numbers) = parse(input);
        let mut sum = 0;
        for n in numbers {
            if check_adjacency(&n, &symbols) {
                sum += n.n;
            }
        }
        println!("{}", sum);
    }

    fn part_two(&self, input: String) {
        let (symbols, numbers) = parse(input);
        let gears = symbols
            .into_iter()
            .filter(|(_, c)| c == &'*')
            .collect::<HashMap<(usize, usize), char>>();
        let powers_sum = gears
            .into_iter()
            .fold(0, |sum, ((x, y), _)| sum + get_power(&numbers, x, y));
        println!("{}", powers_sum);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::days::day03::{check_adjacency, Number};
    #[test]
    fn adjascency_test() {
        let mut symbols = HashMap::new();
        symbols.insert((0, 0), '#');
        let number = Number {
            n: 12,
            x1: 1,
            x2: 1,
            y: 1,
        };
        assert!(check_adjacency(&number, &symbols));
    }
}
