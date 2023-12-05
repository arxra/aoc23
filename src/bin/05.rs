use itertools::Itertools;
use nom::{
    character::complete::{multispace1, newline, space1, u64},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use rayon::prelude::*;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Game {
    rounds: Vec<Vec<(u64, u64, u64)>>,
    seeds: Vec<u64>,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, seeds) = separated_list1(space1, u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, rounds) = separated_list1(
        tuple((newline, newline)),
        separated_list1(newline, tuple((u64, space1, u64, space1, u64))),
    )(input)?;

    dbg!(rounds.len());

    let rounds = rounds
        .iter()
        .map(|inner| inner.iter().map(|(a, _, b, _, c)| (*a, *b, *c)).collect())
        .collect();
    Ok((input, Game { rounds, seeds }))
}

fn solve(game: Game) -> Option<u64> {
    let min = game
        .seeds
        .into_par_iter()
        .map(|mut current| {
            'b: for round in game.rounds.iter() {
                for (dest, src, dist) in round {
                    if current >= *src && current < *src + *dist {
                        current = current + dest - src;
                        continue 'b;
                    }
                }
            }
            current
        })
        .min()
        .unwrap();
    Some(min)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, game) = parse_game(input).unwrap();
    solve(game)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut game) = parse_game(input).unwrap();
    game.seeds = game
        .seeds
        .into_iter()
        .tuples()
        .flat_map(|(start, range)| start..start + range)
        .collect();
    solve(game)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(240320250));
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
        //28062336
    }
}
