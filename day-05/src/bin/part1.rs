fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Mapping {
    #[allow(dead_code)]
    name: String,
    ranges: Vec<Ranges>,
}

impl Mapping {
    fn translate(&self, id: u64) -> u64 {
        self.ranges
            .iter()
            .find(|r| r.source_contains(&id))
            .map_or(id, |r| r.destination_for(&id))
    }
}

#[derive(Debug)]
struct Ranges {
    source_start: u64,
    dest_start: u64,
    range_len: u64,
}

impl Ranges {
    fn source_contains(&self, id: &u64) -> bool {
        let source_span = self.source_start..(self.source_start + self.range_len);
        source_span.contains(&id)
    }

    fn destination_for(&self, source_id: &u64) -> u64 {
        let offset = *source_id - self.source_start;
        self.dest_start + offset
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

fn part1(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let mut section_iter = lines.split(|l| l.is_empty());

    // Process seeds
    let seeds = section_iter
        .next()
        .and_then(|s| s.first())
        .and_then(|&l| l.strip_prefix("seeds: "))
        .and_then(|nums| Some(get_list_of_nums(nums)))
        .expect("List of seeds.");

    // Process maps
    let lookups: Vec<_> = section_iter.map(gen_map).collect();
    seeds
        .iter()
        .map(|s| {
            lookups.iter().fold(*s, |mut acc, map| {
                acc = map.translate(acc);
                acc
            })
        })
        .min()
        .expect("There has to be a min value")
        .to_string()
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

        assert_eq!("35", part1(sample));
    }
}
