use crate::inputs;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;

fn parse_inputs() -> Vec<i32> {
    let lines = inputs::read_lines("./src/inputs/day7.txt");
    lines[0]
        .split(',')
        .into_iter()
        .map(|e| e.parse::<i32>().unwrap())
        .collect()
}

fn partition(data: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });

            Some((left, pivot, right))
        }
    }
}

fn select(data: &[i32], k: usize) -> Option<i32> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

fn median(data: &[i32]) -> Option<f32> {
    let size = data.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) as f32 / 2.0),
                _ => None,
            }
        }
        odd => select(data, odd / 2).map(|x| x as f32),
    }
}

pub fn part1() {
    let coords = parse_inputs();
    let best_y = median(&coords).unwrap();
    let gas_count = coords
        .into_iter()
        .fold(0, |acc, e| acc + (e - best_y as i32).abs());
    println!("min gas : {}", gas_count);
}

fn calculate_fuel_cost(from: i32, to: i32) -> i32 {
    (from..=to)
        .enumerate()
        .fold(0, |acc, (i, _)| acc + i as i32)
}

fn mean(numbers: &Vec<i32>) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn std_derivation(numbers: &Vec<i32>) -> f32 {
    let mean = mean(numbers);
    let variance = numbers
        .iter()
        .map(|value| {
            let diff = mean - (*value as f32);

            diff * diff
        })
        .sum::<f32>()
        / numbers.len() as f32;
    variance.sqrt()
}

// Need optimization, really slow
pub fn part2() {
    let coords = parse_inputs();

    let max_coord = *coords.iter().max().unwrap();
    let min_coord = *coords.iter().min().unwrap();

    let gas_count = (min_coord..=max_coord)
        .map(|y| {
            (coords).iter().fold(0, |acc, e| {
                let min = min(y, *e);
                let max = max(y, *e);
                acc + calculate_fuel_cost(min, max)
            })
        })
        .min()
        .unwrap();
    println!("min gas : {}", gas_count);
}

