use crate::inputs;

pub fn part1() {
    let lines = inputs::read_lines("./src/inputs/day3.txt");
    let mut ones: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut zeros: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    lines.iter().for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                ones[i] += 1
            } else {
                zeros[i] += 1
            }
        })
    });

    let empty_gamma = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let gamma: Vec<u32> = empty_gamma
        .iter()
        .enumerate()
        .map(|(i, _)| u32::from(ones[i] > zeros[i]))
        .collect();
    let epsilon: Vec<u32> = gamma
        .iter()
        .map(|n| match n {
            1 => 0,
            0 => 1,
            _ => panic!("err"),
        })
        .collect();
    println!("gamma : {:?} epsilon : {:?}", gamma, epsilon);

    let gamma_dec = gamma.iter().fold(0, |acc, &b| acc * 2 + b as i32);
    let eps_dec = epsilon.iter().fold(0, |acc, &b| acc * 2 + b as i32);

    println!("gamma : {:?} epsilon : {:?}", gamma_dec, eps_dec);

    println!("{:?}", gamma_dec * eps_dec);
}

fn most_common_bit(lines: &[String], i: usize) -> u32 {
    let nbr_zeros = lines.iter().fold(0, |acc, l| {
        if l.chars().nth(i).unwrap() == '0' {
            acc + 1
        } else {
            acc
        }
    });
    let nbr_ones = lines.iter().fold(0, |acc, l| -> i32 {
        if l.chars().nth(i).unwrap() == '1' {
            acc + 1
        } else {
            acc
        }
    });
    u32::from(nbr_ones >= nbr_zeros)
}

pub fn part2() {
    let lines = inputs::read_lines("./src/inputs/day3.txt");

    let mut o2_rating: Vec<String> = lines.clone();
    let mut i = 0;
    let mut mcb_o2: u32;
    while o2_rating.len() > 1 {
        mcb_o2 = most_common_bit(&o2_rating, i);
        o2_rating.retain(|l| {
            mcb_o2
                == l.chars()
                    .nth(i)
                    .unwrap()
                    .to_string()
                    .parse::<u32>()
                    .unwrap()
        });
        i += 1;
    }
    let o2_rating_dec = isize::from_str_radix(&o2_rating[0][..], 2).unwrap();
    println!("o2 rating : {}", o2_rating_dec);

    let mut co2_rating: Vec<String> = lines;
    let mut j = 0;
    let mut mcb_co2: u32;
    while co2_rating.len() > 1 {
        mcb_co2 = most_common_bit(&co2_rating, j);
        co2_rating.retain(|l| {
            mcb_co2
                != l.chars()
                    .nth(j)
                    .unwrap()
                    .to_string()
                    .parse::<u32>()
                    .unwrap()
        });
        j += 1;
    }
    let co2_rating_dec = isize::from_str_radix(&co2_rating[0][..], 2).unwrap();
    println!("co2 rating : {}", co2_rating_dec);

    println!("Solution: {}", co2_rating_dec * o2_rating_dec);
}

