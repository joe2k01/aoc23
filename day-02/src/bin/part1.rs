fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let lines = input.split("\n");
    let red = 12;
    let green = 13;
    let blue = 14;
    let possible_games = lines
        .map(|line| {
            let revealed_in_game = line.split(":").collect::<Vec<_>>();
            let game_id = revealed_in_game[0].split(" ").collect::<Vec<_>>()[1]
                .parse::<i32>()
                .unwrap();

            return match revealed_in_game[1].split(";").any(|reveal| {
                let blocks_reveals = reveal.split(",");
                for blocks in blocks_reveals {
                    let blocks_info = blocks.split(" ").collect::<Vec<_>>();

                    if blocks_info[2].contains("red")
                        && blocks_info[1].parse::<u32>().unwrap() > red
                    {
                        return true;
                    }

                    if blocks_info[2].contains("blue")
                        && blocks_info[1].parse::<u32>().unwrap() > blue
                    {
                        return true;
                    }

                    if blocks_info[2].contains("green")
                        && blocks_info[1].parse::<u32>().unwrap() > green
                    {
                        return true;
                    }
                }

                return false;
            }) {
                true => 0,
                false => game_id,
            };
        })
        .sum();

    return possible_games;
}
