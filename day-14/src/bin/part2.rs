use std::{fmt, collections::HashMap};

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
    let mut map = Map::from(input);
    // map.spin(1_000_000_000);
    let score_after_spin = map.spin(1_000_000_000);
    score_after_spin.to_string()
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Map {
    grid: Vec<Vec<char>>,
    cache: HashMap<String, String>
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
            cache: HashMap::new()
        }
    }
}

impl Map {
    fn spin(&mut self, spin_count: usize) -> u64 {
        let cycle_dir = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];
        // let mut cache_hit = 0u64;
        let mut first_cache_hit: Option<(u64, u64, String)> = None;
        let mut scores: Vec<u64> = vec![];
 
        for cycle_cnt in 1..=spin_count {
            let current = format!("{}", self);
            if let Some(cached_map) = self.cache.get(&current) {
                let newmap = Self::from(cached_map.as_str());
                // cache_hit += 1;
                // println!("{cycle_cnt}: cache_hit:{}", cache_hit);

                if let Some((_idx, score, ref fcmap)) = first_cache_hit {
                    scores.push(newmap.score());
                    if score == newmap.score() && fcmap == &format!("{}", newmap) {
                        // println!("{}", self);
                        // println!("--");
                        // println!("{}", newmap);
                        break;
                    }
                } else {
                    scores.push(newmap.score());
                    first_cache_hit = Some((cycle_cnt as u64, newmap.score().into(), cached_map.clone()));
                }
                self.grid = newmap.grid;
            } else {
                cycle_dir.iter().for_each(|dir| {
                    self.roll_rocks(dir);
                });
                self.cache.insert(current, format!("{}", self));
            }
            // println!("{cycle_cnt}: {}", self.score());
        }

        // Remove the last score as it is the same as first
        _ = scores.pop();
        if let Some((first_cache_hit_idx, _, _)) = first_cache_hit {
            let remaining = spin_count as u64 - first_cache_hit_idx;
            let score_idx = remaining as usize % scores.len();
            // println!("score:{} remainign:{} idx:{} scores:{:?}", scores[score_idx], remaining, score_idx, scores);
            scores[score_idx].into()
        } else {
            self.score().into()
        }

    }

    fn score(&self) -> u64 {
        let max_rows = self.grid.len();
        self.grid
            .iter()
            .enumerate()
            .map(|(idx, row)| {
                row.iter().filter(|ch| *ch == &'O').count() as u64 * (max_rows - idx) as u64
            })
            .sum::<u64>()
    }

    fn roll_rocks(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.roll_north(),
            Direction::South => self.roll_south(),
            Direction::East => self.roll_east(),
            Direction::West => self.roll_west(),
        }
    }

    fn roll_west(&mut self) {
        self.grid.iter_mut().for_each(|row| {
            row.split_mut(|ch| ch == &'#').for_each(|sl| {
                let ocount = sl.iter().filter(|ch| *ch == &'O').count();
                sl.iter_mut().for_each(|ch| *ch = '.');
                sl.iter_mut().take(ocount).for_each(|ch| *ch = 'O')
            });
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
    fn spin_once() {
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
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        let mut map = Map::from(sample);
        map.spin(1);

        assert_eq!(expected, format!("{map}"))
    }

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
        let strmap = format!("{}", map);

        assert_eq!(expected, strmap, "expected: \n{expected} \ngot: \n{strmap}");
    }


    #[test]
    fn example_part2() {
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
        // map.spin(20);
        let result = map.spin(1_000_000_000);
        // let result = map.spin(21);

        assert_eq!("64", result.to_string());
    }
}
