use crate::common::{Direction, Grid};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => {
                return Err(());
            }
        })
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    input.parse().unwrap()
}

#[aoc(day16, part1)]
pub fn part1(grid: &Grid<Tile>) -> usize {
    let (start, _) = grid
        .pos_iter()
        .find(|(_, &tile)| tile == Tile::Start)
        .unwrap();

    let (_path, cost) = dijkstra(
        &(start, Direction::East),
        |&(pos, dir)| {
            let mut succ = [
                None,
                Some(((pos, dir.rotate_cw()), 1000)),
                Some(((pos, dir.rotate_ccw()), 1000)),
            ];

            let neighbor = dir.offset(&pos);
            if grid[neighbor] != Tile::Wall {
                succ[0] = Some(((neighbor, dir), 1usize));
            }

            succ.into_iter().flatten()
        },
        |&(pos, _)| grid[pos] == Tile::End,
    )
    .unwrap();

    cost
}

#[aoc(day16, part2)]
pub fn part2(grid: &Grid<Tile>) -> usize {
    let (start, _) = grid
        .pos_iter()
        .find(|(_, &tile)| tile == Tile::Start)
        .unwrap();

    let (paths, _cost) = astar_bag(
        &(start, Direction::East),
        |&(pos, dir)| {
            let mut succ = [
                None,
                Some(((pos, dir.rotate_cw()), 1000)),
                Some(((pos, dir.rotate_ccw()), 1000)),
            ];

            let neighbor = dir.offset(&pos);
            if grid[neighbor] != Tile::Wall {
                succ[0] = Some(((neighbor, dir), 1usize));
            }

            succ.into_iter().flatten()
        },
        |_| 0, // there were no significant performance differences with a heuristic
        |&(pos, _)| grid[pos] == Tile::End,
    )
    .unwrap();

    paths
        .flat_map(|path| path.into_iter().map(|(pos, _)| pos))
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT_1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
    const INPUT_2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 7036);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 11048);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&input_generator(INPUT_1)), 45);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&input_generator(INPUT_2)), 64);
    }
}
