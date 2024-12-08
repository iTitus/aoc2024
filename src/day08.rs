use crate::common::{Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Integer;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Antenna(char),
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Empty,
            c if c.is_ascii_alphanumeric() => Tile::Antenna(c),
            _ => {
                return Err(());
            }
        })
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> (Grid<Tile>, HashMap<char, Vec<Vec2i>>) {
    let grid: Grid<Tile> = input.parse().unwrap();
    let antennas = grid
        .pos_iter()
        .filter_map(|(pos, tile)| {
            if let Tile::Antenna(freq) = tile {
                Some((*freq, pos))
            } else {
                None
            }
        })
        .into_group_map();
    (grid, antennas)
}

fn count_antinodes<const PART2: bool>(input: &(Grid<Tile>, HashMap<char, Vec<Vec2i>>)) -> usize {
    input
        .1
        .values()
        .flat_map(|antennas| {
            antennas.iter().tuple_combinations().flat_map(|(&a, &b)| {
                assert_ne!(a, b);
                let dir = if PART2 {
                    let dir = b - a;
                    let gcd = dir.x.gcd(&dir.y);
                    Vec2i::new(dir.x / gcd, dir.y / gcd)
                } else {
                    b - a
                };

                // both ranges need to be of the same type
                let range = if PART2 { 0..i64::MAX } else { 1..2 };
                range
                    .clone()
                    .map(move |i| a - i * dir)
                    .take_while(|pos| input.0.in_bounds(pos))
                    .chain(
                        range
                            .map(move |i| a + (i + 1) * dir)
                            .take_while(|pos| input.0.in_bounds(pos)),
                    )
            })
        })
        .unique()
        .count()
}

#[aoc(day8, part1)]
pub fn part1(input: &(Grid<Tile>, HashMap<char, Vec<Vec2i>>)) -> usize {
    count_antinodes::<false>(input)
}

#[aoc(day8, part2)]
pub fn part2(input: &(Grid<Tile>, HashMap<char, Vec<Vec2i>>)) -> usize {
    count_antinodes::<true>(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 34);
    }
}
