use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use itertools::Itertools;

const hand_size: u32 = 5;
const cards: &str = "AKQJT98765432";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse, // 23332
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Num(u8),
    T,
    J,
    Q,
    K,
    A,
}
impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '2'..='9' => Card::Num(c.to_digit(10).unwrap() as u8),
            _ => panic!("Invalid card character: {}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    bid: u64,
    hand: String,
}

pub fn solve(data: &str) -> Result<u64, &'static str> {
    let mut hands = vec![];
    // winning = bid * rank
    // rank is based on the strength of the hand. Stronget hand is top

    for (i, line) in data.lines().enumerate() {
        let mut char_count = HashMap::new();

        let raw = line.split(" ").collect::<Vec<&str>>();
        let hand = raw[0];
        let bid = raw[1];

        for c in hand.chars() {
            let counter = char_count.entry(c).or_insert(0);
            *counter += 1;
        }

        // Now i need to determine what kind of hand this is

        show_map(&char_count);

        let hand_type = get_hand_type(&char_count);

        let hand = Hand {
            bid: bid.parse::<u64>().unwrap(),
            hand: String::from(hand),
            hand_type,
        };

        println!("{:?}", hand.clone());

        hands.push(hand);
    }

    // Perform ranking
    let sorted: Vec<&Hand> = hands
        .iter()
        .sorted_by(|a, b| {
            let res = a.hand_type.cmp(&b.hand_type);

            for (i, ax) in a.hand.chars().enumerate() {
                let a_c: Card = ax.into();
                let bx: Card = b.hand.chars().nth(i).unwrap().into();
                if a_c == bx {
                    continue;
                }

                if a_c < bx {
                    return res.then_with(|| Ordering::Less);
                } else if a_c > bx {
                    return res.then_with(|| Ordering::Greater);
                }
            }

            res
        })
        .collect();

    let winnings = sorted
        .iter()
        .enumerate()
        .fold(0, |acc, x| acc + ((x.0 as u64 + 1) * x.1.bid));

    println!("{:?}", sorted);

    Ok(winnings)
}

fn show_map(map: &HashMap<char, i32>) {
    for (k, v) in map {
        println!("{}={}", k, v);
    }
}

fn get_hand_type(map: &HashMap<char, i32>) -> HandType {
    let mut hand_type = HandType::HighCard;

    for (k, v) in map {
        let new_hand_type = match v {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if hand_type == HandType::TwoPair {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if hand_type == HandType::OnePair {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        };

        if new_hand_type > hand_type {
            hand_type = new_hand_type
        }
    }

    hand_type
}

#[cfg(test)]
mod tests {
    use crate::part1::solve;

    #[test]
    fn test_example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let res = solve(input).unwrap();

        assert_eq!(res, 6440)
    }
}
