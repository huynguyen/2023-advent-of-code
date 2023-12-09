use itertools::Itertools;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|d| d.parse::<i32>().expect("a valid number"))
                .collect_vec()
        })
        .map(|mut sensor_reading| {
            sensor_reading.reverse();
            gen_sensor_until_zero(sensor_reading.as_slice())
        })
        .map(|sensor_data| {
            sensor_data
                .iter()
                .rev()
                .skip(1)
                .fold(0, |mut acc, curr_line| {
                    acc = curr_line.last().expect("must have a value.") - acc;
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
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            a - b
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn subset_part2() {
        let sample = "10 13 16 21 30 45";

        assert_eq!("5", part2(sample));
    }
 
    #[test]
    fn example_part2() {
        let sample = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!("2", part2(sample));
    }
}
