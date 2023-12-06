use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");
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
                .map(|m| m.as_str())
                .collect::<Vec<_>>()
                .join("")
                .parse::<f64>()
                .unwrap();
        })
        .collect::<Vec<_>>();

    let t = &input_parse[0];
    let d = &input_parse[1];

    let greater_root = (*t + (t.powi(2) - 4.0 * d).sqrt()) / 2.0;
    greater_root.floor() - (*t - greater_root).floor()
}
