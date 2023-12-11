use std::iter;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn rows_to_expand(spacemap: &str) -> impl Iterator<Item = usize> + '_ {
    spacemap
        .lines()
        .enumerate()
        .filter(|(_idx, l)| l.chars().all(|ch| ch == '.'))
        .map(|(idx, _l)| idx)
}

fn cols_to_expand(spacemap: &str) -> impl Iterator<Item = usize> + '_ {
    let max_x = spacemap
        .lines()
        .next()
        .and_then(|l| Some(l.chars().count() - 1))
        .expect("size of first line has a max");

    (0..=max_x).filter(|x| {
        spacemap.lines().all(|l| match l.chars().nth(*x) {
            Some('.') => true,
            _ => false,
        })
    })
}

fn expand_space(spacemap: &str) -> String {
    let rows_to_expand = rows_to_expand(spacemap).collect_vec();
    let cols_to_expand = cols_to_expand(spacemap).collect_vec();

    let mut expanded = String::new();
    spacemap
        .lines()
        .enumerate()
        .fold(&mut expanded, |acc, (y, line)| {
            let new_line = line
                .chars()
                .enumerate()
                .fold("".to_string(), |mut nline, (x, ch)| {
                    nline.push(ch);
                    if cols_to_expand.contains(&x) {
                        nline.push('.')
                    }
                    nline
                });
            acc.push_str(&new_line);
            acc.push('\n');
            if rows_to_expand.contains(&y) {
                acc.push_str(&new_line);
                acc.push('\n');
            }
            acc
        });
    _ = expanded.pop();
    expanded
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    fn steps(&self, other: &Self) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx + dy
    }
}

fn part1(input: &str) -> String {
    let expanded = expand_space(input);
    let galaxy_loc = &expanded
        .lines()
        .enumerate()
        .flat_map(|(y, l)| iter::repeat(y).zip(l.chars().enumerate()))
        .flat_map(|(y, (x, ch))| match ch {
            '#' => Some(Loc { x, y }),
            _ => None,
        }).collect_vec();

    galaxy_loc.iter().enumerate().flat_map(|(idx, loc)| {
        galaxy_loc[idx+1..].iter().zip(iter::repeat(loc))
        // iter::repeat(loc).zip(&galaxy_loc[idx..].iter())
    })
        .map(|(a,b)| {
            let distance = a.steps(b);
            // let msg = format!("{:?}-{:?} steps: {}", a, b, distance);
            // dbg!(msg);
            distance
        })
        // .inspect(|v| {dbg!(v);})
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn expand() {
        let sample = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let expected = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
        let result = expand_space(sample);

        assert_eq!(
            expected, result,
            "\nexpected: \n{} \n actual:\n{} \n",
            expected, result
        );
    }

    #[test]
    fn example_part1() {
        let sample = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!("374", part1(sample));
    }
}
