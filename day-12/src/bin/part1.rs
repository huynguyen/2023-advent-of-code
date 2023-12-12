use std::ops::RangeInclusive;

use itertools::{repeat_n, Itertools};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[allow(dead_code)]
struct SpringRecord {
    record: String,
    allocations: Vec<usize>,
}

impl From<(&str, Vec<usize>)> for SpringRecord {
    fn from(value: (&str, Vec<usize>)) -> Self {
        Self {
            record: value.0.to_string(),
            allocations: value.1,
        }
    }
}

fn part1(input: &str) -> String {
    let results = input
        .lines()
        .map(parse_line)
        .map(|(l, counts)| {
            let space_regions = regions(l)
                .into_iter()
                .filter(|(ch, _idxs)| *ch == '?')
                .collect_vec();
            combinations_match(space_regions, l, counts).len()
        })
        .collect_vec();

    // dbg!(&results);
    results.iter().sum::<usize>().to_string()
}

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let mut it = line.split_whitespace();
    let records = it.next().expect("A parsable record for a spring.");
    let nums = it
        .flat_map(|nums| nums.split(','))
        .map(|n| n.parse::<usize>().expect("convert to list of numbers"))
        .collect_vec();
    (records, nums)
}

#[allow(dead_code)]
fn regions(records: &str) -> Vec<(char, Vec<usize>)> {
    records
        .chars()
        .enumerate()
        .group_by(|(_idx, ch)| *ch)
        .into_iter()
        .flat_map(|(ch, v)| match ch {
            '?' | '#' => Some((ch, v.map(|(idx, _ch)| idx).collect_vec())),
            _ => None,
        })
        .collect_vec()
}

fn generate_permutations(length: usize) -> Vec<Vec<char>> {
    let it = ['.', '#'].into_iter();
    repeat_n(it, length)
        .multi_cartesian_product()
        // .filter(|combo| {
        //     !combo
        //         .iter()
        //         .dedup_by_with_count(|a, b| a == b)
        //         .any(|(count, item)| (*item == '#' && count > 1))
        // })
        .collect_vec()
}

fn spring_count(input: &str) -> Vec<usize> {
    regions(input)
        .iter()
        .filter(|(ch, _idxs)| *ch == '#')
        .map(|(_, spans)| spans.len())
        .collect_vec()
}

fn convert_idxs_to_range(idxs: &[usize]) -> RangeInclusive<usize> {
    match (idxs.first(), idxs.last()) {
        (Some(start), Some(end)) => *start..=*end,
        _ => panic!("idxs {:?} do not make a valid range.", idxs),
    }
}

fn combinations_match(
    space_regions: Vec<(char, Vec<usize>)>,
    line: &str,
    counts: Vec<usize>,
) -> Vec<String> {
    let all_permutations = space_regions
        .iter()
        .fold(vec![line.to_string()], |acc, (_, idxs)| {
            acc.iter()
                .flat_map(|l| {
                    generate_permutations(idxs.len())
                        .iter()
                        .map(|replacement| {
                            let mut new_line = l.chars().collect_vec();
                            _ = new_line
                                .splice(convert_idxs_to_range(idxs), replacement.iter().copied())
                                .collect_vec();
                            new_line
                        })
                        .map(|new_line| new_line.iter().join(""))
                        .collect_vec()
                })
                .collect_vec()
        });

    // dbg!(&line);
    // dbg!(&all_permutations);
    all_permutations
        .into_iter()
        .filter(|line| spring_count(line.as_str()) == counts)
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn first_line() {
        let sample = "???.### 1,1,3";

        assert_eq!("1", part1(sample));
    }

    #[test]
    fn second_line() {
        let sample = ".??..??...?##. 1,1,3";

        assert_eq!("4", part1(sample));
    }

    #[test]
    fn third() {
        let sample = "?#?#?#?#?#?#?#? 1,3,1,6";

        assert_eq!("1", part1(sample));
    }

    #[test]
    fn fourth() {
        let sample = "????.#...#... 4,1,1";

        assert_eq!("1", part1(sample));
    }

    #[test]
    fn fifth() {
        let sample = "????.######..#####. 1,6,5";

        assert_eq!("4", part1(sample));
    }

    #[test]
    fn sixth() {
        let sample = "?###???????? 3,2,1";

        assert_eq!("10", part1(sample));
    }

    #[test]
    fn groups_symbols() {
        let sample = "???.### 1,1,3";

        let (records, _) = parse_line(sample);
        let regions = regions(records);

        assert_eq!(('?', vec![0, 1, 2]), regions[0]);
        assert_eq!(('#', vec![4, 5, 6]), regions[1]);
    }

    #[test]
    fn example_part1() {
        let sample = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!("21", part1(sample));
    }
}
