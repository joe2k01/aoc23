use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> f64 {
    let re = Regex::new(r"\d+").unwrap();
    let input_parse = input
        .split("\n")
        .map(|line| {
            return re
                .find_iter(line)
                .map(|m| m.as_str().parse::<f64>().unwrap())
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    let timings = &input_parse[0];
    let distances = &input_parse[1];

    timings
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let greater_root = (*t + (t.powi(2) - 4.0 * distances[i]).sqrt()) / 2.0;
            greater_root.floor() - (*t - greater_root).floor()
        })
        .filter(|c| *c > 0.0)
        .product()
}
