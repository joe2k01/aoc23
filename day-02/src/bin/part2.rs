fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let lines = input.split("\n");
    let possible_games = lines
        .map(|line| {
            let revealed_in_game = line.split(":").collect::<Vec<_>>();

            let mut red: i32 = 0;
            let mut blue: i32 = 0;
            let mut green: i32 = 0;

            revealed_in_game[1].split(";").for_each(|reveal| {
                let blocks_reveals = reveal.split(",");

                for blocks in blocks_reveals {
                    let blocks_info = blocks.split(" ").collect::<Vec<_>>();
                    let mut m_red: i32 = 0;
                    let mut m_blue: i32 = 0;
                    let mut m_green: i32 = 0;

                    if blocks_info[2].contains("red") {
                        m_red = blocks_info[1].parse::<i32>().unwrap();
                    }

                    if blocks_info[2].contains("blue") {
                        m_blue = blocks_info[1].parse::<i32>().unwrap();
                    }

                    if blocks_info[2].contains("green") {
                        m_green = blocks_info[1].parse::<i32>().unwrap();
                    }

                    if m_red > red {
                        red = m_red;
                    };
                    if m_blue > blue {
                        blue = m_blue;
                    };
                    if m_green > green {
                        green = m_green;
                    };
                }
            });

            return red * blue * green;
        })
        .sum();

    return possible_games;
}
