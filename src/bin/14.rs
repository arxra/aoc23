use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug)]
enum Dirr {
    North,
    South,
    East,
    West,
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|a| a.chars().collect_vec()).collect_vec()
}

fn print(map: &[Vec<char>]) {
    eprintln!(" calculating load ");
    eprintln!("--------STATE------------");
    map.iter()
        .for_each(|a| eprintln!("{}", a.iter().collect::<String>()));
    eprintln!("---------END-------------");
}

fn tilt(map: &mut Vec<Vec<char>>, direction: Dirr) {
    let width = map[0].len();
    let mut rows = (0..map.len()).collect_vec();
    let mut columns = (0..width).collect_vec();

    if let Dirr::North = direction {
        rows = rows.into_iter().rev().collect_vec();
    }
    if let Dirr::West = direction {
        columns = columns.into_iter().rev().collect_vec();
    }

    match direction {
        Dirr::North => {
            for row in rows.into_iter() {
                for c in columns.iter() {
                    if map[row][*c] == 'O' {
                        let mut upper = row;
                        let mut last_seen_free = row;
                        while map[upper][*c] != '#' {
                            if let Some(inner) = upper.checked_sub(1) {
                                upper = inner;
                            } else {
                                break;
                            }
                            if map[upper][*c] == '.' {
                                last_seen_free = upper;
                            }
                        }
                        if last_seen_free != row {
                            map[row][*c] = '.';
                            map[last_seen_free][*c] = 'O';
                        }
                    }
                }
            }
        }
        Dirr::South => {
            for row in rows.into_iter() {
                for c in columns.iter() {
                    if map[row][*c] == 'O' {
                        let mut upper = row;
                        let mut last_seen_free = row;
                        while map[upper][*c] != '#' {
                            upper += 1;
                            if upper == map.len() {
                                break;
                            }
                            if map[upper][*c] == '.' {
                                last_seen_free = upper;
                            }
                        }
                        if last_seen_free != row {
                            map[row][*c] = '.';
                            map[last_seen_free][*c] = 'O';
                        }
                    }
                }
            }
        }
        Dirr::West => {
            for c in columns.into_iter() {
                for row in rows.iter() {
                    if map[*row][c] == 'O' {
                        let mut upper = c;
                        let mut last_seen_free = c;
                        while map[*row][upper] != '#' {
                            if let Some(inner) = upper.checked_sub(1) {
                                upper = inner;
                            } else {
                                break;
                            }
                            if map[*row][upper] == '.' {
                                last_seen_free = upper;
                            }
                        }
                        if last_seen_free != c {
                            map[*row][c] = '.';
                            map[*row][last_seen_free] = 'O';
                        }
                    }
                }
            }
        }
        Dirr::East => {
            for c in columns.into_iter() {
                for row in rows.iter() {
                    if map[*row][c] == 'O' {
                        let mut next = c;
                        let mut last_seen_free = c;
                        while map[*row][next] != '#' {
                            next += 1;
                            if next == width {
                                break;
                            }
                            if map[*row][next] == '.' {
                                last_seen_free = next;
                            }
                        }
                        if last_seen_free != c {
                            map[*row][c] = '.';
                            map[*row][last_seen_free] = 'O';
                        }
                    }
                }
            }
        }
    }
}

fn calculate_load(input: &[Vec<char>]) -> u32 {
    print(input);
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(index, row)| {
            row.iter()
                .filter(|a| **a == 'O')
                // .inspect(|found| eprintln!("found {found} on this row"))
                .count()
                * (index + 1)
        })
        .sum::<usize>() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = parse(input);
    tilt(&mut input, Dirr::North);
    Some(calculate_load(&input))
}

fn run_round(input: &mut Vec<Vec<char>>, max_rounds: usize) {
    let mut round: usize = 0;
    let mut seen_rounds = HashMap::new();
    while round < max_rounds {
        for dir in [Dirr::North, Dirr::West, Dirr::South, Dirr::East] {
            tilt(input, dir);
        }
        if let Some(cycle) = seen_rounds.insert(input.clone(), round) {
            dbg!(round, cycle);
            round = max_rounds - (max_rounds - round).rem_euclid(round - cycle);
            // round += round - cycle;
        }
        round += 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let round_max = 1000000000;
    let mut input = parse(input);
    run_round(&mut input, round_max);

    Some(calculate_load(&input))
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
    fn test_part_two_mini() {
        let input = "...
#.#
O.O";
        let result = part_two(input);
        assert_eq!(result, Some(3 + 1));
    }

    #[test]
    fn test_part_two_small() {
        let input = "....
##.#
OO.O
##.#
....";
        let result = part_two(input);
        assert_eq!(result, Some(5 + 3 + 1));
    }
    #[test]
    fn test_part_two_small_trans() {
        let input = ".#O#.
.....
.#.#.
.#O#.
.#O#.";
        let expected_frame = ".#.#.
.....
.#.#.
O#.#.
O#O#."
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        let mut frame = parse(input);
        run_round(&mut frame, 10000000);
        assert_eq!(frame, expected_frame);

        let result = part_two(input);
        assert_eq!(result, Some(1 + 2 + 1));
    }

    #[test]
    fn test_part_two_examples() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_two_example_one_cycle() {
        let expected = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        let input = &advent_of_code::template::read_file("examples", DAY);
        let mut state = parse(input);
        run_round(&mut state, 1);

        assert_eq!(state, expected);
    }

    #[test]
    fn test_part_two_example_two_cycles() {
        let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        let input = &advent_of_code::template::read_file("examples", DAY);
        let mut state = parse(input);
        run_round(&mut state, 2);

        assert_eq!(state, expected);
    }

    #[test]
    fn test_part_two_example_three_cycles() {
        let expected = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        let input = &advent_of_code::template::read_file("examples", DAY);
        let mut state = parse(input);
        run_round(&mut state, 3);

        assert_eq!(state, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert!(result > Some(84239));
        assert_eq!(result, None);
    }
}
