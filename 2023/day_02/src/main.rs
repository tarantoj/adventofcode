// struct Play {
//     red: i32,
//     green: i32,
//     blue: i32,
// }

// struct Game {
//     number: i32,
//     plays: Vec<Play>,
// }

// impl FromStr for Play {
//     type Err = std::num::ParseIntError;

//     // Parses a color hex code of the form '#rRgGbB..' into an
//     // instance of 'RGB'
//     fn from_str(play: &str) -> Result<Self, Self::Err> {
//         let red = i32::from
//         // u8::from_str_radix(src: &str, radix: u32) converts a string
//         // slice in a given base to u8
//         Ok(Play { red, green, blue })
//     }
// }

fn main() {
    let lines = include_str!("./input");
    let game_line: Vec<Vec<Vec<Vec<&str>>>> = lines
        .lines()
        .map(|line| {
            line.split(r":")
                .map(|row: &str| {
                    row.split(r";")
                        .map(|game: &str| game.split(",").map(|cube| cube.trim()).collect())
                        .collect()
                })
                .collect()
        })
        .collect();
    println!("{:?}", game_line);
}
