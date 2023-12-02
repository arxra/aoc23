advent_of_code::solution!(2);
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{alpha0, digit0, space0},
    },
    multi::separated_list0,
    IResult,
};

#[derive(Debug, Default)]
struct Game {
    //Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    green: u32,
    blue: u32,
    red: u32,
}

impl Game {
    fn is_larger(&self) -> bool {
        self.green > 13 || self.red > 12 || self.blue > 14
    }
}

#[derive(Debug)]
enum Color {
    Green(u32),
    Red(u32),
    Blue(u32),
}

impl Color {
    fn from_str(input: &str, num: u32) -> Self {
        match input {
            "green" => Self::Green(num),
            "red" => Self::Red(num),
            "blue" => Self::Blue(num),
            a => panic!("unallowed color: {a}"),
        }
    }
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = space0(input)?;
    let (input, num) = character::complete::u32(input)?;
    let (input, _) = space0(input)?;
    let (input, raw_color) = alpha0(input)?;
    let color = Color::from_str(raw_color, num);

    Ok((input, color))
}

fn parse_set(input: &str) -> IResult<&str, Vec<Color>> {
    let (input, set) = separated_list0(tag(","), parse_color)(input)?;
    Ok((input, set))
}

fn parse_round(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, _) = digit0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, sets) = separated_list0(tag(";"), parse_set)(input)?;
    let mut green = 0;
    let mut red = 0;
    let mut blue = 0;

    sets.iter()
        .flat_map(|set| set.iter())
        .for_each(|f| match f {
            Color::Green(a) => green = green.max(*a),
            Color::Red(a) => red = red.max(*a),
            Color::Blue(a) => blue = blue.max(*a),
        });

    let game = Game { green, blue, red };
    dbg!(&game);

    Ok((input, game))
}

fn parse_game(input: &str) -> IResult<&str, Vec<Game>> {
    let mut games = Vec::new();
    for l in input.lines() {
        let (_, g) = parse_round(l)?;
        games.push(g);
    }
    Ok((input, games))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, games) = parse_game(input).unwrap();
    let res = games
        .iter()
        .enumerate()
        .filter(|(_, a)| !a.is_larger())
        .map(|(num, _)| (num + 1) as u32)
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, games) = parse_game(input).unwrap();
    let res = games.iter().map(|a| a.green * a.red * a.blue).sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
