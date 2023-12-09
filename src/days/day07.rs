use crate::problem::Problem;

pub struct DaySeven {}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum Hands {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_hand_type(hand: String) -> Hands {
    let mut sorted_hand: Vec<char> = hand.chars().collect();
    sorted_hand.sort_by(|a, b| b.cmp(a));
    let mut common_letters: [u8; 5] = [0, 0, 0, 0, 0];
    let mut index = 0;
    for card in &sorted_hand {
        let char = sorted_hand.get(index).unwrap();
        if card == char {
            common_letters[index] += 1;
        } else {
            index += 1;
        }
    }
    println!("{:?}", common_letters);
    Hands::OnePair
}

impl Problem for DaySeven {
    fn part_one(&self, input: String) {
        let hand = String::from("AA8A9");
        let got = get_hand_type(hand);
    }

    fn part_two(&self, input: String) {
        todo!();
    }
}

#[test]
fn test_get_hand_type() {
    let hand = String::from("AA8A9");
    let got = get_hand_type(hand);
    assert_eq!(got, Hands::OnePair);
}
