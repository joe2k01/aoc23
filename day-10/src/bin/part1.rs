use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("Result: {output}");
}

fn can_move(from: &char, to: &char, m: &char) -> bool {
    match *m {
        'U' => match *from {
            'J' | 'L' | '|' | 'S' => {
                if ['7', 'F', '|', 'S'].contains(to) {
                    return true;
                }
                return false;
            }
            _ => return false,
        },
        'D' => match *from {
            'S' | '|' | '7' | 'F' => {
                if ['|', 'S', 'J', 'L'].contains(to) {
                    return true;
                }
                return false;
            }
            _ => return false,
        },
        'L' => match *from {
            'S' | '-' | 'J' | '7' => {
                if ['S', '-', 'F', 'L'].contains(to) {
                    return true;
                }
                return false;
            }
            _ => return false,
        },
        'R' => match *from {
            'S' | '-' | 'L' | 'F' => {
                if ['S', '-', 'J', '7'].contains(to) {
                    return true;
                }
                return false;
            }
            _ => return false,
        },
        _ => return false,
    }
}

fn process(input: &str) -> i32 {
    let lines = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let re = Regex::new(r"[S]").unwrap();
    let start_pos_flat = re.find(input).unwrap().start();

    let line_len = lines.first().unwrap().len() + 1;
    let num_lines = lines.len() as i32;
    let start_row_idx = start_pos_flat / line_len;
    let start_col_idx = start_pos_flat - (line_len * start_row_idx);

    if *lines
        .get(start_row_idx)
        .unwrap()
        .get(start_col_idx)
        .unwrap()
        != 'S'
    {
        panic!("Start coordinates are not 'S' char");
    }

    let mut q = Vec::new();
    q.push((start_row_idx, start_col_idx, 0 as usize, 0 as usize, 0));
    let deltas = [(1, 0, 'D'), (-1, 0, 'U'), (0, 1, 'R'), (0, -1, 'L')];

    while q.len() > 0 {
        let (row_idx, col_idx, from_row, from_col, hops) = q.pop().unwrap();
        for delta in deltas {
            let new_row = row_idx as i32 + delta.0;
            let new_col = col_idx as i32 + delta.1;

            if new_row >= 0
                && new_col >= 0
                && new_col < (line_len - 1) as i32
                && new_row < num_lines
            {
                let new_char = lines
                    .get(new_row as usize)
                    .unwrap()
                    .get(new_col as usize)
                    .unwrap();

                let curr_char = lines
                    .get(row_idx as usize)
                    .unwrap()
                    .get(col_idx as usize)
                    .unwrap();

                if (from_col != new_col as usize || from_row != new_row as usize)
                    && can_move(curr_char, new_char, &delta.2)
                {
                    if *new_char == 'S' {
                        return (hops + 1) / 2;
                    }

                    q.push((
                        new_row as usize,
                        new_col as usize,
                        row_idx,
                        col_idx,
                        hops + 1,
                    ));
                }
            }
        }
    }

    println!("Could not loop over");
    0
}
