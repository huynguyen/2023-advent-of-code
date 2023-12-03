use std::cmp::min;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct PartNumber {
    row_idx: usize,
    start_idx: usize,
    end_idx: usize,
}

impl PartNumber {
    fn has_adjacent_symbol(&self, map: &str) -> bool {
        let extended_start = self.start_idx.saturating_sub(1);
        let extended_end = min(self.end_idx + 1, map.lines().nth(0).unwrap().len() - 1);

        let above = self.row_idx.saturating_sub(1);
        let below = min(self.row_idx + 1, map.lines().count() - 1);

        [above, self.row_idx, below].iter().any(|r_idx| {
            map.lines()
                .nth(*r_idx)
                .and_then(|line| {
                    let has_symbol = &line[extended_start..=extended_end]
                        .chars()
                        .filter(|&ch| !(ch.is_ascii_digit() || ch == '.'))
                        .count()
                        > &0;
                    // dbg!(&line[extended_start..=extended_end], has_symbol);
                    has_symbol.then(|| ())
                })
                .is_some()
        })
    }
}

fn part1(input: &str) -> String {
    let parts: Vec<_> =
        input
            .lines()
            .enumerate()
            .fold(vec![], |mut acc: Vec<PartNumber>, (idx, line)| {
                let locs: Vec<_> = number_pos(line)
                    .iter()
                    .map(|loc| PartNumber {
                        row_idx: idx,
                        start_idx: loc.0,
                        end_idx: loc.1,
                    })
                    .collect();
                acc.extend(locs);
                acc
            });

    parts
        .iter()
        .filter(|p| p.has_adjacent_symbol(input))
        .map(|p| {
            let line = input.lines().nth(p.row_idx).unwrap();
            &line[p.start_idx..=p.end_idx]
        })
        .map(|n| n.parse::<u32>().unwrap())
        .sum::<u32>()
        .to_string()
}

fn number_pos(input: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];

    let mut it = input.char_indices();

    while let Some((start_idx, _)) = it.find(|(_, ch)| ch.is_ascii_digit()) {
        let len = find_len(&input[start_idx..]);
        let end_idx = start_idx + len;
        result.push((start_idx, end_idx));
        for _ in 0..len {
            it.next();
        }
    }
    result
}

fn find_len(input: &str) -> usize {
    input
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<Vec<_>>()
        .len()
        - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_range_of_digits() {
        let sample = "..35..633.";
        assert_eq!(vec![(2, 3), (6, 8)], number_pos(sample));
    }

    #[test]
    fn example_input() {
        let sample = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!("4361", part1(sample));
    }
}
