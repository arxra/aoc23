use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{space1, u32},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Card {
    winners: Vec<u32>,
    numbers: Vec<u32>,
    amount: usize,
}

fn winners(c: &Card) -> u32 {
    set(&c.winners).intersection(&set(&c.numbers)).count() as u32
}

fn card_parser(input: &str) -> IResult<&str, Card> {
    let (input, _) = tuple((tag("Card"), space1, u32, tag(":"), space1))(input)?;
    dbg!(input);
    let (input, (winners, _, numbers)) = tuple((
        separated_list1(space1, u32),
        tuple((space1, tag("|"), space1)),
        separated_list1(space1, u32),
    ))(input)?;
    Ok((
        input,
        Card {
            winners,
            numbers,
            amount: 1,
        },
    ))
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .flat_map(card_parser)
        .map(|(_, c)| c)
        .collect::<Vec<Card>>()
}

fn set(v: &[u32]) -> HashSet<u32> {
    HashSet::from_iter(v.iter().cloned())
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_input(input);
    dbg!(&cards);
    let res = cards
        .iter()
        .map(winners)
        .map(|a| if a > 0 { u32::pow(2, a - 1) } else { a })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_map = parse_input(input);
    for a in 0..card_map.len() {
        let winners = winners(&card_map[a]);
        for b in a + 1..(a + winners as usize + 1) {
            card_map[b].amount += card_map[a].amount;
        }
    }

    let total = card_map.iter().map(|c| c.amount as u32).sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
