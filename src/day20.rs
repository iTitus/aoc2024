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

fn solve(grid: &Grid<Tile>, max_cheat_distance: usize, min_amount_saved: usize) -> usize {
    let (end, _) = grid
        .pos_iter()
        .filter(|(_, &tile)| tile == Tile::End)
        .exactly_one()
        .map_err(|_| ())
        .unwrap();

    let all_distances_from_end = dijkstra_all(&end, |&p| {
        Direction::VALUES
            .iter()
            .map(move |d| d.offset(&p))
            .filter(|&n| grid.in_bounds(&n) && grid[n] != Tile::Wall)
            .map(|p| (p, 1usize))
    })
    .into_iter()
    .map(|(v, (_, d))| (v, d))
    .chain([(end, 0)])
    .collect_vec();

    /*
    real_dist = |start->cheat_start| + |cheat_start->cheat_end| + |cheat_end->end|
              = |start->cheat_start| + |cheat_start->end|
    cheat_dist = |start->cheat_start| + lp1(cheat_start, cheat_end) + |cheat_end->end|

        real_dist - cheat_dist >= min_amount_saved
    <=> |start->cheat_start| + |cheat_start->end| - |start->cheat_start| - lp1(cheat_start, cheat_end) - |cheat_end->end| >= min_amount_saved
    <=> |cheat_start->end| - lp1(cheat_start, cheat_end) - |cheat_end->end| >= min_amount_saved
    <=> |cheat_start->end| - |cheat_end->end| >= min_amount_saved + lp1(cheat_start, cheat_end)

    We could use cartesian_product to iterate over every pair of reachable...
    But using tuple_combinations we get each (a, b) and (b, a) exactly once which is fine:
    Only one pair will have a non-negative lhs, so we use abs_diff to check both at once.
    */
    all_distances_from_end
        .into_iter()
        .tuple_combinations()
        .filter(|&((p1, d1), (p2, d2))| {
            let cheat_distance = lp1_norm(&(p1 - p2)) as usize;
            cheat_distance <= max_cheat_distance
                && d1.abs_diff(d2) >= cheat_distance + min_amount_saved
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
