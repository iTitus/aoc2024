use crate::common::{Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use num::ToPrimitive;
use pathfinding::prelude::*;
use tinyvec::ArrayVec;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Elevation(u8);

impl TryFrom<char> for Elevation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value
            .to_digit(10)
            .and_then(|n| n.to_u8())
            .map(Elevation)
            .ok_or(())
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> (Grid<Elevation>, Vec<Vec2i>) {
    let grid: Grid<Elevation> = input.parse().unwrap();
    let trailheads: Vec<Vec2i> = grid
        .pos_iter()
        .filter(|(_, e)| e.0 == 0)
        .map(|(p, _)| p)
        .collect();
    (grid, trailheads)
}

fn neighbors(grid: &Grid<Elevation>, pos: Vec2i) -> impl IntoIterator<Item = Vec2i> {
    let next = grid[pos].0 + 1;
    [
        Vec2i::new(1, 0),
        Vec2i::new(-1, 0),
        Vec2i::new(0, 1),
        Vec2i::new(0, -1),
    ]
    .into_iter()
    .map(|dir| pos + dir)
    .filter(|pos| grid.in_bounds(pos) && grid[*pos].0 == next)
    .collect::<ArrayVec<[Vec2i; 4]>>()
}

#[aoc(day10, part1)]
pub fn part1((grid, trailheads): &(Grid<Elevation>, Vec<Vec2i>)) -> usize {
    let mut score = 0;
    for t in trailheads {
        let reachable = bfs_reach(*t, |&pos| neighbors(grid, pos));
        score += reachable.filter(|&p| grid[p].0 == 9).count();
    }

    score
}

#[aoc(day10, part2)]
pub fn part2((grid, trailheads): &(Grid<Elevation>, Vec<Vec2i>)) -> usize {
    let mut rating = 0;
    for t in trailheads {
        let paths = count_paths(*t, |&pos| neighbors(grid, pos), |&pos| grid[pos].0 == 9);
        rating += paths;
    }

    rating
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
    const CHALLENGE: &str = include_str!("../alternative_inputs/day10.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 81);
    }

    #[test]
    fn test_part1_challenge() {
        assert_eq!(part1(&input_generator(CHALLENGE)), 464);
    }

    #[test]
    fn test_part2_challenge() {
        assert_eq!(part2(&input_generator(CHALLENGE)), 16451);
    }
}
