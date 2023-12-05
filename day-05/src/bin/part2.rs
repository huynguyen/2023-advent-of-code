use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Mapping {
    #[allow(dead_code)]
    name: String,
    ranges: Vec<Ranges>,
}

#[allow(dead_code)]
impl Mapping {
    fn translate(&self, id: u64) -> u64 {
        self.ranges
            .iter()
            .find(|r| r.source_contains(&id))
            .map_or(id, |r| r.destination_for(&id))
    }

    fn dest_to_source(&self, id: u64) -> u64 {
        self.ranges
            .iter()
            .find(|r| r.dest_contains(&id))
            .map_or(id, |r| r.source_for(&id))
    }
}

#[derive(Debug)]
struct Ranges {
    source_start: u64,
    dest_start: u64,
    range_len: u64,
}

#[allow(dead_code)]
impl Ranges {
    fn source_contains(&self, id: &u64) -> bool {
        let source_span = self.source_start..(self.source_start + self.range_len);
        source_span.contains(&id)
    }

    fn dest_contains(&self, id: &u64) -> bool {
        let dest_span = self.dest_start..(self.dest_start + self.range_len);
        dest_span.contains(&id)
    }

    fn destination_for(&self, source_id: &u64) -> u64 {
        let offset = *source_id - self.source_start;
        self.dest_start + offset
    }

    fn source_for(&self, dest_id: &u64) -> u64 {
        let offset = *dest_id - self.dest_start;
        self.source_start + offset
    }
}

impl From<Vec<u64>> for Ranges {
    fn from(value: Vec<u64>) -> Self {
        Ranges {
            source_start: value[1],
            dest_start: value[0],
            range_len: value[2],
        }
    }
}

fn part2(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let mut section_iter = lines.split(|l| l.is_empty());

    // Process seeds
    let seeds: Vec<_> = section_iter
        .next()
        .and_then(|s| s.first())
        .and_then(|&l| l.strip_prefix("seeds: "))
        .and_then(|nums| Some(get_list_of_nums(nums)))
        .and_then(|nums| {
            Some(
                nums.iter()
                    .tuples::<(&u64, &u64)>()
                    .map(|(&start, &len)| start..(start + len))
                    // .flat_map(|r| r.map(|i| i).collect::<Vec<u64>>() )
                    .collect()
            )
        })
        .expect("List of seeds.");

    // Process maps
    let lookups: Vec<_> = section_iter.map(gen_map).collect();

    (0..u64::MAX).find(|&id| {
        let possible_seed = lookups.iter().rev().fold(id, |mut acc, map| {
            acc = map.dest_to_source(acc);
            acc
        });
        seeds.iter().any(|r| r.contains(&possible_seed))
    }).unwrap().to_string()
}

fn get_list_of_nums(nums: &str) -> Vec<u64> {
    nums.split_whitespace()
        .map(|n| n.parse::<u64>().expect("seed numbers to u64l"))
        .collect()
}

fn gen_map(input: &[&str]) -> Mapping {
    let name = input
        .iter()
        .nth(0)
        .and_then(|l| l.strip_suffix(" map:"))
        .expect("Able to parse name of map.")
        .to_string();
    let ranges = input
        .iter()
        .skip(1)
        .map(|nums| get_list_of_nums(nums))
        .map(Ranges::from)
        .collect();
    Mapping { name, ranges }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_input() {
        let sample = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!("46", part2(sample));
    }
}
