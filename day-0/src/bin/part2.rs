fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut elf_carrying = input
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory.lines().map(|line| {
                line.parse::<usize>()
                    .expect("Unable to parse number from line.")
            })
        })
        .map(|per_elf| per_elf.sum::<usize>())
        .collect::<Vec<usize>>();
    elf_carrying.sort_unstable();
    elf_carrying.iter().rev().take(3).sum::<usize>().to_string()
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
        assert_eq!("45000", part2(input));
    }
}
