use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u32,
    max_distance: u32,
}

impl From<(u32, u32)> for Race {
    fn from(value: (u32, u32)) -> Self {
        Race {
            time: value.0,
            max_distance: value.1,
        }
    }
}

impl Race {
    fn holds_that_win(&self) -> Option<RangeInclusive<u32>> {
        let winners: Vec<_> = (0..=self.time).filter(|t| self.distance_if_held(*t) > self.max_distance).collect();
        let start = winners.first();
        let end = winners.last();

        match (start, end) {
            (Some(s), Some(l)) => Some(*s..=*l),
            _ => None
        }
    }

    fn distance_if_held(&self, time: u32) -> u32 {
        (self.time - time) * time
    }
}

fn part1(input: &str) -> String {
    let times = input.lines().nth(0).and_then(|tl| {
        tl.strip_prefix("Time:").and_then(|l| {
            l.trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect("Able to parse time for races."))
                // .collect::<Vec<u32>>()
                .into()
        })
    });
    let distances = input.lines().nth(1).and_then(|dl| {
        dl.strip_prefix("Distance:").and_then(|l| {
            l.trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect("Able to parse time for races."))
                // .collect::<Vec<u32>>()
                .into()
        })
    });

    times
        .zip(distances)
        .and_then(|(ts, ds)| ts.zip(ds).into())
        .expect("able to get iters")
        .map(Race::from)
        .flat_map(|r| r.holds_that_win())
        .map(|r| r.into_iter().count() as u32)
        .product::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let sample = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!("288", part1(sample));
    }
}
