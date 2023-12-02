fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory.lines().map(|line| {
                line.parse::<usize>()
                    .expect("Unable to parse number from line.")
            })
        })
        .map(|per_elf| per_elf.sum::<usize>())
        .max()
        .expect("Unable to find max.")
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_input() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!("24000", part1(input));
    }
}
