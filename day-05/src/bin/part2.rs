use std::{
    fs::File,
    io::{BufRead, BufReader}, cmp::min, cmp::max,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u128,
    end: u128,
}

fn main() {
    let f = File::open("./src/bin/input2.txt").unwrap();
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

fn next_hop(positions: &mut Vec<Range>, line: &mut String, reader: &mut BufReader<File>) -> Vec<Range> {
    skip_to_data(line, reader);
    let mut ranges = Vec::new();
    loop {
        let nums = get_numbers_in_line(line);

        if nums.len() == 0 {
            break;
        }

        let from = Range {
            start: nums[1],
            end: nums[1] + nums[2] - 1,
        };

        let to = Range {
            start: nums[0],
            end: nums[0] + nums[2] - 1,
        };

        ranges.push((from, to));

        read_line(line, reader);
    }

    let mut res = vec![];
    for pos in positions {
        let mut covered_ranges = vec![];
        for (from, to) in &ranges {
            if pos.end < from.start || from.end < pos.start {
                continue;
            }
            let matching_start = max(pos.start, from.start);
            let matching_end = min(pos.end, from.end);
            covered_ranges.push(Range {
                start: matching_start,
                end: matching_end
            });
            res.push(Range {
                start: matching_start - from.start + to.start,
                end: matching_end - from.start + to.start
            });
        }
        if covered_ranges.len() == 0 {
            res.push(*pos);
            continue;
        }
        covered_ranges.sort();
        if covered_ranges[0].start > pos.start {
            res.push(Range {
                start: pos.start,
                end: covered_ranges[0].end - 1
            });
        }
        if covered_ranges[covered_ranges.len() - 1].end < pos.end {
            res.push(Range {
                start: covered_ranges[covered_ranges.len() - 1].end + 1,
                end: pos.end,
            });
        }

        for i in 0..(covered_ranges.len()-1) {
            if covered_ranges[i + 1].start > covered_ranges[i].end + 1 {
                res.push(Range {
                    start: covered_ranges[i].end + 1,
                    end: covered_ranges[i + 1].start - 1,
                });
            }
        }
    }
    
    return res;
}

fn process(reader: &mut BufReader<File>) -> u128 {
    let mut line = String::new();
    // Get seeds
    read_line(&mut line, reader);
    let positions_ranges = get_numbers_in_line(&mut line);
    let mut positions = Vec::new();

    let mut i = 0;
    loop {
        if i >= positions_ranges.len() {
            break;
        }

        let start = positions_ranges[i];
        let length = positions_ranges[i + 1];
        i += 2;

        positions.push(Range {
            start,
            end: start + length - 1,
        })
    }

    // Seed to soil
    positions = next_hop(&mut positions, &mut line, reader);

    // Soil to fertiliser
    positions = next_hop(&mut positions, &mut line, reader);

    // Fertiliser to water
    positions = next_hop(&mut positions, &mut line, reader);

    // Water to light
    positions = next_hop(&mut positions, &mut line, reader);

    // Light to temperature
    positions = next_hop(&mut positions, &mut line, reader);

    // Temperature to humidity
    positions = next_hop(&mut positions, &mut line, reader);

    // Humidity to location
    positions = next_hop(&mut positions, &mut line, reader);

    // return *positions.iter().min().unwrap();
    return positions.iter().min().unwrap().start;
}
