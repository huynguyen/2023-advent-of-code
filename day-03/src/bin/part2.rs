use std::cmp::min;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct PartNumber {
    row_idx: usize,
    start_idx: usize,
    end_idx: usize,
}

fn part2(input: &str) -> String {
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

    let gears =
        input
            .lines()
            .enumerate()
            .fold(vec![], |mut acc: Vec<(usize, usize)>, (row, line)| {
                for m in line.match_indices('*') {
                    acc.push((row, m.0));
                }
                acc
            });

    gears
        .iter()
        .filter_map(|g| {
            let gabove = g.0.saturating_sub(1);
            let gbelow = min(g.0 + 1, input.lines().count() - 1);
            let gstart = g.1.saturating_sub(1);
            let gend = min(g.1 + 1, input.lines().nth(0).unwrap().len() - 1);

            let attached_parts: Vec<_> = parts
                .iter()
                .filter(|p| (gabove..=gbelow).contains(&p.row_idx))
                .filter(|p| p.start_idx <= gend && gstart <= p.end_idx)
                .collect();

            Some(attached_parts)
        })
        .filter(|p| p.len() == 2)
        .map(|ps| {
            ps.iter().map(|p| {
                let line = input.lines().nth(p.row_idx).unwrap();
                &line[p.start_idx..=p.end_idx]
            })
            .map(|n| n.parse::<u32>().unwrap())
            .product::<u32>()
        })
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

        assert_eq!("467835", part2(sample));
    }
}
