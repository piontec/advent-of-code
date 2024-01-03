use crate::DayTask;
use std::collections::HashMap;

pub struct Task;

const TI: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[derive(Debug, PartialEq, Eq, Hash)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl DayTask<i32> for Task {
    fn day_no(&self) -> u8 {
        7
    }

    fn get_part1_test_input(&self) -> &'static str {
        TI
    }

    fn get_part2_test_input(&self) -> &'static str {
        TI
    }

    fn get_part1_test_result(&self) -> i32 {
        6440
    }

    fn get_part2_test_result(&self) -> i32 {
        todo!()
    }

    fn run_p1(&self, lines: &Vec<String>) -> i32 {
        let mut hand_types: HashMap<HandType, Vec<&str>> = HashMap::new();
        let mut bids = HashMap::new();
        for line in lines {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let hand = parts[0];
            let bid = parts[1].parse::<i32>().unwrap();
            bids.insert(hand, bid);
            let hand_type = get_hand_type(hand);
            hand_types
                .entry(hand_type)
                .and_modify(|l| l.push(hand))
                .or_insert(vec![hand]);
        }

        let mut rank = 1;
        let mut result = 0;
        for hand_type in [HandType::HighCard, HandType::Pair, HandType::TwoPair, HandType::ThreeOfAKind,
        HandType::FullHouse, HandType::FourOfAKind, HandType::FiveOfAKind] {
            if !hand_types.keys().any(|k| *k == hand_type) {
                continue;
            }
            let mut hands_of_type = hand_types.get(&hand_type).unwrap().clone();
            hands_of_type.sort_by(|a, b| { compare_hands(a, b) });
            for hand in hands_of_type {
                result += rank * bids.get(hand).unwrap();
                rank += 1;
            }
        }

        result
    }

    fn run_p2(&self, lines: &Vec<String>) -> i32 {
        todo!()
    }

    fn get_part1_result(&self) -> Option<i32> {
        None
    }

    fn get_part2_result(&self) -> Option<i32> {
        None
    }
}

fn get_hand_type(hand: &str) -> HandType {
    let mut cards = HashMap::new();
    for c in hand.chars() {
        cards.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }

    if cards.len() == 1 {
        return HandType::FiveOfAKind;
    }
    if cards.values().any(|c| *c == 4) {
        return HandType::FourOfAKind;
    }
    if cards.values().any(|c| *c == 3) {
        if cards.values().any(|c| *c == 2) {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    let pairs_count = cards.values().filter(|c| **c == 2).count();
    if pairs_count == 2 {
        return HandType::TwoPair;
    }
    if pairs_count == 1 {
        return HandType::Pair;
    }
    HandType::HighCard
}

fn compare_hands(h1: &str, h2: &str) -> std::cmp::Ordering {
    assert_eq!(h1.len(), h2.len());
    for i in 0..h1.len() {
        let c1 = card_to_val(h1.chars().nth(i).unwrap());
        let c2 = card_to_val(h2.chars().nth(i).unwrap());
        if c1 == c2 {
            continue;
        }
        return c1.cmp(&c2);
    }
    panic!("Hands are equal")
}

fn card_to_val(card: char) -> u8 {
    if card.is_numeric() {
        return card.to_digit(10).unwrap() as u8;
    }
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card: {}", card)
    }
}
