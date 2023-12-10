#![feature(iter_map_windows)]

use rayon::prelude::*;

use nom::{
    character::complete::{i64, space1},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(9);

fn parse_linei(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, i64)(input)
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .flat_map(parse_linei)
        .map(|(_, a)| a)
        .collect()
}
fn solve(inputs: Vec<Vec<i64>>) -> i64 {
    let mut res = 0;
    for mut input in inputs {
        let mut extra_lines = Vec::with_capacity(input.len() - 1);
        extra_lines.push(input.clone());
        // generate all base lines
        while input.iter().any(|p| *p != 0) {
            input = extra_lines[extra_lines.len() - 1]
                .iter()
                .map_windows(|[a, b]| *b - *a)
                .collect();
            extra_lines.push(input.clone());
        }
        // Add the last 0 to the last line
        let resulting_lines = extra_lines.len();
        extra_lines[resulting_lines - 1].push(0);

        // Add new number to each line after the last row of 0's is done'
        for line in (0..extra_lines.len() - 1).rev() {
            let new_pos = extra_lines[line].len();
            let projected = extra_lines[line][new_pos - 1] + extra_lines[line + 1][new_pos - 1];
            extra_lines[line].push(projected);
        }
        dbg!(&extra_lines);
        // Add the result
        res += extra_lines[0].iter().last().unwrap();
    }
    res
}

pub fn part_one(input: &str) -> Option<i64> {
    let inputs = parse(input);
    Some(solve(inputs))
}

pub fn part_two(input: &str) -> Option<i64> {
    let inputs = parse(input)
        .into_par_iter()
        .map(|a| a.into_iter().rev().collect())
        .collect();
    Some(solve(inputs))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result > Some(1516292712));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
