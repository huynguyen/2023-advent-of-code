use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
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

impl From<&[u32]> for HandType {
    fn from(value: &[u32]) -> Self {
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
            'J' => 11,
            'T' => 10,
            _ => rank.to_digit(10).expect("valid numeric value for rank."),
        }
    }
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(Hand::from)
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .enumerate()
        // .inspect(|v| {dbg!(v);})
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str_to_hand() {
        let sample = "32T3K 765";
        let hand = Hand {
            bid: 765,
            cards: vec![3, 2, 10, 3, 13],
            hand_type: HandType::OnePair,
        };
        assert_eq!(hand, Hand::from(sample));
    }

    #[test]
    fn rank_cards_equal_handtypes() {
        let full_eight = Hand::from("77888 0");
        let full_seven = Hand::from("77788 0");

        assert!(full_seven < full_eight);
    }

    #[test]
    fn example_part1() {
        let sample = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!("6440", part1(sample));
    }
}
