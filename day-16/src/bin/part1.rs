#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
use std::{ops::Add, collections::HashSet};

use itertools::Itertools;
use Direction::*;
use Tile::*;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Point {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Point {
    fn in_bounds(&self, maxx: usize, maxy: usize) -> bool {
        (0..maxx).contains(&self.x) && (0..maxy).contains(&self.y)
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        let delta = other.delta();
        let x = self.x as i64 + delta.0 as i64;
        let y = self.y as i64 + delta.1 as i64;
        Self {
            x: x as usize,
            y: y as usize,
            dir: other,
        }
    }
}

impl Direction {
    fn delta(&self) -> (i8, i8) {
        match self {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
    }
}

#[derive(Debug)]
struct Beam {
    loc: Point,
    dir: Direction,
    maxx: usize,
    maxy: usize,
    repeat_count: usize,
}

enum Tile {
    Empty,
    UpCorner,
    DownCorner,
    SplitVert,
    SplitHoriz,
}

impl From<&char> for Tile {
    fn from(value: &char) -> Self {
        match value {
            '.' => Empty,
            '/' => UpCorner,
            '\\' => DownCorner,
            '|' => SplitVert,
            '-' => SplitHoriz,
            _ => unreachable!("There are no other characters in the grid."),
        }
    }
}

impl Beam {
    fn done(&self) -> bool {
        self.repeat_count > 1
    }

    fn advance(&mut self, grid: &[Vec<char>], used: &mut HashSet<Point>) -> Tile {
        used.insert(self.loc);
        let new_loc = self.loc + self.dir;
        if new_loc.in_bounds(self.maxx, self.maxy) && self.repeat_count < 2 {
            if used.contains(&new_loc) {
                self.repeat_count += 1;
            }
            self.loc = new_loc;
        } else {
            self.repeat_count = 2;
        }
        Tile::from(&grid[self.loc.y][self.loc.x])
    }

    fn bounce(&mut self, tile: Tile) {
        match (self.dir, tile) {
            (_, Empty) => {}
            (East, UpCorner) => self.dir = North,
            (South, UpCorner) => self.dir = West,
            (North, UpCorner) => self.dir = East,
            (West, UpCorner) => self.dir = South,

            (East, DownCorner) => self.dir = South,
            (South, DownCorner) => self.dir = East,
            (North, DownCorner) => self.dir = West,
            (West, DownCorner) => self.dir = North,
            (_, _) => panic!("Doesn't bounce at beam splitters."),
        }
    }

    fn split_beam(&mut self, tile: Tile) -> Option<Self> {
        match (self.dir, tile) {
            (East, SplitVert) => {
                self.dir = North;
                Some(Self {
                    dir: South,
                    ..*self
                })
            }
            (West, SplitVert) => {
                self.dir = North;
                Some(Self {
                    dir: South,
                    ..*self
                })
            }
            (North, SplitHoriz) => {
                self.dir = East;
                Some(Self {
                    dir: West,
                    ..*self
                })
            }
            (South, SplitHoriz) => {
                self.dir = East;
                Some(Self {
                    dir: West,
                    ..*self
                })
            }
            (_, _) => None,
        }
    }
}

fn part1(input: &str) -> String {
    #[allow(unused_mut)]
    let mut grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let starting_beam = Beam {
        loc: Point { x: 0, y: 0, dir: East },
        dir: East,
        maxy: grid.len(),
        maxx: grid[0].len(),
        repeat_count: 0,
    };

    let mut finished_beams = vec![];
    let mut processing = vec![starting_beam];
    let mut used: HashSet<Point> = HashSet::new();

    while let Some(mut beam) = processing.pop() {
        match beam.advance(&grid, &mut used) {
            Empty => {
                if beam.done() {
                    finished_beams.push(beam);
                } else {
                    processing.push(beam);
                }
            }
            UpCorner => {
                if beam.done() {
                    finished_beams.push(beam);
                } else {
                    beam.bounce(UpCorner);
                    processing.push(beam);
                }
            }
            DownCorner => {
                if beam.done() {
                    finished_beams.push(beam);
                } else {
                    beam.bounce(DownCorner);
                    processing.push(beam);
                }
            }
            SplitVert => {
                if beam.done() {
                    finished_beams.push(beam);
                } else {
                    if let Some(other_beam) = beam.split_beam(SplitVert) {
                        processing.push(other_beam);
                    }
                    processing.push(beam);
                }
            }
            SplitHoriz => {
                if beam.done() {
                    finished_beams.push(beam);
                } else {
                    if let Some(other_beam) = beam.split_beam(SplitHoriz) {
                        processing.push(other_beam);
                    }
                    processing.push(beam);
                }
            }
        }
    }

    used.iter().unique_by(|p| {
        (p.x, p.y)
    }).count().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_part1() {
        let sample = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        assert_eq!("46", part1(sample));
    }
}
