advent_of_code::solution!(1);

fn str_to_int(num: String) -> u32 {
    let resnum = num.to_owned();
    let num = format!(
        "{}{}",
        resnum.chars().next().unwrap(),
        resnum.chars().last().unwrap()
    );
    num.parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect())
        .map(str_to_int)
        .sum::<u32>()
        .into()
}

fn parse_int(input: &str) -> (&str, Option<char>) {
    let ch = match input {
        c if c.starts_with("one") => Some('1'),
        c if c.starts_with("two") => Some('2'),
        c if c.starts_with("three") => Some('3'),
        c if c.starts_with("four") => Some('4'),
        c if c.starts_with("five") => Some('5'),
        c if c.starts_with("six") => Some('6'),
        c if c.starts_with("seven") => Some('7'),
        c if c.starts_with("eight") => Some('8'),
        c if c.starts_with("nine") => Some('9'),
        c if c.chars().next().unwrap().is_ascii_digit() => Some(c.chars().next().unwrap()),
        _ => None,
    };
    (&input[1..], ch)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;
    for mut l in input.lines() {
        let mut ints = String::new();
        while !l.is_empty() {
            match parse_int(l) {
                (c, None) => l = c,
                (c, Some(num)) => {
                    ints.push(num);
                    l = c;
                }
            }
        }
        let new_int = str_to_int(ints);
        res += new_int;
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(53921));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.unwrap() < 54678);
        assert_eq!(result, Some(54676));
    }
}
