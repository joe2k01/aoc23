use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let lines = input.split("\n").collect::<Vec<_>>();

    let mut sum = 0;

    let numbers_re = Regex::new(r"\d+").unwrap();
    let symbols_re = Regex::new(r"[^\d.]").unwrap();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for i in 0..lines.len() {
        numbers_re.find_iter(lines[i]).for_each(|m| {
            numbers.push((
                i,
                m.start(),
                m.end() - 1,
                m.as_str().parse::<i32>().unwrap(),
            ));
        });
        symbols_re.find_iter(lines[i]).for_each(|m| {
            symbols.push((i, m.start(), m.as_str()));
        })
    }

    for number_pos in numbers {
        for symbol_pos in &symbols {
            let s_row = symbol_pos.0;
            let s_col = symbol_pos.1;

            let n_row = number_pos.0;
            let n_start = number_pos.1;
            let n_end = number_pos.2;

            if s_row == n_row || s_row + 1 == n_row || s_row - 1 == n_row {
                if (n_start <= s_col && n_end >= s_col)
                    || (n_start + 1 <= s_col && n_end + 1 >= s_col)
                    || (n_start - (if n_start != 0 { 1 } else { 0 }) <= s_col && n_end - 1 >= s_col)
                {
                    sum += number_pos.3;
                }
            }
        }
    }

    return sum;
}
