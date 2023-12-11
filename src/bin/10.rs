use std::collections::BTreeMap;

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Tile {
    distance: Option<u32>,
    section: char,
}

impl From<char> for Tile {
    fn from(section: char) -> Self {
        Self {
            distance: None,
            section,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| l.chars().map_into().collect_vec())
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    let (start_y, start_line_tiles) = map
        .iter()
        .find_position(|inner| {
            inner
                .iter()
                .find_position(|tile| tile.section == 'S')
                .is_some()
        })
        .expect("Did not find line with starting pos");
    let start_x = start_line_tiles
        .iter()
        .position(|tile| tile.section == 'S')
        .expect("to find the starting node X position");
    dbg!(&start_y, &start_x);

    let mut next_nodes = BTreeMap::new();
    map[start_y][start_x].distance = Some(0);
    next_nodes.insert(0, vec![(start_y as i32, start_x as i32)]);

    while let Some((dist, list)) = next_nodes.pop_first() {
        for (y, x) in list {
            eprintln!("Processing node: {:?}", map[y as usize][x as usize]);
            for (yy, xx) in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                eprintln!("{dist}:({},{})", yy, xx);
                if yy < 0
                    || xx < 0
                    || map.len() as i32 == yy
                    || map[0].len() as i32 == xx
                    || map[yy as usize][xx as usize].distance.is_some()
                {
                    // dbg!("outside map or distance already set!");
                    continue;
                }
                // Map current node and check if its connected
                match (
                    map[y as usize][x as usize].section,
                    map[yy as usize][xx as usize].section,
                ) {
                    (a, b)
                        if (xx < x
                            && ['S', '-', 'J', '7'].contains(&a)
                            && ['-', 'L', 'F'].contains(&b))
                            || (yy < y
                                && ['S', '|', 'J', 'L'].contains(&a)
                                && ['|', '7', 'F'].contains(&b))
                            || (yy > y
                                && ['S', '|', '7', 'F'].contains(&a)
                                && ['|', 'J', 'L'].contains(&b))
                            || (xx > x
                                && ['S', '-', 'L', 'F'].contains(&a)
                                && ['-', 'J', '7'].contains(&b)) =>
                    {
                        let new_dist = dist + 1;
                        // (yy, xx)
                        if let Some(v) = next_nodes.get_mut(&new_dist) {
                            v.push((yy, xx));
                        } else {
                            next_nodes.insert(new_dist, vec![(yy, xx)]);
                        }
                        map[yy as usize][xx as usize].distance = Some(new_dist);
                        eprintln!("found new home for ({a},{b}) in {yy},{xx}");
                    }

                    a => (), //eprintln!("Not matched: {a:?}"),
                }
            }
        }
    }
    map.into_iter()
        .flat_map(|l| l.into_iter().flat_map(|t| t.distance).max())
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
