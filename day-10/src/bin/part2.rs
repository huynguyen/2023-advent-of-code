use std::cmp::min;
use std::collections::HashSet;
use std::hash::Hash;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Move {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, Clone, Copy)]
struct Loc {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    steps: usize,
}

impl Hash for Loc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.max_x.hash(state);
        self.max_y.hash(state);
    }
}

impl PartialEq for Loc {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.max_x == other.max_x
            && self.max_y == other.max_y
    }
}

impl Loc {
    fn is_ymax(&self) -> bool {
        self.y >= self.max_y
    }

    fn is_ymin(&self) -> bool {
        self.y == 0
    }

    fn is_xmax(&self) -> bool {
        self.x >= self.max_x
    }

    fn is_xmin(&self) -> bool {
        self.x == 0
    }

    fn is_ch_valid(&self, map: &[Vec<char>]) -> bool {
        let ch = self.char_at(map);
        !['.', '*'].contains(&ch)
    }

    fn move_dir(&self, dir: &Move) -> Option<Self> {
        use Move::*;
        match dir {
            North if !self.is_ymin() => Some(Self {
                y: self.y - 1,
                steps: self.steps + 1,
                ..*self
            }),
            South if !self.is_ymax() => Some(Self {
                y: self.y + 1,
                steps: self.steps + 1,
                ..*self
            }),
            East if !self.is_xmax() => Some(Self {
                x: self.x + 1,
                steps: self.steps + 1,
                ..*self
            }),
            West if !self.is_xmin() => Some(Self {
                x: self.x - 1,
                steps: self.steps + 1,
                ..*self
            }),
            _ => None,
        }
    }

    fn next_for(&self, ch: &char) -> Vec<Self> {
        use Move::*;
        match ch {
            'S' => [North, South, East, West]
                .iter()
                .flat_map(|d| self.move_dir(d))
                .collect(),
            '|' => [North, South]
                .iter()
                .flat_map(|d| self.move_dir(d))
                .collect(),
            '-' => [East, West].iter().flat_map(|d| self.move_dir(d)).collect(),
            'L' => [North, East]
                .iter()
                .flat_map(|d| self.move_dir(d))
                .collect(),
            'J' => [North, West]
                .iter()
                .flat_map(|d| self.move_dir(d))
                .collect(),
            '7' => [South, West]
                .iter()
                .flat_map(|d| self.move_dir(d))
                .collect(),
            'F' => [South, East]
                .iter()
                .flat_map(|d| self.move_dir(d))
                .collect(),
            '.' => {
                panic!("Should not get a .")
            }
            n @ _ => {
                vec![]
            }
        }
    }

    fn char_at(&self, map: &[Vec<char>]) -> char {
        map[self.y][self.x]
    }
}
fn swap_start(start: &Loc, map: &[Vec<char>]) -> char {
    use Move::*;
    let connected_to: Vec<Move> = [North, South, East, West]
        .into_iter()
        .filter(|d| {
            start
                .move_dir(d)
                .and_then(|loc| match (d, loc.char_at(map)) {
                    (North, '|') => Some(loc),
                    (North, '7') => Some(loc),
                    (North, 'F') => Some(loc),
                    (South, '|') => Some(loc),
                    (South, 'L') => Some(loc),
                    (South, 'J') => Some(loc),
                    (East, '-') => Some(loc),
                    (East, 'J') => Some(loc),
                    (East, '7') => Some(loc),
                    (West, '-') => Some(loc),
                    (West, 'L') => Some(loc),
                    (West, 'F') => Some(loc),
                    _ => None,
                })
                .is_some()
        })
        .collect();
    // dbg!(&connected_to);

    match connected_to.as_slice() {
        [North, South] => '|',
        [East, West] => '-',
        [North, East] => 'L',
        [North, West] => 'J',
        [South, West] => '7',
        [South, East] => 'F',
        n @ _ => panic!("Unable to replace S. {:?}", n),
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<char>]) {
    for l in map {
        println!("{:?}", l);
    }
}

fn part2(input: &str) -> String {
    let max_y = input.lines().count() - 1;
    let max_x = input
        .lines()
        .next()
        .and_then(|l| Some(l.chars().count() - 1))
        .expect("Able to get max_x.");

    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars()
                .find_position(|ch| ch == &'S')
                .and_then(|(x, _)| {
                    Some(Loc {
                        x,
                        y,
                        max_y,
                        max_x,
                        steps: 0,
                    })
                })
        })
        .expect("Able to locate starting location.");

    // dbg!(&start);

    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect();
    let replacement = swap_start(&start, map.as_slice());
    map[start.y][start.x] = replacement;

    let mut stack: Vec<Vec<Loc>> = vec![vec![start]];
    let mut answer: Vec<Loc> = vec![];
    while let Some(curr_locs) = stack.pop() {
        let next_locs = curr_locs
            .iter()
            .flat_map(|loc| {
                let ch = loc.char_at(&map);
                map[loc.y][loc.x] = '*'; // mark seen
                loc.next_for(&ch)
            })
            .collect_vec()
            .into_iter()
            .dedup()
            .filter(|loc| loc.is_ch_valid(&map))
            .collect_vec();

        // dbg!(&next_locs);
        // dbg!(&stack);
        if next_locs.is_empty() && stack.is_empty() {
            answer = curr_locs;
            break;
        } else {
            stack.push(next_locs);
        }
    }

    let mut masked_original: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect();
    map.iter().enumerate().for_each(|(y, l)| {
        l.iter().enumerate().for_each(|(x, ch)| match ch {
            '*' => (),
            _ => masked_original[y][x] = '.',
        })
    });

    // print_map(&masked_original);
    let mut changes: Vec<(usize, usize, char)> = vec![];

    for (y, line) in masked_original.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            match ch {
                '.' => {
                    let after = min(x + 1, max_x);
                    let count = line[after..]
                        .iter()
                        .filter(|&ch| ['S', 'F', '7', '|'].contains(ch))
                        .count();
                    let value = if count % 2 == 1 { 'I' } else { 'O' };
                    changes.push((y, x, value));
                }
                _ => continue,
            }
        }
    }

    // print_map(&map);
    changes.iter().for_each(|chg| {
        map[chg.0][chg.1] = chg.2;
    });
    // println!("After =====");
    // print_map(&map);
    // dbg!(&stack);
    map.iter()
        .flat_map(|v| v.iter())
        .filter(|ch| **ch == 'I')
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn base_case() {
        let sample = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!("1", part2(sample));
    }

    #[test]
    fn example_part2() {
        let sample = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!("4", part2(sample));
    }

    #[test]
    fn random_bits() {
        let sample = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!("8", part2(sample));
    }

    #[test]
    fn non_main_loop() {
        let sample = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!("10", part2(sample));
    }
}
