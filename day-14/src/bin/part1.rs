use std::fmt;

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
    let mut map = Map::from(input);
    map.roll_north();
    map.score().to_string()
}
enum Direction {
    North,
    South,
    East,
    West,
}

struct Map {
    grid: Vec<Vec<char>>,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid = self.grid.iter().map(|l| l.iter().join("")).join("\n");
        let _ = f.debug_struct("Map").field("grid", &"").finish();

        f.write_fmt(format_args!("\n{}", grid))
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid = self.grid.iter().map(|l| l.iter().join("")).join("\n");
        f.write_fmt(format_args!("{}", grid))
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self {
            grid: value.lines().map(|l| l.chars().collect_vec()).collect_vec(),
        }
    }
}

impl Map {
    fn score(&self) -> u32 {
        let max_rows = self.grid.len();
        self.grid
            .iter()
            .enumerate()
            .map(|(idx, row)| row.iter().filter(|ch| *ch == &'O').count() as u32 * (max_rows - idx) as u32)
            .sum::<u32>()
    }

    fn roll_rocks(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.roll_north(),
            Direction::South => self.roll_south(),
            Direction::East => self.roll_east(),
            Direction::West => self.roll_west(),
        }
    }

    fn roll_west(&mut self) {
        self.grid.iter_mut().for_each(|row| {
            (0..row.len()).for_each(|idx| {
                // dbg!(idx, &row[idx..].iter().join(""));
                if row[idx] == '.' {
                    row[idx..]
                        .iter()
                        .position(|ch| ch == &'#')
                        .map_or(&row[idx..], |hidx| &row[idx..hidx + idx])
                        .iter()
                        .rposition(|ch| ch == &'O')
                        .and_then(|oidx| {
                            // dbg!(idx, oidx + idx);
                            // dbg!(&row[idx], &row[oidx + idx]);
                            Some(row.swap(idx, oidx + idx))
                        });
                };
            })
        });
    }

    fn roll_east(&mut self) {
        self.grid.iter_mut().for_each(|row| {
            (0..row.len()).rev().for_each(|idx| {
                if row[idx] == '.' {
                    let hidx = row[..idx].iter().rposition(|ch| ch == &'#').unwrap_or(0);

                    row[hidx..idx]
                        .iter()
                        .position(|ch| ch == &'O')
                        .and_then(|oidx| Some(row.swap(idx, hidx + oidx)));
                };
            })
        });
    }

    fn roll_north(&mut self) {
        let maxx = self.grid[0].len();
        let maxy = self.grid.len();
        (0..maxx).for_each(|x| {
            (0..maxy).for_each(|y| {
                if self.grid[y][x] == '.' {
                    let cube_idx = (y..maxy)
                        .find(|&yidx| self.grid[yidx][x] == '#')
                        .unwrap_or(maxy);

                    (y..cube_idx)
                        .find(|&oidx| self.grid[oidx][x] == 'O')
                        .and_then(|oidx| {
                            self.grid[oidx][x] = '.';
                            self.grid[y][x] = 'O';
                            Some(())
                        });
                }
            })
        })
    }

    fn roll_south(&mut self) {
        let maxx = self.grid[0].len();
        let maxy = self.grid.len();
        (0..maxx).for_each(|x| {
            (0..maxy).rev().for_each(|y| {
                if self.grid[y][x] == '.' {
                    let cube_idx = (0..y).rfind(|&yidx| self.grid[yidx][x] == '#').unwrap_or(0);

                    (cube_idx..y)
                        .find(|&oidx| self.grid[oidx][x] == 'O')
                        .and_then(|oidx| {
                            self.grid[oidx][x] = '.';
                            self.grid[y][x] = 'O';
                            Some(())
                        });
                }
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_row_west() {
        // let sample = "O.OO#....#";
        let sample = "OO.#O....O";
        let mut map = Map::from(sample);
        map.roll_west();

        assert_eq!("OO.#OO....", format!("{map}"));
    }
    #[test]
    fn roll_rocks() {
        let sample = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let expected = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        let mut map = Map::from(sample);
        map.roll_north();

        assert_eq!(expected, format!("{}", map));
    }

    #[test]
    fn roll_south() {
        let sample = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let expected = ".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O";
        let mut map = Map::from(sample);
        map.roll_south();

        assert_eq!(expected, format!("{}", map));
    }

    #[test]
    fn example_part1() {
        let sample = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut map = Map::from(sample);
        map.roll_north();

        assert_eq!("136", part1(sample));
    }
}
