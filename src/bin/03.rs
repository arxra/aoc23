use std::collections::HashSet;

advent_of_code::solution!(3);

fn is_engine_pars(char: char) -> bool {
    !(char == '.' || char.is_ascii_digit())
}

fn get_engine_numbers(input: &str) -> Vec<u32> {
    let mut res = Vec::new();

    // make input into array
    let input: Vec<Vec<char>> = input.lines().map(|p| p.chars().collect()).collect();
    let x_max = input[0].len();
    let y_max = input.len();

    let mut currrent_string = String::new();
    let mut current_is_adjecent = false;
    for y in 0..y_max {
        for x in 0..x_max {
            if input[y][x].is_ascii_digit() {
                currrent_string.push(input[y][x]);

                // We don't know if there is a enginet yet. Check.
                if (x > 0 && is_engine_pars(input[y][x - 1]))
                    || (y > 0 && is_engine_pars(input[y - 1][x]))
                    || (x < x_max - 1 && is_engine_pars(input[y][x + 1]))
                    || (y < y_max - 1 && is_engine_pars(input[y + 1][x]))
                    || (y < y_max - 1 && x < x_max - 1 && is_engine_pars(input[y + 1][x + 1]))
                    || (y > 0 && x > 0 && is_engine_pars(input[y - 1][x - 1]))
                    || (y > 0 && x < x_max - 1 && is_engine_pars(input[y - 1][x + 1]))
                    || (y < y_max - 1 && x > 0 && is_engine_pars(input[y + 1][x - 1]))
                {
                    current_is_adjecent = true;
                }
            }

            if !input[y][x].is_ascii_digit() || x == x_max - 1 {
                if !currrent_string.is_empty() && current_is_adjecent {
                    let engine_part = currrent_string.parse().unwrap();
                    res.push(engine_part);

                    // Clear variables for new enginge part
                    currrent_string.clear();
                    current_is_adjecent = false;
                } else if !currrent_string.is_empty() {
                    // Clear variables for new enginge part
                    dbg!(&currrent_string, "not part of engine!");
                    currrent_string.clear();
                }
                // else we were not parsing a number and have nothing to do before new row
            }
        }
    }

    res
}

fn parse_digit_around(x: usize, y: usize, input: &[Vec<char>]) -> u32 {
    let x_max = input[0].len();
    let mut x = x;
    let mut xx = x;
    while xx > 0 && input[y][xx - 1].is_ascii_digit() {
        xx -= 1;
    }

    while x < x_max - 1 && input[y][x + 1].is_ascii_digit() {
        x += 1;
    }
    let num_str = input[y][xx..=x].iter().collect::<String>();

    num_str.parse::<u32>().unwrap()
}

fn get_gears(input: &str) -> u32 {
    let mut res = 0;

    // make input into array
    let input: Vec<Vec<char>> = input.lines().map(|p| p.chars().collect()).collect();
    let x_max = input[0].len();
    let y_max = input.len();

    for y in 0..y_max {
        for x in 0..x_max {
            if input[y][x] == '*' {
                // Find neighbor nums
                let mut num_set = HashSet::new();

                if x > 0 && (input[y][x - 1]).is_ascii_digit() {
                    num_set.insert(parse_digit_around(x - 1, y, &input));
                }
                if y > 0 && (input[y - 1][x]).is_ascii_digit() {
                    num_set.insert(parse_digit_around(x, y - 1, &input));
                }
                if x < x_max - 1 && input[y][x + 1].is_ascii_digit() {
                    num_set.insert(parse_digit_around(x + 1, y, &input));
                }
                if y < y_max - 1 && input[y + 1][x].is_ascii_digit() {
                    num_set.insert(parse_digit_around(x, y + 1, &input));
                }
                if y < y_max - 1 && x < x_max - 1 && input[y + 1][x + 1].is_ascii_digit() {
                    num_set.insert(parse_digit_around(x + 1, y + 1, &input));
                }
                if y > 0 && x > 0 && input[y - 1][x - 1].is_ascii_digit() {
                    num_set.insert(parse_digit_around(x - 1, y - 1, &input));
                }
                if y > 0 && x < x_max - 1 && input[y - 1][x + 1].is_ascii_digit() {
                    num_set.insert(parse_digit_around(x + 1, y - 1, &input));
                }
                if y < y_max - 1 && x > 0 && input[y + 1][x - 1].is_ascii_digit() {
                    num_set.insert(parse_digit_around(x - 1, y + 1, &input));
                }
                if num_set.len() != 2 {
                    continue;
                }
                res += num_set.iter().product::<u32>();
            }
        }
    }
    res
}
pub fn part_one(input: &str) -> Option<u32> {
    let engine_numbers = get_engine_numbers(input);
    dbg!(&engine_numbers);
    Some(engine_numbers.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_gears(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4556));
    }
    #[test]
    fn test_part_res_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));

        assert_ne!(result, Some(849187));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
