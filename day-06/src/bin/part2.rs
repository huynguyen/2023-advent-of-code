use std::ops::RangeInclusive;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u64,
    max_distance: u64,
}

impl From<(u64, u64)> for Race {
    fn from(value: (u64, u64)) -> Self {
        Race {
            time: value.0,
            max_distance: value.1,
        }
    }
}

impl Race {
    fn holds_that_win(&self) -> Option<RangeInclusive<u64>> {
        let start = (0..=self.time).find(|t| self.distance_if_held(*t) > self.max_distance);
        let end = (0..=self.time)
            .rev()
            .find(|t| self.distance_if_held(*t) > self.max_distance);

        match (start, end) {
            (Some(s), Some(l)) => Some(s..=l),
            _ => None,
        }
    }

    fn distance_if_held(&self, time: u64) -> u64 {
        (self.time - time) * time
    }
}

fn part2(input: &str) -> String {
    let times = input.lines().nth(0).and_then(|tl| {
        tl.strip_prefix("Time:").and_then(|l| {
            l.trim()
                .split_whitespace()
                .join("")
                .parse::<u64>()
                .expect("Able to parse time for races.")
                .into()
        })
    });
    let distances = input.lines().nth(1).and_then(|dl| {
        dl.strip_prefix("Distance:").and_then(|l| {
            l.trim()
                .split_whitespace()
                .join("")
                .parse::<u64>()
                .expect("Able to parse time for races.")
                .into()
        })
    });

    Race::from(times.zip(distances).expect("able to get iters"))
        .holds_that_win()
        .expect("To be able to win.")
        .into_iter()
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part2() {
        let sample = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!("71503", part2(sample));
    }
}
