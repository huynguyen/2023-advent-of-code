use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(PartialEq, Debug, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl HandType {
    fn normal_hand_type(value: &[u32]) -> Self {
        let card_counts = value.iter().counts();
        match card_counts.values().sorted().as_slice() {
            [5] => HandType::FiveOfKind,
            [1, 4] => HandType::FourOfKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Impossible HandType."),
        }
    }
}

impl From<&[u32]> for HandType {
    fn from(value: &[u32]) -> Self {
        let jacks = value.iter().filter(|&d| *d == 1).count();

        if jacks > 0 {
            let remaining_cards = value.iter().filter(|&d| *d != 1).counts();
            match (remaining_cards.values().sorted().as_slice(), jacks) {
                ([], 5) => HandType::FiveOfKind,
                ([4], 1) => HandType::FiveOfKind,
                ([1, 3], 1) => HandType::FourOfKind,
                ([2, 2], 1) => HandType::FullHouse,
                ([1, 1, 2], 1) => HandType::ThreeOfKind,
                ([1,1,1,1], 1) => HandType::OnePair,

                ([3], 2) =>  HandType::FiveOfKind,
                ([1, 2], 2) =>  HandType::FourOfKind,
                ([1, 1, 1], 2) =>  HandType::ThreeOfKind,
                
                ([2], 3) => HandType::FiveOfKind,
                ([1, 1], 3) => HandType::FourOfKind,

                ([1], 4) => HandType::FiveOfKind,
                n @ _ => {
                    dbg!(value, n);
                    panic!("Impossible HandType.")
                },
            }
        } else {
            HandType::normal_hand_type(value)
        }
    }

}

#[derive(Debug, PartialEq)]
struct Hand {
    bid: u32,
    cards: Vec<u32>,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (&self.hand_type, &self.cards).partial_cmp(&(&other.hand_type, &other.cards))
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards, bid) = value
            .split_whitespace()
            .collect_tuple()
            .expect("Able to split cards from bid.");
        let bid: u32 = bid.parse().expect("Able to parse bid");
        let cards: Vec<u32> = cards
            .chars()
            .map(|ch| Hand::rank_to_numeric_value(&ch))
            .collect();
        let hand_type = HandType::from(cards.as_slice());

        Hand {
            bid,
            cards,
            hand_type,
        }
    }
}

impl Hand {
    fn rank_to_numeric_value(rank: &char) -> u32 {
        match rank {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => rank.to_digit(10).expect("valid numeric value for rank."),
        }
    }
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(Hand::from)
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part2() {
        let sample = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";


        assert_eq!("5905", part2(sample));
    }
}
