fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let mut calibration_values = Vec::new();
    input.split("\n").for_each(|line| {
        let mut numbers = Vec::new();
        line.split("").for_each(|char| {
            match char.parse::<u32>() {
                Ok(n) => numbers.push(n),
                Err(_) => {},
            }
        });
        
        calibration_values.push(numbers[0] * 10 + numbers[numbers.len() - 1]);
    });

    return calibration_values.into_iter().reduce(|a, b| a + b).unwrap();
}