use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    input
        .split("\n")
        .map(|line| {
            let numbers = line.split(":").collect::<Vec<_>>()[1]
                .split("|")
                .collect::<Vec<_>>();
            let winning_set: HashSet<&str> =
                HashSet::from_iter(re.find_iter(numbers[0]).map(|m| m.as_str()));

            let mut wins = 0;

            re.find_iter(numbers[1]).for_each(|m| {
                if winning_set.contains(m.as_str()) {
                    if wins == 0 {
                        wins = 1;
                    } else {
                        wins *= 2;
                    }
                }
            });

            return wins;
        })
        .sum()
}
