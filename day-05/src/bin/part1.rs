use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet,
};

struct Range {
    start: u128,
    length: u128,
}

fn main() {
    let f = File::open("./src/bin/input1.txt").unwrap();
    let mut reader = BufReader::new(f);

    let output = process(&mut reader);
    println!("{}", output);
}

fn get_numbers_in_line(line: &mut String) -> Vec<u128> {
    let mut res = Vec::new();

    let mut num = u128::MAX;
    for c in line.split("") {
        if c.chars().ne(None) && c.chars().nth(0).unwrap().is_digit(10) {
            if num == u128::MAX {
                num = c.parse::<u128>().unwrap();
            } else {
                num *= 10;
                num += c.parse::<u128>().unwrap();
            }
        } else if num != u128::MAX {
            res.push(num);
            num = u128::MAX;
        }
    }

    return res;
}

fn skip_to_data(line: &mut String, reader: &mut BufReader<File>) {
    line.clear();
    while !line.contains(":") {
        read_line(line, reader);
    }
    read_line(line, reader);
}

fn read_line(line: &mut String, reader: &mut BufReader<File>) -> bool {
    line.clear();

    return match reader.read_line(line) {
        Ok(_) => true,
        Err(_) => false,
    };
}

fn next_hop(positions: &mut Vec<u128>, line: &mut String, reader: &mut BufReader<File>) {
    skip_to_data(line, reader);
    let remaining_positions: HashSet<usize> = HashSet::from_iter(0..positions.len());
    let mut done_positions: HashSet<usize> = HashSet::new();
    loop {
        let nums = get_numbers_in_line(line);

        if nums.len() == 0 || remaining_positions.is_empty() {
            break;
        }

        let from = Range {
            start: nums[1],
            length: nums[2],
        };

        let to = Range {
            start: nums[0],
            length: nums[2],
        };

        for i in remaining_positions.iter() {
            let pos = positions[*i];
            if !done_positions.contains(i) && from.start <= pos && (from.start + from.length - 1) >= pos {
                positions[*i] = (pos - from.start) + to.start;
                let _ = done_positions.insert(*i);
            }
        }

        read_line(line, reader);
    };
}

fn process(reader: &mut BufReader<File>) -> u128 {
    let mut line = String::new();
    // Get seeds
    read_line(&mut line, reader);
    let mut positions = get_numbers_in_line(&mut line);
    // Seed to soil
    next_hop(&mut positions, &mut line, reader);
    // Soil to fertiliser
    next_hop(&mut positions, &mut line, reader);
    // Fertiliser to water
    next_hop(&mut positions, &mut line, reader);
    // Water to light
    next_hop(&mut positions, &mut line, reader);
    // Light to temperature
    next_hop(&mut positions, &mut line, reader);
    // Temperature to humidity
    next_hop(&mut positions, &mut line, reader);
    // Humidity to location
    next_hop(&mut positions, &mut line, reader);

    return *positions.iter().min().unwrap();
}
