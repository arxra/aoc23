use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace1, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

advent_of_code::solution!(8);

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!("Unknown directive"),
        }
    }
}

#[derive(Debug)]
struct Game<'a> {
    dirs: Vec<Dir>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

fn parse_input(input: &str) -> IResult<&str, Game> {
    let (input, dirs) = alphanumeric1(input)?;
    let dirs: Vec<Dir> = dirs.chars().map_into().collect();
    let (input, _) = multispace1(input)?;

    let (input, nodes) =
        separated_list1(
            newline,
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    char('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    char(')'),
                ),
            ),
        )(input)?;
    let nodes: HashMap<&str, (&str, &str)> = nodes.into_iter().collect();

    // todo!("finish parsing");
    Ok((input, Game { dirs, nodes }))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, input) = parse_input(input).unwrap();
    let mut node = "AAA";
    let mut steps = 0;
    let mut pos = 0;
    while node != "ZZZ" {
        eprintln!("Processing node: {node}");
        dbg!(steps, pos);
        steps += 1;
        node = match input.dirs.get(pos).unwrap() {
            Dir::Left => input.nodes.get(node).unwrap().0,
            Dir::Right => input.nodes.get(node).unwrap().1,
        };
        if pos == input.dirs.len() - 1 {
            pos = 0;
        } else {
            pos += 1;
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, input) = parse_input(input).unwrap();
    let mut nodes: Vec<&str> = input
        .nodes
        .clone()
        .into_keys()
        .filter(|a| a.ends_with('A'))
        .collect();
    // dbg!(&nodes);
    let nodes_len = nodes.len();
    let mut steps = Vec::with_capacity(nodes.len());
    let mut current = 0;
    let mut pos = 0;
    while nodes.iter().any(|a| !a.ends_with('Z')) {
        for i in 0..nodes_len {
            // dbg!(nodes[i], steps, pos);
            nodes[i] = match input.dirs.get(pos).unwrap() {
                Dir::Left => input.nodes.get(nodes[i]).unwrap().0,
                Dir::Right => input.nodes.get(nodes[i]).unwrap().1,
            };
            if nodes[i].ends_with('Z') {
                nodes.swap_remove(i);
                steps.push(current);
            }
        }
        current += 1;
        if pos == input.dirs.len() - 1 {
            pos = 0;
        } else {
            pos += 1;
        }
    }
    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
