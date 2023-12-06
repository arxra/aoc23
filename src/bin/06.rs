use nom::{
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(6);

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, time) = separated_list1(space1, u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, distance) = separated_list1(space1, u64)(input)?;
    Ok((input, (time, distance)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (time, distance)) = parse_input(input).unwrap();
    let mut res = 1;
    for round in 0..time.len() {
        let mut winners = 0;
        for t in 0..time[round] {
            if (time[round] - t) * t > distance[round] {
                winners += 1;
            }
        }
        res *= winners;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_one(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
