use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<i32>().expect("a valid number"))
                .collect_vec()
        })
        .map(|sensor_reading| gen_sensor_until_zero(sensor_reading.as_slice()))
        .map(|sensor_data| {
            sensor_data
                .iter()
                .rev()
                .skip(1)
                .fold(0, |mut acc, curr_line| {
                    acc += curr_line.last().expect("must have a value.");
                    acc
                })
        })
        .sum::<i32>()
        .to_string()
}

fn gen_sensor_until_zero(start: &[i32]) -> Vec<Vec<i32>> {
    let mut interpolated_sensors: Vec<Vec<i32>> = vec![start.to_vec()];
    while let Some(prev) = interpolated_sensors.last() {
        if prev.iter().all(|v| *v == 0) {
            break;
        }
        let n = gen_next_line(prev.as_slice());
        interpolated_sensors.push(n);
    }
    interpolated_sensors
}

fn gen_next_line(input: &[i32]) -> Vec<i32> {
    input
        .windows(2)
        .map(|nums| {
            nums.iter()
                .rev()
                .map(|&d| d)
                .reduce(|acc, next| acc - next)
                .expect("a difference is generated between prev and curr.")
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_next_based_on_diff() {
        let sample = [0, 3, 6, 9, 12, 15];
        assert_eq!(vec![3, 3, 3, 3, 3], gen_next_line(&sample))
    }

    #[test]
    fn first_line_expansion() {
        let sample = "0 3 6 9 12 15";

        assert_eq!("18", part1(sample));
    }

    #[test]
    fn example_part1() {
        let sample = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!("114", part1(sample));
    }
}
