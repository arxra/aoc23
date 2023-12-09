use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    character::complete::{self, alphanumeric1, space1},
    IResult,
};

advent_of_code::solution!(7);

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<u32>,
    counts: Vec<usize>,
    rank: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self, &other) {
            (s, o) if s.counts == o.counts => {
                let mut cmp = Ordering::Equal;
                for i in 0..5 {
                    cmp = s.cards[i].cmp(&o.cards[i]);
                    if cmp != Ordering::Equal {
                        break;
                    }
                }
                cmp
            }
            (s, o) if s.counts.len() == o.counts.len() => s.counts[0].cmp(&o.counts[0]),
            (s, o) if s.counts.len() != o.counts.len() => {
                s.counts.len().cmp(&o.counts.len()).reverse()
            }
            (s, o) => unimplemented!("{s:?}, {o:?}"),
        }
    }
}

fn parse_card(input: char) -> u32 {
    match input {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        c if c.is_numeric() => c.to_digit(10).unwrap(),
        c => unreachable!("Unknown char found: {c}"),
    }
}
fn parse_card_v2(input: char) -> u32 {
    match input {
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        c if c.is_numeric() => c.to_digit(10).unwrap(),
        c => unreachable!("Unknown char found: {c}"),
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = alphanumeric1(input)?;
    let cards: Vec<u32> = cards.chars().map(parse_card).collect();
    let counts = cards.iter().counts().into_values().sorted().rev().collect();
    let (input, _) = space1(input)?;
    let (input, rank) = complete::u32(input)?;
    Ok((
        input,
        Hand {
            cards,
            rank,
            counts,
        },
    ))
}

fn parse_hand_v2(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = alphanumeric1(input)?;
    let cards: Vec<u32> = cards.chars().map(parse_card_v2).collect();
    let mut counts: Vec<usize> = cards
        .iter()
        .filter(|a| **a != 0)
        .counts()
        .into_values()
        .sorted()
        .rev()
        .collect();

    let jokers = cards.iter().filter(|a| **a == 0).count();
    match counts.get_mut(0) {
        Some(a) => *a += jokers,
        None => counts.push(jokers),
    }

    //cards.iter().filter(|a| **a == 0).count();

    let (input, _) = space1(input)?;
    let (input, rank) = complete::u32(input)?;
    Ok((
        input,
        Hand {
            cards,
            rank,
            counts,
        },
    ))
}

fn game(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(parse_hand)
        .filter_map(|a| a.ok())
        .map(|(_, a)| a)
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let game = game(input);
    // dbg!(game);
    let res = game
        .iter()
        .sorted()
        // .inspect(|v| eprintln!("{v:?}"))
        .enumerate()
        .map(|(e, hand)| (e as u32 + 1) * hand.rank)
        .sum();
    //
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let game = input
        .lines()
        .map(parse_hand_v2)
        .filter_map(|a| a.ok())
        .map(|(_, a)| a)
        .sorted()
        .enumerate()
        .map(|(e, hand)| (e as u32 + 1) * hand.rank)
        .sum();

    Some(game)
}

#[cfg(test)]
mod tests {
    use super::*;
    // too low: 248721746
    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result.unwrap() > 248721746);
        assert!(result.unwrap() > 248852321);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
