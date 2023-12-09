use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(file: &str) -> Vec<String> {
    let f = File::open(file).expect("Unable to open file");
    let f = BufReader::new(f);

    f.lines().map(|l| l.expect("Could not parse line")).collect()
}