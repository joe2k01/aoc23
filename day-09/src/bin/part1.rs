fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i64 {
    input
        .split("\n")
        .map(|line| {
            let init = line
                .split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let mut pyramid = Vec::new();
            let mut differences = Vec::new();

            for i in 1..init.len() {
                differences.push(init[i] - init[i - 1]);
            }

            pyramid.push(init);

            loop {
                if differences.iter().filter(|n| **n != 0).count() == 0 {
                    break;
                }

                let mut new_diff = Vec::new();
                for i in 1..differences.len() {
                    new_diff.push(differences.get(i).unwrap() - differences[i - 1]);
                }

                pyramid.push(differences);

                differences = new_diff;
            }

            let mut term = *pyramid[pyramid.len() - 1].last().unwrap();
            for i in (0..(pyramid.len() - 1)).rev() {
                term += pyramid[i].last().unwrap();
            }

            term
        })
        .sum()
}
