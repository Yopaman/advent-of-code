use crate::problem::Problem;

pub struct DayTwo {}

#[derive(Debug)]
struct Game {
    id: i16,
    max_red: i16,
    max_green: i16,
    max_blue: i16,
}

impl Game {
    fn new(mut line: String) -> Self {
        let id_with_colon = line.split(' ').nth(1).unwrap();
        let id: String = id_with_colon[..id_with_colon.len() - 1].to_string();

        let cut_index = line.find(':').unwrap();
        let cleaned_string: String = line.drain(cut_index + 2..).collect();

        let games_split = cleaned_string.split("; ");

        let new_games_split = games_split
            .map(|turn: &str| {
                turn.split(", ")
                    .map(|color: &str| {
                        let mut split = color.split(' ');
                        (
                            split.next().unwrap().parse::<i16>().unwrap(),
                            split.next().unwrap(),
                        )
                    })
                    .collect::<Vec<(i16, &str)>>()
            })
            .collect::<Vec<Vec<(i16, &str)>>>();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for turn in new_games_split {
            for color in turn {
                match color {
                    (n, "red") => {
                        if max_red < n {
                            max_red = n
                        }
                    }
                    (n, "blue") => {
                        if max_blue < n {
                            max_blue = n
                        }
                    }
                    (n, "green") => {
                        if max_green < n {
                            max_green = n
                        }
                    }
                    _ => {}
                }
            }
        }

        Game {
            id: id.parse::<i16>().unwrap(),
            max_red,
            max_green,
            max_blue,
        }
    }
}

fn parse(input: String) -> Vec<Game> {
    let lines = input.split('\n');
    lines
        .map(|l| {
            if !l.is_empty() {
                Game::new(l.to_string())
            } else {
                Game {
                    id: 0,
                    max_red: 0,
                    max_blue: 0,
                    max_green: 0,
                }
            }
        })
        .collect::<Vec<Game>>()
}

impl Problem for DayTwo {
    fn part_one(&self, input: String) {
        let games = parse(input);
        let mut sum = 0;
        for game in games {
            if game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14 {
                sum += game.id;
            }
        }
        println!("{}", sum);
    }

    fn part_two(&self, input: String) {
        let games = parse(input);
        let mut powers: u32 = 0;
        for game in games {
            powers += (game.max_green * game.max_red * game.max_blue) as u32;
        }
        println!("{}", powers);
    }
}
