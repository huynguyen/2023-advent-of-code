use itertools::Itertools;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    input
        .split("\n\n")
        .map(|g| {
            let h = horizontal_split(g);
            (0, h)
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
            let lines_wrong = back
                .iter()
                .zip(front.iter().rev())
                .map(|(a, b)| {
                    let it = a.1.chars().enumerate().zip(b.1.chars().enumerate());
                    let mismatch_per_line = it.filter(|((aidx, a), (bidx, b))| a != b);
                    // dbg!(&mismatch_per_line);
                    mismatch_per_line.count()
                })
                .collect_vec();

            // dbg!(idx, &lines_wrong);
            if lines_wrong.len() <= 1 {
                // println!("only 1 wrong match");
                lines_wrong.iter().all(|c| *c <= 1)
            } else {
                let less_than_one = lines_wrong.iter().filter(|c| **c != 0).all(|c| *c <= 1);
                let only_one = lines_wrong.iter().filter(|c| **c != 0).count() <= 1;
                let result = less_than_one && only_one;
                // println!(
                //     "multiple wrong matches less_than_one:{} only_one:{} result:{}",
                //     less_than_one, only_one, result
                // );
                result
            }
        })
        .map(|t| t.0)
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn horizontal_pattern() {
        let sample = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        assert_eq!(3, horizontal_split(sample));
    }

    #[test]
    fn input1_sample1() {
        let sample = "...#..##.####.#
##..##.###..###
#.#............
#...#...#..#.#.
.#..###.#.##.#.
###.###.##..##.
.....##........
##..#....#..#..
#....#...#..#..
#..##.#.#.##.#.
#..##.#.#.##.#.
#....#...#..#..
##..#....#..#..
.....##........
###.###.##..##.";

        assert_eq!(10, horizontal_split(sample));
    }

    #[test]
    fn example_part2() {
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

        assert_eq!("400", part2(sample));
    }
}
