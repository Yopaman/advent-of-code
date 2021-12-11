use crate::inputs;



pub fn part1() {
    let lines = inputs::read_lines("./src/inputs/day3.txt");
    let mut ones: [u32;12] = [0,0,0,0,0,0,0,0,0,0,0,0];
    let mut zeros: [u32;12] = [0,0,0,0,0,0,0,0,0,0,0,0];

    lines.iter()
    .for_each(|l| {
        l.chars().enumerate()
        .for_each(|(i, c)| if c == '1' {ones[i] += 1} else {zeros[i] += 1})
    });

    let empty_gamma = vec![0,0,0,0,0,0,0,0,0,0,0,0];
    let gamma: Vec<u32> = empty_gamma.iter().enumerate().map(|(i, _)| if ones[i] > zeros[i] {1} else {0} ).collect();
    let epsilon: Vec<u32> = gamma.iter().map(|n| {
        match n {
            1 => 0,
            0 => 1,
            _ => panic!("err")
        }
    }).collect();
    println!("gamma : {:?} epsilon : {:?}", gamma, epsilon);
    
    let gamma_dec = gamma.iter().fold(0, |acc, &b| acc*2 + b as i32);
    let eps_dec = epsilon.iter().fold(0, |acc, &b| acc*2 + b as i32);

    println!("gamma : {:?} epsilon : {:?}", gamma_dec, eps_dec);

    println!("{:?}", gamma_dec*eps_dec);

}