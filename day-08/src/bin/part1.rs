use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u64 {
    let mut input_info = input.split("\n\n");
    let instructions = input_info
        .next()
        .unwrap()
        .split("")
        .filter(|&c| !c.is_empty())
        .collect::<Vec<_>>();
    let re = Regex::new(r"\w+").unwrap();

    let nodes_map = input_info
        .next()
        .unwrap()
        .split("\n")
        .fold(HashMap::new(), |mut acc, line| {
            let mut nodes = re.find_iter(line);

            acc.insert(
                nodes.next().unwrap().as_str(),
                (
                    nodes.next().unwrap().as_str(),
                    nodes.next().unwrap().as_str(),
                ),
            );

            return acc;
        });

    let mut node = "AAA";
    let target = "ZZZ";
    let mut moves = 0;
    let mut move_i = 0;

    loop {
        if node == target {
            break;
        }

        if move_i == instructions.len() {
            move_i = 0;
        }

        node = match instructions[move_i] {
            "L" => nodes_map.get(node).unwrap().0,
            "R" => nodes_map.get(node).unwrap().1,
            _ => "AAA",
        };

        moves += 1;
        move_i += 1;
    }

    moves
}
