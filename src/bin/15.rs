#![feature(ascii_char)]

advent_of_code::solution!(15);
pub fn part_one(input: &str) -> Option<u32> {
    input
        .split(',')
        .map(|l| {
            l.chars()
                .flat_map(|c| c.as_ascii())
                .map(|a| a.to_u8())
                .fold(0, |acc, b| ((acc + b as u32) * 17).rem_euclid(256))
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    //rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
