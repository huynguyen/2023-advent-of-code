#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input.trim().split(',').map(holiday_helper_hash).sum::<u32>().to_string()
}

fn holiday_helper_hash(fragment: &str) -> u32 {
    fragment.chars().fold(0, |mut acc, ch| {
        acc += ch as u32;
        acc *= 17;
        acc %= 256;
        acc
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hash_algo_breakdown() {
        let sample = "HASH";

        assert_eq!(52, holiday_helper_hash(sample));
    }

    #[test]
    fn hash_algo() {
        let sample = "rn=1";

        assert_eq!(30, holiday_helper_hash(sample));
    }
    #[test]
    fn example_part1() {
        let sample = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!("1320", part1(sample));
    }
}
