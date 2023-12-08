use std::{collections::BTreeMap, ops::ControlFlow};

use itertools::Itertools;
use num_integer::Integer;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut it = input.lines();

    let directions = it.next().expect("directions on the first line");

    let map = it.skip_while(|l| l.is_empty()).map(parse_map_line).fold(
        BTreeMap::<&str, (&str, &str)>::new(),
        |mut acc, values| {
            acc.insert(values.0, values.1);
            acc
        },
    );

    let starts: Vec<&str> = map
        .keys()
        .filter(|&&k| k.ends_with("A"))
        .map(|&k| k)
        .collect();

    let result = starts.iter().map(|s| {
        let found_path = directions
            .chars()
            .cycle()
            .enumerate()
            .try_fold((*s, 0), |acc, (idx, dir)| {
                if acc.0.ends_with("Z") {
                    return ControlFlow::Break((acc.0, idx));
                }

                let new_loc = match dir {
                    'L' => map.get(acc.0).expect("Next value must exist.").0,
                    'R' => map.get(acc.0).expect("Next value must exist.").1,
                    _ => panic!("There are no directions besides L or R."),
                };

                ControlFlow::Continue((new_loc, idx))
            });

        match found_path {
            ControlFlow::Break((_, steps)) => steps,
            _ => panic!("Should not be unreachable."),
        }
    });

    result.reduce(|acc, n| acc.lcm(&n)).expect("Non empty number of steps to reach Z").to_string()
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
    fn example_part2() {
        let sample = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!("6", part2(sample));
    }
}
