#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use Instruction::*;
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

#[derive(Debug, PartialEq)]
enum Instruction<'a> {
    Remove(&'a str),
    Insert(Lens<'a>)
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        match value.find('=') {
            Some(byte_idx) => {
                let (label, rest) = value.split_at(byte_idx);
                let focal_length = rest.strip_prefix('=').and_then(|slice| slice.parse::<usize>().ok()).expect("parse focal_length");
                Insert(Lens { label, focal_length })
            },
            None => Remove(value.strip_suffix('-').expect("Must be one of two instructions."))
        }
    }
}

fn part2(input: &str) -> String {
    let mut boxes: [Vec<Lens>; 256] = core::array::from_fn(|_| vec![]);
    input.trim().split(',').map(Instruction::from).for_each(|inst| {
        match inst {
            Remove(label) => {
                let box_idx = holiday_helper_hash(label) as usize;
                if let Some(lens_idx) = boxes[box_idx].iter().position(|&l| l.label == label) {
                    boxes[box_idx].remove(lens_idx);
                }
            },
            Insert(lens) => {
                let box_idx = holiday_helper_hash(lens.label) as usize;
                if let Some(prev_lens) = boxes[box_idx].iter_mut().find(|l| l.label == lens.label) {
                    *prev_lens = lens;
                } else {
                    boxes[box_idx].push(lens);
                }
            }
        }
    });
    // dbg!(&boxes);
    power(&boxes).to_string()
}

fn power(boxes: &[Vec<Lens>]) -> usize {
    boxes.iter().enumerate().map(|(idx, lenses)| {
        let box_power = idx + 1;
        lenses.iter().enumerate().map(|(lens_idx, lens)| {
            let lens_position = lens_idx + 1;
            box_power * lens_position * lens.focal_length
        }).sum::<usize>()
    }).sum::<usize>()
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
    fn hash_algo() {
        let sample = "rn=1";
        let expected = Instruction::Insert(Lens{label: "rn", focal_length: 1});

        assert_eq!(expected, Instruction::from(sample));
    }
    #[test]
    fn example_part2() {
        let sample = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!("145", part2(sample));
    }
}
