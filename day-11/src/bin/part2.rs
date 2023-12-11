use std::{iter, cmp::{max, min}};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input, 1_000_000);
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

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    fn steps(&self, other: &Self, rows_to_expand: &[usize], cols_to_expand: &[usize], inflation_factor: usize) -> usize {
        let x_range = min(self.x, other.x)..=max(self.x, other.x);
        let y_range = min(self.y, other.y)..=max(self.y, other.y);
        // dbg!(rows_to_expand);

        let cross_y = rows_to_expand.iter().filter(|r| y_range.contains(r)).count();
        let cross_x = cols_to_expand.iter().filter(|r| x_range.contains(r)).count();
        // dbg!(x_range, y_range, cross_x, cross_y);
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx + (inflation_factor * cross_x) + dy + (inflation_factor * cross_y) - (1 * cross_x) - (1 * cross_y)
    }
}

fn part2(input: &str, inflation_factor: usize) -> String {
    let rows_to_expand = rows_to_expand(input).collect_vec();
    let cols_to_expand = cols_to_expand(input).collect_vec();

    let galaxy_loc = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| iter::repeat(y).zip(l.chars().enumerate()))
        .flat_map(|(y, (x, ch))| match ch {
            '#' => Some(Loc { x, y }),
            _ => None,
        }).collect_vec();

    galaxy_loc.iter().enumerate().flat_map(|(idx, loc)| {
        galaxy_loc[idx+1..].iter().zip(iter::repeat(loc))
    })
        .map(|(a,b)| {
            let distance = a.steps(b, &rows_to_expand, &cols_to_expand, inflation_factor);
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
    // #[ignore]
    #[test]
    fn simplified_example() {
        let sample = "...#......
.......*..
*.........
..........
......*...
.*........
.........*
..........
.......#..
*...*.....";

        assert_eq!("15", part2(sample, 2));

    }

    #[test]
    // #[ignore]
    fn example_part2() {
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

        assert_eq!("1030", part2(sample, 10));
        assert_eq!("8410", part2(sample, 100));
    }
}
