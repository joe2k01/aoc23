use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut calibration_values = Vec::new();
    input.split("\n").for_each(|line| {
        let first = (0..line.len())
            .find_map(|i| numbers.keys().find(|key| line[i..].starts_with(*key)))
            .unwrap();
        let second = (0..line.len())
            .find_map(|i| {
                numbers
                    .keys()
                    .find(|key| line[..(line.len() - i)].ends_with(*key))
            })
            .unwrap();
        calibration_values.push(numbers[first] * 10 + numbers[second]);
    });

    return calibration_values.into_iter().reduce(|a, b| a + b).unwrap();
}
