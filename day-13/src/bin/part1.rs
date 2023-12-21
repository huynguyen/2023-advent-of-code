use itertools::Itertools;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|g| {
            let h = horizontal_split(g);
            let v = vertical_split(g);
            (v, h)
        })
        .map(|(v, h)| score(v, h))
        .sum::<usize>()
        .to_string()
}

fn score(vert: usize, horz: usize) -> usize {
    vert + (horz * 100)
}

fn vertical_split(input: &str) -> usize {
    let chars = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let result = transpose(chars.as_slice());
    let transposed = result.iter().map(|l| l.iter().join("")).join("\n");
    horizontal_split(transposed.as_str())
}

fn transpose<T: Copy>(input: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!input.is_empty());
    let xmax = input[0].len();
    let ymax = input.len();

    (0..xmax).fold(vec![], |mut acc, x| {
        let new_row = (0..ymax).map(|y| input[y][x]).collect_vec();
        acc.push(new_row);
        acc
    })
}

fn horizontal_split(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .skip(1)
        .find(|(idx, _line)| {
            let mut front = input.lines().enumerate().collect::<Vec<(usize, &str)>>();
            let back = front.split_off(*idx);
            back.iter().zip(front.iter().rev()).all(|(a, b)| a.1 == b.1)
        })
        .map(|t| t.0)
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn vertical_pattern() {
        let sample = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

        assert_eq!(5, vertical_split(sample));
    }
    #[test]
    fn horizontal_pattern() {
        let sample = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(4, horizontal_split(sample));
    }

    #[test]
    fn example_part1() {
        let sample = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!("405", part1(sample));
    }
}
