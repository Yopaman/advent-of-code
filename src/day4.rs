use crate::inputs;

#[derive(Debug)]
struct BingoNumber {
    n: u32,
    marked: bool 
}

fn parse_inputs(lines: Vec<String>) -> (Vec<u32>, Vec<Vec<Vec<BingoNumber>>>) {
    (
        lines[0].split(",").map(|n| n.parse::<u32>().unwrap()).collect(),
    
        lines[1..].iter().filter(|l| l != &"").collect::<Vec<&String>>().chunks(5)
        .map(|l| 
            l.iter().map(|&m| 
                m.split(" ")
                .filter(|n| n != &"")
                .map(|o| 
                    BingoNumber{ n: o.parse::<u32>().unwrap(), marked: false }
                ).collect::<Vec<BingoNumber>>()
            ).collect::<Vec<Vec<BingoNumber>>>()
        ).collect::<Vec<Vec<Vec<BingoNumber>>>>()
    )   
}

fn mark_number(card: &Vec<Vec<BingoNumber>>, n: u32) -> Vec<Vec<BingoNumber>> {
    card.iter().map(|l| {
        l.iter().map(|i| if i.n == n || i.marked {BingoNumber{n: i.n, marked: true}} else {BingoNumber{n: i.n, marked: false}}).collect::<Vec<BingoNumber>>()
    }).collect::<Vec<Vec<BingoNumber>>>()
}

fn is_winner(card: &Vec<Vec<BingoNumber>>) -> bool {
    card.iter().any(|l| {
        l.iter().all(|n| n.marked)
    }) ||
    (0..5).any(|i| {
        card.iter().all(|l| l[i].marked)
    })
}

fn calculate_score(card: &Vec<Vec<BingoNumber>>, win_number: u32) -> u32 {
    card.iter().fold(0, |acc, l| {
        acc + l.iter().fold(0, |l_acc, n| if !n.marked {l_acc + n.n} else {l_acc})
    })*win_number
}


pub fn part1() {
    let lines = inputs::read_lines("./src/inputs/day4.txt");
    let (numbers, mut bingo_cards) = parse_inputs(lines);
    for n in numbers {
        bingo_cards = bingo_cards.iter().map(|card| {
            mark_number(card,n)
        }).collect();
        let winner = bingo_cards.iter().find(|card| {
            is_winner(card)
        });
        if let Some(card) = winner {
            println!("Final score : {}", calculate_score(card, n));
            break;
        } 
    }
}


pub fn part2() {
    let lines = inputs::read_lines("./src/inputs/day4.txt");
    let (numbers, mut bingo_cards) = parse_inputs(lines);
    let mut last_score: u32 = 0;
    for n in numbers {
        bingo_cards = bingo_cards.iter().map(|card| {
            mark_number(card,n)
        }).collect();

        let winner = bingo_cards.iter().find(|card| {
            is_winner(card)
        });

        if let Some(card) = winner {
            let score = calculate_score(card, n);
            if score != 0 {last_score = score}
        } 

        bingo_cards = bingo_cards.into_iter().filter(|card| {
            !is_winner(card)
        }).collect();
    } 
    println!("Last score : {}", last_score);
}