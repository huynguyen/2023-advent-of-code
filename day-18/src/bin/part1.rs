use std::iter::successors;

use itertools::{repeat_n, Itertools};

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

fn print_map(input: &[Vec<char>]) {
    for r in input {
        let just = r.iter().join(" ");
        println!("{:?}", just);
    }
}

fn build_map(trenches: &[Trench]) -> Vec<Vec<char>> {
    let maxx = trenches.iter().map(|t| t.x).max().unwrap_or(0);
    let minx = trenches.iter().map(|t| t.x).min().unwrap();
    let maxy = trenches.iter().map(|t| t.y).max().unwrap_or(0);
    let miny = trenches.iter().map(|t| t.y).min().unwrap();

    // dbg!(minx, maxx, miny, maxy);
    let dx = maxx + minx.abs();
    let dy = maxy + miny.abs();
    let mut map = (0..dy + 1)
        .map(|_| repeat_n('.', (dx + 1) as usize).collect_vec())
        .collect_vec();

    for t in trenches {
        // dbg!(t);
        let x = (t.x + minx.abs()) as usize;
        let y = (t.y + miny.abs()) as usize;
        map[y][x] = match t.dir {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Start => 's',
        };
    }

    for t in trenches {
        let x = (t.x + minx.abs()) as usize;
        let y = (t.y + miny.abs()) as usize;
        let belowcorner = map.get(y + 1).and_then(|r| r.get(x));
        let abovecorner = y
            .checked_sub(1)
            .and_then(|yidx| map.get(yidx))
            .and_then(|r| r.get(x));
        let rightcorner = map.get(y).and_then(|r| r.get(x + 1));
        let leftcorner = map
            .get(y)
            .and_then(|r| x.checked_sub(1).and_then(|idx| r.get(idx)));

        match (belowcorner, rightcorner, leftcorner, abovecorner) {
            (Some(bch), Some(rch), _, _) if ![bch, rch].contains(&&'.') => {
                map[y][x] = 'F';
            }
            (Some(bch), _, Some(lch), _) if ![bch, lch].contains(&&'.') => {
                map[y][x] = '7';
            }
            (_, _, Some(lch), Some(ach)) if ![ach, lch].contains(&&'.') => {
                map[y][x] = 'J';
            }
            (_, Some(rch), _, Some(ach)) if ![ach, rch].contains(&&'.') => {
                map[y][x] = 'L';
            }
            _ => {}
        }
    }

    map
}

fn fill_inner(map: &mut Vec<Vec<char>>) {
    for row in map {
        let maxx = row.len();
        let new_row = row
            .iter()
            .enumerate()
            .map(|(idx, c)| {
                match c {
                    '.' => {
                        // .filter(|&ch| ['S', 'F', '7', '|'].contains(ch))
                        let allow = ['^', 'v', 's', 'F', '7'];
                        let crosses = row[idx + 1..]
                            .iter()
                            .dedup()
                            .filter(|&ch| allow.contains(ch))
                            .count();
                        if crosses % 2 == 1 {
                            '#'
                        } else {
                            *c
                        }
                    }
                    _ => *c,
                }
            })
            .collect_vec();
        *row = new_row;
    }
}

fn part1(input: &str) -> String {
    let start = Trench {
        x: 0,
        y: 0,
        color: "#000000".into(),
        dir: Direction::Start,
    };
    let trenches = input
        .lines()
        .map(Instruction::from)
        .fold(vec![start], |mut acc, inst| {
            match acc.last() {
                Some(prev) => {
                    let instructions = successors(Some(prev.clone()), |p: &Trench| {
                        Some(p.from_instruction(&inst))
                    })
                    .skip(1)
                    .take(inst.steps as usize);
                    acc.extend(instructions);
                }
                _ => {}
            }
            acc
        });

    let mut map = build_map(&trenches);
    fill_inner(&mut map);

    print_map(&map);
    map.iter().map(|r| r.iter().filter(|ch| *ch != &'.').count()).sum::<usize>().to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Trench {
    x: i64,
    y: i64,
    color: String,
    dir: Direction,
}

impl Trench {
    fn from_instruction(&self, instruction: &Instruction) -> Self {
        let (x, y) = match &instruction.dir {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
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

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
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
    use super::*;
    #[test]
    #[ignore]
    fn instructions() {
        let sample = "R 6 (#70c710)";
        let expected_instruction = Instruction {
            dir: Direction::Right,
            steps: 6,
            color: String::from("#70c710"),
        };
        assert_eq!(expected_instruction, Instruction::from(sample));
    }

    #[test]
    fn example_part1() {
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

        assert_eq!("62", part1(sample));
    }
}
