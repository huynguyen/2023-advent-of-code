use std::collections::HashSet;
use std::hash::Hash;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
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
        self.x == other.x && self.y == other.y && self.max_x == other.max_x && self.max_y == other.max_y 
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

fn print_map(map: &[Vec<char>]) {
    for l in map {
        println!("{:?}", l);
    }
}

fn part1(input: &str) -> String {
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
    print_map(&map);
    // dbg!(&stack);
    answer
        .iter()
        .map(|l| l.steps)
        .max()
        .expect("highest step.")
        .to_string()

}

fn depth_search(loc: &Loc, visited: &HashSet<Loc>, map: &[Vec<char>]) -> usize {
    let ch = loc.char_at(map);
    loc.next_for(&ch).iter().filter(|loc| loc.is_ch_valid(map)).filter(|loc| visited.contains(loc)).map(|loc| {
        let mut new_visited = visited.clone();
        new_visited.insert(*loc);
        depth_search(loc, visited, map)
    }).max().unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn char_at() {
        let mut sample = ".....
.S-7.
.|.|.
.L-J.
....."
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        let j = Loc {
            x: 3,
            y: 3,
            max_y: 4,
            max_x: 4,
            steps: 0,
        };
        let s = Loc {
            x: 1,
            y: 1,
            max_y: 4,
            max_x: 4,
            steps: 0,
        };

        let r = swap_start(&s, sample.as_slice());
        sample[s.y][s.x] = r;
        for l in &sample {
            println!("{:?}", l);
        }
        assert_eq!('J', j.char_at(&sample));
    }

    #[test]
    fn base_case() {
        let sample = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!("4", part1(sample));
    }

    #[test]
    fn example_part1() {
        let sample = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!("8", part1(sample));
    }
}
