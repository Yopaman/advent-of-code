use std::time::Instant;
use std::{fs::File, io::Write};

use clap::{Parser, Subcommand};
use inputs::get_input;

use crate::days::day01::DayOne;
use crate::days::day02::DayTwo;
use crate::days::day03::DayThree;
use crate::days::day04::DayFour;
use crate::days::day05::DayFive;
use crate::days::day06::DaySix;
use crate::days::day07::DaySeven;
use crate::days::day08::DayEight;
use crate::days::day09::DayNine;
use crate::days::day10::DayTen;
use crate::days::day11::DayEleven;
use crate::problem::Problem;

mod days;
mod inputs;
mod problem;

#[derive(Parser)]
#[command(author, version, about, long_about = None, subcommand_required = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        day: String,

        part: String,

        #[arg(short, long, default_value_t = false)]
        benchmark: bool,
    },
    Get {
        day: String,
    },
}

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne {})),
        2 => Some(Box::new(DayTwo {})),
        3 => Some(Box::new(DayThree {})),
        4 => Some(Box::new(DayFour {})),
        5 => Some(Box::new(DayFive {})),
        6 => Some(Box::new(DaySix {})),
        7 => Some(Box::new(DaySeven {})),
        8 => Some(Box::new(DayEight {})),
        9 => Some(Box::new(DayNine {})),
        10 => Some(Box::new(DayTen {})),
        _ => None,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run {
            day,
            part,
            benchmark,
        }) => {
            let boxed_problem: Box<dyn Problem> = day_to_problem(str::parse(day).unwrap()).unwrap();
            let problem: &dyn Problem = boxed_problem.as_ref();
            let input: String = get_input(day).expect("Input does not exist.");
            match str::parse(part).unwrap() {
                1 => {
                    if *benchmark {
                        let start = Instant::now();
                        problem.part_one(input);
                        let duration = start.elapsed();
                        println!("Time elapsed : {:?}", duration);
                    } else {
                        problem.part_one(input);
                    }
                }
                2 => {
                    if *benchmark {
                        let start = Instant::now();
                        problem.part_two(input);
                        let duration = start.elapsed();
                        println!("Time elapsed : {:?}", duration);
                    } else {
                        problem.part_two(input);
                    }
                }
                _ => println!("error"),
            };
        }
        Some(Commands::Get { day }) => {
            let input = inputs::get_input_from_website(day);
            let file_path = format!("inputs/day{}.txt", day);
            let mut file = File::create(&file_path).expect("Error creating the file !");
            file.write_all(input.as_bytes())
                .expect("Error writing content");
            println!("Wrote to {}", file_path);
        }
        None => {}
    }
}
