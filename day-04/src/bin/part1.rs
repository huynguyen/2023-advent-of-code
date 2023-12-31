use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
struct Card {
    card_id: u8,
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (head, rest) = value
            .split_once(":")
            .expect("Unable to find Card deliminiter ':'");
        let card_id = Card::parse_game_id(head);
        let (winning, numbers) = rest.split_once("|").expect("Unable to find deliminter '|'");
        Card {
            card_id,
            winning: Card::parse_numbers(winning),
            numbers: Card::parse_numbers(numbers),
        }
    }
}

impl Card {
    fn parse_game_id(value: &str) -> u8 {
        value
            .strip_prefix("Card")
            .and_then(|id| id.trim().parse::<u8>().ok())
            .expect("Unable to parse game_id")
    }

    fn parse_numbers(value: &str) -> HashSet<u32> {
        value
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().expect("Unable to convert to numeric."))
            .collect::<HashSet<_>>()
    }

    fn score(&self) -> u32 {
        let matches = self.winning.intersection(&self.numbers).count();
        match matches {
            0 => 0,
            _ => 1 << (matches - 1)
        }
    }
}

fn part1(input: &str) -> String {
    input.lines().map(Card::from).map(|c| c.score()).sum::<u32>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_from_str_to_card() {
        let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let winning = [41, 48, 83, 86, 17];
        let numbers = [83, 86, 6, 31, 17, 9, 48, 53];
        assert_eq!(
            Card {
                card_id: 1u8,
                winning: winning.into(),
                numbers: numbers.into()
            },
            Card::from(sample)
        );
    }

    #[test]
    fn score() {
        let winning = [41, 48, 83, 86, 17];
        let numbers = [83, 86, 6, 31, 17, 9, 48, 53];
        assert_eq!(
            8,
            Card {
                card_id: 1u8,
                winning: winning.into(),
                numbers: numbers.into()
            }
            .score()
        );
    }

    #[test]
    fn example_input() {
        let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!("13", part1(sample));
    }
}
