use crate::common::{lp1_norm, Direction, Grid};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Track,
    Wall,
    Start,
    End,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Track,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => {
                return Err(());
            }
        })
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    input.parse().unwrap()
}

fn solve(grid: &Grid<Tile>, max_cheat_distance: usize, amount_saved: usize) -> usize {
    let (start, _) = grid
        .pos_iter()
        .filter(|(_, &tile)| tile == Tile::Start)
        .exactly_one()
        .map_err(|_| ())
        .unwrap();
    let (end, _) = grid
        .pos_iter()
        .filter(|(_, &tile)| tile == Tile::End)
        .exactly_one()
        .map_err(|_| ())
        .unwrap();

    let all_paths_from_start = dijkstra_all(&start, |&p| {
        Direction::VALUES
            .iter()
            .map(move |d| d.offset(&p))
            .filter(|&n| grid.in_bounds(&n) && grid[n] != Tile::Wall)
            .map(|p| (p, 1usize))
    });
    assert!(all_paths_from_start.contains_key(&end));
    let all_paths_from_end = dijkstra_all(&end, |&p| {
        Direction::VALUES
            .iter()
            .map(move |d| d.offset(&p))
            .filter(|&n| grid.in_bounds(&n) && grid[n] != Tile::Wall)
            .map(|p| (p, 1usize))
    });
    assert!(all_paths_from_end.contains_key(&start));
    let original_path_length = all_paths_from_start[&end].1;

    [start]
        .into_iter()
        .chain(all_paths_from_start.keys().copied())
        .cartesian_product([end].into_iter().chain(all_paths_from_end.keys().copied()))
        .filter(|&(cheat_start, cheat_end)| {
            let cheat_distance = lp1_norm(&(cheat_end - cheat_start)) as usize;
            if cheat_distance > max_cheat_distance {
                return false;
            }

            let distance_from_start = if cheat_start == start {
                0
            } else {
                all_paths_from_start[&cheat_start].1
            };
            let distance_from_end = if cheat_end == end {
                0
            } else {
                all_paths_from_end[&cheat_end].1
            };

            let new_length = distance_from_start + cheat_distance + distance_from_end;
            new_length <= original_path_length && original_path_length - new_length >= amount_saved
        })
        .count()
}

#[aoc(day20, part1)]
pub fn part1(grid: &Grid<Tile>) -> usize {
    solve(grid, 2, 100)
}

#[aoc(day20, part2)]
pub fn part2(grid: &Grid<Tile>) -> usize {
    solve(grid, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve(&input_generator(INPUT), 2, 2), 44);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&input_generator(INPUT), 20, 50), 285);
    }
}
