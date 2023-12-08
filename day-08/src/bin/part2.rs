use regex::Regex;
use std::collections::HashMap;
use std::cmp::{max, min};

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    println!("{}", output);
}

fn lcm(a: u64, b: u64) -> u64 {
    let large = max(a, b);
    let small = min(a, b);
    let mut i = large;

    while i % small != 0 {
        i += large;
    }

    return i;
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

    let mut nodes = Vec::new();
    let nodes_map = input_info
        .next()
        .unwrap()
        .split("\n")
        .fold(HashMap::new(), |mut acc, line| {
            let mut nodes_in_line = re.find_iter(line);

            let node = nodes_in_line.next().unwrap().as_str();
            if node.ends_with("A") {
                nodes.push(node);
            }

            acc.insert(
                node,
                (
                    nodes_in_line.next().unwrap().as_str(),
                    nodes_in_line.next().unwrap().as_str(),
                ),
            );

            return acc;
        });

    let mut moves = Vec::new();
    let mut move_i = 0;

    for i in 0..nodes.len() {
        let mut moves_counter: u64 = 0;
        let mut node = nodes[i];
        loop {
            if node.ends_with("Z") {
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

            moves_counter += 1;
            move_i += 1;
        }
        moves.push(moves_counter);
    }

    let mut acc = moves[0];
    for i in 1..moves.len() {
        acc = lcm(acc, moves[i]);
    }

    acc
}
