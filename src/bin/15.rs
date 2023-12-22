#![feature(if_let_guard)]
#![feature(ascii_char)]

use itertools::Itertools;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::alpha1,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(15);

#[derive(Debug)]
enum Op {
    Equals(u32),
    Dash,
}

impl From<&str> for Op {
    fn from(s: &str) -> Op {
        let mut chars = s.chars();
        match chars.next() {
            Some('-') => Self::Dash,
            Some('=') => {
                let int = chars
                    .next()
                    .expect("Next char to exist")
                    .to_digit(10)
                    .unwrap();
                Self::Equals(int)
            }
            _ => unimplemented!("Not a valid op"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    label: Vec<char>,
    operator: Op,
}

impl From<(&str, &str)> for Instruction {
    fn from(value: (&str, &str)) -> Self {
        Instruction {
            label: value.0.chars().collect_vec(),
            operator: value.1.into(),
        }
    }
}
impl From<&Instruction> for usize {
    fn from(value: &Instruction) -> Self {
        value
            .label
            .iter()
            .flat_map(|c| c.as_ascii())
            .map(|a| a.to_u8())
            .fold(0, |acc, b| ((acc + b as usize) * 17).rem_euclid(256))
    }
}

#[derive(Debug)]
struct Lens {
    label: Vec<char>,
    strength: u32,
}

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

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, res) = separated_list1(tag(","), tuple((alpha1, is_a("=-0123456789"))))(input)?;
    let res = res.into_iter().map_into().collect_vec();
    Ok((input, res))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, instructions) = parse(input).unwrap();
    let mut boxes: Vec<Vec<Lens>> = Vec::with_capacity(256);
    (0..256).for_each(|_| boxes.push(Vec::new()));

    for inst in instructions.into_iter() {
        let box_id = usize::from(&inst);
        let box_pos: Option<usize> = boxes
            .get(box_id)
            .iter()
            .flat_map(|inner| (**inner).iter().find_position(|l| l.label == inst.label))
            .next()
            .map(|(a, _)| a);

        match inst.operator {
            Op::Equals(strength) => match box_pos {
                Some(pos) => {
                    boxes[box_id][pos] = Lens {
                        label: inst.label,
                        strength,
                    }
                }
                None => boxes[box_id].push(Lens {
                    label: inst.label,
                    strength,
                }),
            },
            Op::Dash if let Some(a)= box_pos => { boxes.get_mut(box_id).unwrap().remove(a); },
            _ => continue,
        }
    }
    let res = boxes
        .into_iter()
        .enumerate()
        .map(|(ind, lenscol)| {
            lenscol
                .into_iter()
                .enumerate()
                .map(|(box_ind, l)| l.strength * (box_ind as u32 + 1) * (ind as u32 + 1))
                .sum::<u32>()
        })
        .sum();
    Some(res)
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
        assert_eq!(result, Some(145));
    }
}
