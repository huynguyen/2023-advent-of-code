use std::cmp::{max};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    input
        .lines()
        .map(Game::from)
        .inspect(|v| {dbg!(&v);})
        .map(|g| g.power())
        .sum::<u32>()
        .to_string()
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u8,
    red: u32,
    green: u32,
    blue: u32,
}

#[allow(dead_code)]
const MAX_GAME: Game = Game {id: 0, red: 12, green: 13, blue: 14};

#[allow(dead_code)]
impl Game {
    fn valid(&self) -> bool {
        self.red <= MAX_GAME.red && self.green <= MAX_GAME.green && self.blue <= MAX_GAME.blue
    }

    fn parse_game_id(game_id_line: &str) -> u8 {
        game_id_line
            .strip_prefix("Game ")
            .and_then(|id: &str| id.parse().ok())
            .expect("Invalid game_id fragment")
    }

    fn parse_blocks(blocks: &str) -> (u32, u32, u32) {
        blocks
            .split_terminator(',')
            .map(str::trim)
            .fold((0, 0, 0), |mut acc, b| {
                if let Some(n) = b.strip_suffix(" red") {
                    acc.0 = n.parse().expect("Unable to find num red boxes.");
                } else if let Some(n) = b.strip_suffix(" green") {
                    acc.1 = n.parse().expect("Unable to find num red boxes.");
                } else if let Some(n) = b.strip_suffix(" blue") {
                    acc.2 = n.parse().expect("Unable to find num red boxes.");
                }
                acc
            })
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut it = value.split_terminator(&[':', ';']).into_iter();
        let game_id = Game::parse_game_id(it.next().expect("Must have valid Game id chunk."));
        // dbg!(&game_id);

        it.map(|s| s.trim()).map(Game::parse_blocks).fold(
            Game {
                id: game_id,
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut acc, b| {
                acc.red = max(acc.red, b.0);
                acc.green = max(acc.green, b.1);
                acc.blue = max(acc.blue, b.2);
                acc
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn min_blocks_for_game() {
        let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"; 
        assert_eq!(Game { id: 1, red: 4, green: 2, blue: 6}, Game::from(sample));

    }

    #[test]
    fn example_input() {
        let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("2286", part2(sample));
    }
}
