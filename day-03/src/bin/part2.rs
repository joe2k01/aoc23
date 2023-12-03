use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let lines = input.split("\n").collect::<Vec<_>>();

    let mut sum = 0;

    let numbers_re = Regex::new(r"\d+").unwrap();
    let gear_re = Regex::new(r"[*]").unwrap();

    let mut numbers = vec![Vec::new() as Vec<(usize, usize, i32)>; lines.len()];
    let mut gears = Vec::new();
    for i in 0..lines.len() {
        numbers_re.find_iter(lines[i]).for_each(|m| {
            numbers[i].push((m.start(), m.end() - 1, m.as_str().parse::<i32>().unwrap()));
        });
        gear_re.find_iter(lines[i]).for_each(|m| {
            gears.push((i, m.start(), m.as_str()));
        })
    }

    for symbol_pos in gears {
        let s_row = symbol_pos.0;
        let s_col = symbol_pos.1;

        let mut viable_numbers = numbers[s_row].clone();
        if s_row > 0 {
            for n in &numbers[s_row - 1] {
                viable_numbers.push(*n);
            }
        }

        if s_row < lines.len() - 1 {
            for n in &numbers[s_row + 1] {
                viable_numbers.push(*n);
            }
        }

        let mut close_numbers = Vec::new();
        for number_pos in viable_numbers {
            let n_start = number_pos.0;
            let n_end = number_pos.1;

            if (n_start <= s_col && n_end >= s_col)
                || (n_start + 1 <= s_col && n_end + 1 >= s_col)
                || (n_start - (if n_start != 0 { 1 } else { 0 }) <= s_col && n_end - 1 >= s_col)
            {
                close_numbers.push(number_pos.2);
            }
        }

        if close_numbers.len() == 2 {
            sum += close_numbers[0] * close_numbers[1];
        }
    }

    return sum;
}
