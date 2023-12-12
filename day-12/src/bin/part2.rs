fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part2() {
        let sample = "";

        assert_eq!("", part2(sample));
    }
}
