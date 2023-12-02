use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(starts_with)
        // .inspect(|s| {
        //     dbg!(s);
        // })
        .map(|digits| {
            digits
                .parse::<u32>()
                .expect("Unable to parse string into digits.")
        })
        .sum::<u32>()
        .to_string()
}

fn starts_with(input: &str) -> String {
    let matches: Vec<_> = (0..input.len()).filter_map(|idx| {
        let curr_line = &input[idx..];
        let num = if curr_line.starts_with("one") {
            '1'
        } else if curr_line.starts_with("two") {
            '2'
        } else if curr_line.starts_with("three") {
            '3'
        } else if curr_line.starts_with("four") {
            '4'
        } else if curr_line.starts_with("five") {
            '5'
        } else if curr_line.starts_with("six") {
            '6'
        } else if curr_line.starts_with("seven") {
            '7'
        } else if curr_line.starts_with("eight") {
            '8'
        } else if curr_line.starts_with("nine") {
            '9'
        } else {
            curr_line.chars().next().unwrap()
        };
        num.to_digit(10)
    }).collect();

    matches
        .first()
        .zip(matches.last())
        .and_then(|(first, last)| Some(format!("{}{}", first, last)))
        .expect("Unable to locate digits.")
}

fn convert_to_num_rep(possible_num: &str) -> &str {
    match possible_num {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => possible_num
    }
}

#[allow(dead_code)]
fn locate_by_reg(line: &str) -> String {
    // This doesn't work because of overlapping digits, see overlapping_ending.
    let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let matches: Vec<_> = re.find_iter(line)
        .map(|m| m.as_str())
        .map(convert_to_num_rep)
        .collect();

    dbg!(&matches);

    matches
        .first()
        .zip(matches.last())
        .and_then(|(first, last)| Some(format!("{}{}", first, last)))
        .expect("Unable to locate digits.")
}

#[allow(dead_code)]
fn locate_by_ascii_digit(line: &str) -> String {
    let first_digit = line
        .chars()
        .find(|ch: &char| char::is_ascii_digit(ch))
        .expect("Unable to locate first digit");
    let last_digit = line
        .chars()
        .rfind(|ch: &char| char::is_ascii_digit(ch))
        .expect("Unable to locate last digit");

    format!("{}{}", first_digit, last_digit)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_input() {
        let sample = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", part2(sample));
    }

    #[test]
    fn overlapping_ending() {
        let sample = "x3fourtwone";
        // the regex would return 32.
        assert_eq!("31", part2(sample));
    }

    #[test]
    fn bigger_sample() {
        let s1 = "9dlvndqbddghpxc
rtkrbtthree8sixfoureight6
fdxrqmfxdkstpmcj7lmphgsmqqnmjrtwo3tcbc
onetjcsmgk57nvmkvcvkdtqtsksgpchsfsjzkkmb
six8threepvlxttc85two
";
        // 99 <-- apparently if there is only 1 digit it you can reuse it. 
        // 36
        // 73
        // 17
        // 62
        assert_eq!("287", part2(s1));
        
    }

    #[test]
    fn converts_digits() {
        assert_eq!("0", convert_to_num_rep("0"));
        assert_eq!("1", convert_to_num_rep("one"));
    }
}
