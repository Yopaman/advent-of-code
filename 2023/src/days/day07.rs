use std::{
    cmp::{self, Ordering},
    collections::HashMap,
};

use crate::problem::Problem;

pub struct DaySeven {}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    h_type: u8,
    cards: String,
    value: usize,
}

fn get_hand(hand: String, value: usize, part2: bool) -> Hand {
    let mut occ_map: HashMap<char, u8> = HashMap::new();
    for c in hand.chars() {
        *occ_map.entry(c).or_insert(0) += 1;
    }

    let mut jockers_to_add: u8 = 0;
    if part2 {
        let jocker_nbr: Option<u8> = occ_map.get(&'J').copied();
        if let Some(n) = jocker_nbr {
            occ_map.remove(&'J');
            jockers_to_add += n;
        }
    }

    let mut common_letters: Vec<u8> = occ_map.into_values().collect();
    common_letters.sort();
    common_letters.reverse();
    common_letters.resize(5, 0);
    common_letters[0] += jockers_to_add;
    match common_letters[..] {
        [5, 0, 0, 0, 0] => Hand {
            h_type: 6,
            cards: hand,
            value,
        },
        [4, 1, 0, 0, 0] => Hand {
            h_type: 5,
            cards: hand,
            value,
        },
        [3, 2, 0, 0, 0] => Hand {
            h_type: 4,
            cards: hand,
            value,
        },
        [3, 1, 1, 0, 0] => Hand {
            h_type: 3,
            cards: hand,
            value,
        },
        [2, 2, 1, 0, 0] => Hand {
            h_type: 2,
            cards: hand,
            value,
        },
        [2, 1, 1, 1, 0] => Hand {
            h_type: 1,
            cards: hand,
            value,
        },
        [1, 1, 1, 1, 1] => Hand {
            h_type: 0,
            cards: hand,
            value,
        },
        _ => Hand {
            h_type: 0,
            cards: hand,
            value,
        },
    }
}

fn compare_hands(hand_a: Hand, hand_b: Hand, part2: bool) -> cmp::Ordering {
    if hand_a.h_type > hand_b.h_type {
        Ordering::Greater
    } else if hand_a.h_type < hand_b.h_type {
        Ordering::Less
    } else {
        let mut cards_a = hand_a.cards.chars();
        let mut cards_b = hand_b.cards.chars();
        let mut card_a = cards_a.next().unwrap();
        let mut card_b = cards_b.next().unwrap();
        while card_a == card_b {
            card_a = cards_a.next().unwrap();
            card_b = cards_b.next().unwrap();
        }
        if card_a.is_numeric() && !card_b.is_numeric() {
            if part2 && card_b == 'J' {
                return Ordering::Greater;
            }
            return Ordering::Less;
        } else if card_a.is_numeric() && card_b.is_numeric() {
            return card_a
                .to_digit(10)
                .unwrap()
                .partial_cmp(&card_b.to_digit(10).unwrap())
                .unwrap();
        } else if !card_a.is_numeric() && card_b.is_numeric() {
            if part2 && card_a == 'J' {
                return Ordering::Less;
            }
        } else if !card_a.is_numeric() && !card_b.is_numeric() {
            if part2 {
                return match card_a {
                    'J' => Ordering::Less,
                    'A' => Ordering::Greater,
                    'K' => {
                        if card_b == 'A' {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                    'Q' => {
                        if ['A', 'K'].contains(&card_b) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                    'T' => {
                        if card_b == 'J' {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    }
                    _ => Ordering::Less,
                };
            }
            return match card_a {
                'A' => Ordering::Greater,
                'K' => {
                    if card_b == 'A' {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                'Q' => {
                    if ['A', 'K'].contains(&card_b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                'J' => {
                    if ['A', 'K', 'Q'].contains(&card_b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                'T' => Ordering::Less,
                _ => Ordering::Less,
            };
        }
        Ordering::Equal
    }
}

impl Problem for DaySeven {
    fn part_one(&self, input: String) {
        let lines = input.trim().split('\n');
        let mut hands: Vec<Hand> = vec![];
        for line in lines {
            let mut split = line.split(' ');
            let hand = split.next().unwrap();
            let value = split.next().unwrap().parse::<usize>().unwrap();
            hands.push(get_hand(hand.to_string(), value, false));
        }
        hands.sort_by(|a, b| compare_hands(a.clone(), b.clone(), false));
        let mut sum = 0;
        for (i, hand) in hands.iter().enumerate() {
            sum += hand.value * (i + 1);
        }
        println!("{:?}", sum);
    }

    fn part_two(&self, input: String) {
        let lines = input.trim().split('\n');
        let mut hands: Vec<Hand> = vec![];
        for line in lines {
            let mut split = line.split(' ');
            let hand = split.next().unwrap();
            let value = split.next().unwrap().parse::<usize>().unwrap();
            hands.push(get_hand(hand.to_string(), value, true));
        }
        hands.sort_by(|a, b| compare_hands(a.clone(), b.clone(), true));
        let mut sum = 0;
        for (i, hand) in hands.iter().enumerate() {
            sum += hand.value * (i + 1);
        }
        println!("{:?}", sum);
    }
}

#[test]
fn test_get_hand_type() {
    let hand = String::from("JJJJJ");
    let got = get_hand(hand, 5, true);
    assert_eq!(
        got,
        Hand {
            h_type: 6,
            cards: String::from("JJJJJ"),
            value: 5
        }
    );
}
