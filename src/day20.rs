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

fn solve(grid: &Grid<Tile>, max_cheat_length: usize, amount_saved: usize) -> usize {
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

    let all_paths = dijkstra_all(&end, |&p| {
        Direction::VALUES
            .iter()
            .map(move |d| d.offset(&p))
            .filter(|&n| grid.in_bounds(&n) && grid[n] != Tile::Wall)
            .map(|p| (p, 1usize))
    });
    let &(_, original_path_len) = all_paths.get(&start).expect("no path from start to end");

    let mut count = 0;
    let mut cheat_start = start;
    let mut distance_from_start = 0;
    loop {
        let reach = bfs_reach(cheat_start, |&p| {
            Direction::VALUES
                .iter()
                .map(move |d| d.offset(&p))
                .filter(|&n| {
                    lp1_norm(&(n - cheat_start)) as usize <= max_cheat_length && grid.in_bounds(&n)
                })
        });
        for cheat_end in reach.filter(|&p| grid[p] != Tile::Wall) {
            let distance_from_end = if cheat_end == end {
                0
            } else if let Some(&(_, len)) = all_paths.get(&cheat_end) {
                len
            } else {
                continue;
            };

            let cheat_len = lp1_norm(&(cheat_end - cheat_start)) as usize;
            let total_len = distance_from_start + cheat_len + distance_from_end;
            if total_len <= original_path_len && original_path_len - total_len >= amount_saved {
                count += 1;
            }
        }

        if cheat_start == end {
            break;
        } else if let Some(&(parent, _)) = all_paths.get(&cheat_start) {
            cheat_start = parent;
            distance_from_start += 1;
        } else {
            unreachable!("no path from start to end");
        }
    }

    count
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
