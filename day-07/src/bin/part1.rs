use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("{}", output);
}

fn identify_group(hand: &str) -> i8 {
    let mut freq_map = HashMap::new();
    for c in hand.chars() {
        *freq_map.entry(c).or_insert(0) += 1;
    }

    let max_freq = freq_map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    // By checking frequency of most frequent card, group can be narrowed
    match max_freq.1 {
        // High card
        1 => 1,
        2 => {
            // Two pair
            if freq_map.len() == 3 {
                return 3;
            }

            // One pair
            return 2;
        }
        3 => {
            // Three of a kind
            if freq_map.len() == 3 {
                return 4;
            }
            // Full house
            return 5;
        }
        // Four of a kind
        4 => 6,
        // Five of a kind
        5 => 7,
        _ => 0,
    }
}

fn card_order(card: &char) -> i32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    }
}

fn compare_hands(a: &str, b: &str) -> Ordering {
    assert!(a.len() == b.len());

    let a_chars = a.chars().collect::<Vec<_>>();
    let b_chars = b.chars().collect::<Vec<_>>();

    for i in 0..a.len() {
        let a_o = card_order(&a_chars[i]);
        let b_o = card_order(&b_chars[i]);

        if a_o > b_o {
            return Ordering::Greater;
        } else if b_o > a_o {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

fn process(input: &str) -> u64 {
    let mut five_of_a_kind = Vec::new();
    let mut four_of_a_kind = Vec::new();
    let mut full_house = Vec::new();
    let mut three_of_a_kind = Vec::new();
    let mut two_pair = Vec::new();
    let mut one_pair = Vec::new();
    let mut high_card = Vec::new();

    // Map cards to bids
    let bid_map = input.split("\n").fold(HashMap::new(), |mut bid_acc, line| {
        let mut line_iter = line.split(" ");
        let key = line_iter.next().unwrap();
        bid_acc.insert(key, line_iter.next().unwrap().parse::<u64>().unwrap());

        // Group hands into hand groups
        match identify_group(&key) {
            1 => high_card.push(key),
            2 => one_pair.push(key),
            3 => two_pair.push(key),
            4 => three_of_a_kind.push(key),
            5 => full_house.push(key),
            6 => four_of_a_kind.push(key),
            7 => five_of_a_kind.push(key),
            _ => {}
        }

        bid_acc
    });

    // Sort by game rules
    high_card.sort_by(|a, b| compare_hands(a, b));
    one_pair.sort_by(|a, b| compare_hands(a, b));
    two_pair.sort_by(|a, b| compare_hands(a, b));
    three_of_a_kind.sort_by(|a, b| compare_hands(a, b));
    full_house.sort_by(|a, b| compare_hands(a, b));
    four_of_a_kind.sort_by(|a, b| compare_hands(a, b));
    five_of_a_kind.sort_by(|a, b| compare_hands(a, b));

    let all_lists = [
        &high_card,
        &one_pair,
        &two_pair,
        &three_of_a_kind,
        &full_house,
        &four_of_a_kind,
        &five_of_a_kind,
    ];
    let mut rank = 1;
    let mut total = 0;

    // Calculate bid
    for list in all_lists {
        for hand in list {
            let bid = bid_map.get(hand).unwrap();
            total += bid * rank;
            rank += 1;
        }
    }

    total
}
