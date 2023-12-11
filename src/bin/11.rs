use itertools::Itertools;

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    // dbg!(&map.len(), map[0].len());
    map
}

fn expanding_dimensions(map: &mut Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let empty_rows = map
        .iter()
        .enumerate()
        .filter(|(_, l)| !l.contains(&'#'))
        .map(|(c, _)| c)
        .collect_vec();
    let mut empty_columns = Vec::new();
    'a: for i in 0..map[0].len() {
        for ii in 0..map.len() {
            if map[ii][i] == '#' {
                continue 'a;
            }
        }
        empty_columns.push(i);
    }
    (empty_columns, empty_rows)
}

fn distance(x: usize, xx: usize, range: &[usize], expansion_factor: usize) -> u64 {
    let min = x.min(xx);
    let max = x.max(xx);
    let res = max - min + ((min + 1..max).filter(|i| range.contains(i)).count() * expansion_factor);
    res as u64
}

fn solve(
    map: Vec<Vec<char>>,
    expanding_dimensions: (Vec<usize>, Vec<usize>),
    expansion_factor: usize,
) -> u64 {
    // Find the position of all galaxies
    let mut galaxies = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }
    galaxies
        .into_iter()
        .combinations(2)
        .flat_map(|pair| pair.into_iter().tuple_windows())
        .map(|((y, x), (yy, xx))| {
            distance(y, yy, &expanding_dimensions.1, expansion_factor)
                + distance(x, xx, &expanding_dimensions.0, expansion_factor)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = parse(input);
    let ed = expanding_dimensions(&mut map);
    let res = solve(map, ed, 1);

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = parse(input);
    let ed = expanding_dimensions(&mut map);
    let res = solve(map, ed, 1_000_000 - 1);

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(9329143));
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }
    #[test]
    fn test_factor_ten() {
        let mut map = parse(&advent_of_code::template::read_file("examples", DAY));
        let ed = expanding_dimensions(&mut map);
        let res = solve(map, ed, 10 - 1);
        assert_eq!(res, 1030);
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("inputs", DAY)).unwrap();
    //     assert!(result < 710675618476);
    // }
}
