fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let first_digit = line.chars()
                .find(|ch: &char| char::is_ascii_digit(ch))
                .expect("Unable to locate first digit");
            let last_digit = line.chars()
                .rfind(|ch: &char| char::is_ascii_digit(ch))
                .expect("Unable to locate last digit");
            format!("{}{}", first_digit, last_digit)
        })
        .map(|digits| digits.parse::<u32>().expect("Unable to parse string into digits."))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_input() {
        let sample = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", part1(sample));
    }
}
