#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
use std::{collections::HashSet, ops::Add};

use itertools::Itertools;
use Direction::*;
use Tile::*;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("./input1.txt");
    let output = part2(input);
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
}

#[derive(Debug)]
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
    fn advance(&mut self, grid: &[Vec<char>], used: &mut HashSet<Point>) -> Option<Tile> {
        used.insert(self.loc);
        let new_loc = self.loc + self.dir;
        if new_loc.in_bounds(self.maxx, self.maxy) && !used.contains(&new_loc) {
            self.loc = new_loc;
        } else {
            return None
        }
        Some(Tile::from(&grid[self.loc.y][self.loc.x]))
    }

    fn change_dir(&mut self, new_dir: Direction) {
        self.dir = new_dir;
        self.loc.dir = new_dir;
    }

    fn bounce(&mut self, tile: Tile) {
        match (self.dir, tile) {
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
                Some(Self { dir: South, ..*self })
            },
            (West, SplitVert) => {
                self.dir = North;
                Some(Self { dir: South, ..*self })
            },
            (North, SplitHoriz) => {
                self.dir = East;
                Some(Self { dir: West, ..*self })
            },
            (South, SplitHoriz) => {
                self.dir = East;
                Some(Self { dir: West, ..*self })
            },
            (North, SplitVert) => None,
            (South, SplitVert) => None,
            (East, SplitHoriz) => None,
            (West, SplitHoriz) => None,
            n @ (_, _) => panic!("Should not try to split {:?}.", n),
        }
    }
}

fn part2(input: &str) -> String {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let north_edge = (0..grid[0].len()).map(|x| Point {
        x,
        y: 0,
        dir: South,
    });
    let west_edge = (0..grid.len()).map(|y| Point {
        x: 0,
        y,
        dir: East,
    });
    let east_edge = (0..grid.len()).map(|y| Point {
        x: grid.len() - 1,
        y,
        dir: West,
    });
    let south_edge = (0..grid[0].len()).map(|x| Point {
        x,
        y: grid[0].len() - 1,
        dir: North,
    });

    let it = north_edge
        .chain(east_edge)
        .chain(south_edge)
        .chain(west_edge);
    it.map(|p| Beam {
        loc: p,
        dir: p.dir,
        maxy: grid.len(),
        maxx: grid[0].len(),
    }).map(|b| energy_count(b, &grid)).max().unwrap_or(0).to_string()
}

fn energy_count(starting_beam: Beam, grid: &[Vec<char>]) -> usize {
    let mut processing = vec![starting_beam];
    let mut used: HashSet<Point> = HashSet::new();

    while let Some(mut beam) = processing.pop() {
        match beam.advance(&grid, &mut used) {
            Some(Empty) => processing.push(beam),
            Some(UpCorner) => {
                beam.bounce(UpCorner);
                processing.push(beam);
            },
            Some(DownCorner) => {
                beam.bounce(DownCorner);
                processing.push(beam);
            },
            Some(SplitVert) => {
                if let Some(other_beam) = beam.split_beam(SplitVert) {
                    processing.push(other_beam);
                } 
                processing.push(beam);
            },
            Some(SplitHoriz) => {
                if let Some(other_beam) = beam.split_beam(SplitHoriz) {
                    processing.push(other_beam);
                }
                processing.push(beam);
            },
            None => {}
        }
    }

    used.iter().unique_by(|p| (p.x, p.y)).count()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn point_equality_in_set() {
        let mut used = HashSet::<Point>::new();
        let p1 = Point { x: 5, y: 10, dir: North };
        let p2 = Point { x: 5, y: 10, dir: North };
        let p3 = Point { x: 5, y: 10, dir: South };

        assert_eq!(true, used.insert(p1));
        assert_eq!(false, used.insert(p2));
        assert_eq!(true, used.insert(p3));
    }
    #[test]
    fn example_part2() {
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

        assert_eq!("51", part2(sample));
    }
}
