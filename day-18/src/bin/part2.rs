use core::iter;

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
    let start = Vertex {
        x: 0,
        y: 0,
        color: "#000000".into(),
        dir: Direction::Start,
    };
    let vertices = input
        .lines()
        .map(Instruction::from)
        .fold(vec![start], |mut acc, inst| {
            match acc.last() {
                Some(prev) => {
                    acc.push(prev.from_instruction(&inst));
                }
                _ => {}
            }
            acc
        });

    let perimeter = vertices
            .iter()
            .tuple_windows()
            .map(|(v1, v2)| calculate_distance(&v1, &v2))
            .sum::<i64>();

    let area = vertices
        .iter()
        .tuple_windows()
        .flat_map(|(v1, v2)| {
            let (p, s) = calculate_area(&v1, &v2);
            iter::once(p).chain(iter::once(s))
        })
        .collect_vec();



    let result = ((area.iter().sum::<i64>() + perimeter)/ 2) + 1;
    result.to_string()
}

fn calculate_area(v1: &Vertex, v2: &Vertex) -> (i64, i64) {
    let ad = v1.x * v2.y;
    let bd = v2.x * v1.y;
    (ad, -bd)
}

fn calculate_distance(v1: &Vertex, v2: &Vertex) -> i64 {
    (v2.x - v1.x).abs() + (v2.y - v1.y).abs()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Vertex {
    x: i64,
    y: i64,
    color: String,
    dir: Direction,
}

impl Vertex {
    fn from_instruction(&self, instruction: &Instruction) -> Self {
        let (x, y) = match (&instruction.dir, instruction.steps as i64) {
            (Direction::Up, n) => (self.x, self.y - n),
            (Direction::Down, n) => (self.x, self.y + n),
            (Direction::Left, n) => (self.x - n, self.y),
            (Direction::Right, n) => (self.x + n, self.y),
            _ => panic!("Should not get a start direction"),
        };

        Self {
            x,
            y,
            color: instruction.color.clone(),
            dir: instruction.dir,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    dir: Direction,
    steps: u32,
    color: String,
}

impl Instruction {
    #[allow(dead_code)]
    fn old_from(value: &str) -> Self {
        let mut it = value.split_whitespace();
        let dir = it
            .next()
            .and_then(|d| Some(Direction::from(d)))
            .expect("A valid direction.");
        let steps = it
            .next()
            .and_then(|n| n.parse::<u32>().ok())
            .expect("A valid number of steps.");
        let color = it
            .next()
            .and_then(|s| s.strip_prefix('('))
            .and_then(|s| s.strip_suffix(')'))
            .expect("hexcolor in expected format.")
            .to_string();

        Self { dir, steps, color }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut it = value.split_whitespace().skip(2);

        let mut color = it
            .next()
            .and_then(|s| s.strip_prefix("(#"))
            .and_then(|s| s.strip_suffix(')'))
            .expect("hexcolor in expected format.")
            .to_string();

        let dir = color
            .pop()
            .and_then(|d| d.to_digit(10))
            .map(Direction::from)
            .expect("A valid direction.");

        // dbg!(&color);
        let steps: u32 = u32::from_str_radix(color.as_str(), 16).expect("A valid number of steps.");

        Self { dir, steps, color }
    }
}

impl From<u32> for Direction {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("Bad instruction"),
        }
    }
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            d @ _ => panic!("Unknown direction {}", d),
        }
    }
}

#[cfg(test)]
mod test {
    use std::iter;

    use super::*;
    #[test]
    fn shoelace_area() {
        let result = [(1, 6), (3, 1), (7, 2), (4, 4), (8, 5)]
            .iter()
            .chain(iter::once(&(1, 6)))
            .map(|p| Vertex {
                x: p.0,
                y: p.1,
                color: "#000000".into(),
                dir: Direction::Start,
            })
            .tuple_windows()
            .flat_map(|(v1, v2)| {
                let (p, s) = calculate_area(&v1, &v2);
                iter::once(p).chain(iter::once(s))
            })
            .collect_vec();

        let result = result.iter().sum::<i64>() / 2;

        assert_eq!(16, result);
    }

    #[test]
    fn part1_area() {
        let sample = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        let start = Vertex {
            x: 0,
            y: 0,
            color: "#000000".into(),
            dir: Direction::Start,
        };
        let vertices =
            sample
                .lines()
                .map(Instruction::old_from)
                .fold(vec![start.clone()], |mut acc, inst| {
                    match acc.last() {
                        Some(prev) => {
                            acc.push(prev.from_instruction(&inst));
                        }
                        _ => {}
                    }
                    acc
                });

        // dbg!(&vertices);

        let perimeter = vertices
            .iter()
            .tuple_windows()
            .map(|(v1, v2)| calculate_distance(&v1, &v2))
            .sum::<i64>();

        let area = vertices
            .iter()
            .tuple_windows()
            .flat_map(|(v1, v2)| {
                let (p, s) = calculate_area(&v1, &v2);
                iter::once(p).chain(iter::once(s))
            })
            .collect_vec();

        // dbg!(&area);
        let result = area.iter().sum::<i64>() + perimeter;
        // dbg!(result, result / 2);
        // assert_eq!(38, perimeter);
        assert_eq!(62, (result / 2)+1);
    }

    #[test]
    fn instructions() {
        let sample = "R 6 (#70c710)";
        let expected_instruction = Instruction {
            dir: Direction::Right,
            steps: 461937,
            color: String::from("70c71"),
        };
        assert_eq!(expected_instruction, Instruction::from(sample));
    }

    #[test]
    fn example_part2() {
        let sample = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

        assert_eq!("952408144115", part2(sample));
    }
}
