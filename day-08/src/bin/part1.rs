use std::{collections::BTreeMap, ops::ControlFlow};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut it = input.lines();

    let directions = it.next().expect("directions on the first line");

    let map = it.skip_while(|l| l.is_empty()).map(parse_map_line).fold(
        BTreeMap::<&str, (&str, &str)>::new(),
        |mut acc, values| {
            acc.insert(values.0, values.1);
            acc
        },
    );  

    let start = "AAA";
    let end = "ZZZ";

    let result = directions.chars().cycle().enumerate().try_fold((start, 0), |acc, (idx, dir)| {
        if acc.0 == end {
            return ControlFlow::Break((acc.0, idx));
        }

        let next = match dir {
            'L' => map.get(acc.0).expect("Next value must exist.").0,
            'R' => map.get(acc.0).expect("Next value must exist.").1,
            _ => panic!("There are no directions besides L or R.")
        };

        ControlFlow::Continue((next, idx))
    });

    match result {
        ControlFlow::Break((_, steps)) => steps.to_string(),
        _ => panic!("Should not be unreachable.")
    }
}

// really should have used nom on this one.
fn parse_map_line(map_line: &str) -> (&str, (&str, &str)) {
    let mut it = map_line.split("=");
    let key = it.next().expect("parse valid key").trim();
    let left_right = it
        .next()
        .and_then(|l| l.trim().split(", ").collect_tuple())
        .map(|(l, r)| {
            (
                l.strip_prefix('(')
                    .expect("There is a ( that needs to be stripped."),
                r.strip_suffix(')')
                    .expect("There is a ) that needs to be stripped."),
            )
        })
        .expect("parse left right");
    (key, left_right)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cycles_counts_steps() {
        let sample = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", part1(sample));
    }

    #[test]
    fn example_part1() {
        let sample = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!("2", part1(sample));
    }
}
