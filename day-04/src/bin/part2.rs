use regex::Regex;
use std::collections::{HashMap, HashSet, LinkedList};

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut card_to_wins = HashMap::new();
    let mut winning_q = LinkedList::new();

    let initial_run: i32 = input
        .split("\n")
        .map(|line| {
            let info = line.split(":").collect::<Vec<_>>();
            let card_id = re.find(info[0]).unwrap().as_str().parse::<i32>().unwrap();

            let numbers = info[1].split("|").collect::<Vec<_>>();

            let winning_set: HashSet<&str> =
                HashSet::from_iter(re.find_iter(numbers[0]).map(|m| m.as_str()));

            let mut wins = 0;

            re.find_iter(numbers[1]).for_each(|m| {
                if winning_set.contains(m.as_str()) {
                    wins += 1;
                }
            });

            let mut cards_won = Vec::new();
            for i in card_id + 1..=card_id + wins {
                cards_won.push(i);
                winning_q.push_back(i);
            }

            card_to_wins.insert(card_id, cards_won);

            return wins;
        })
        .sum();

    // This is so slow, feel like I'm looking at the problem in the wrong way
    let mut comulative_wins = 0;
    while winning_q.len() > 0 {
        let next_win = winning_q.pop_front().unwrap();

        match card_to_wins.get(&next_win) {
            Some(wins) => {
                for win in wins {
                    winning_q.push_back(*win);
                }
                comulative_wins += wins.len() as i32;
            }
            None => {}
        }
    }

    return comulative_wins + initial_run + card_to_wins.keys().len() as i32;
}
