use std::{fs::File, io::Write};

use clap::{Parser, Subcommand};
use inputs::get_input;

use crate::days::day01::DayOne;
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
            let input: String = get_input(day);
            match str::parse(part).unwrap() {
                1 => problem.part_one(input),
                2 => problem.part_two(input),
                _ => println!("error"),
            };
        }
        Some(Commands::Get { day }) => {
            let input = inputs::get_input(day);
            let file_path = format!("inputs/day{}.txt", day);
            let mut file = File::create(&file_path).expect("Error creating the file !");
            file.write_all(input.as_bytes())
                .expect("Error writing content");
            println!("Wrote to {}", file_path);
        }
        None => {}
    }
}
