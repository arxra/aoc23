use itertools::Itertools;

advent_of_code::solution!(14);

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|a| a.chars().collect_vec()).collect_vec()
}

fn tilt(map: &mut Vec<Vec<char>>) {
    let width = map[0].len();
    for row in (0..map.len()).rev() {
        for c in 0..width {
            if map[row][c] == 'O' {
                let mut upper = row;
                let mut last_seen_free = row;
                while map[upper][c] != '#' {
                    if let Some(inner) = upper.checked_sub(1) {
                        upper = inner;
                    } else {
                        break;
                    }
                    if map[upper][c] == '.' {
                        last_seen_free = upper;
                    }
                }
                if last_seen_free != row {
                    map[row][c] = '.';
                    map[last_seen_free][c] = 'O';
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = parse(input);
    // eprintln!("{:?}", input);
    tilt(&mut input);
    // eprintln!("{:?}", input);
    let res = input
        .into_iter()
        .rev()
        .enumerate()
        .map(|(index, row)| row.iter().filter(|a| **a == 'O').count() * (index + 1))
        .sum::<usize>() as u32;

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
