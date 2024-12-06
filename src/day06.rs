use crate::common::{Direction, Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    Guard(Direction),
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Empty,
            '#' => Tile::Obstacle,
            c if Direction::try_from(c).is_ok() => Tile::Guard(c.try_into()?),
            _ => {
                return Err(());
            }
        })
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    input.parse().unwrap()
}

fn get_path(grid: &Grid<Tile>) -> (FxHashSet<(Vec2i, Direction)>, bool) {
    let (start_pos, start_dir) = grid
        .pos_iter()
        .find_map(|(pos, tile)| {
            if let Tile::Guard(dir) = tile {
                Some((pos, *dir))
            } else {
                None
            }
        })
        .unwrap();

    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut all_pos = FxHashSet::default();
    let out_of_bounds = 'outer: loop {
        if !all_pos.insert((pos, dir)) {
            break 'outer false;
        }

        let initial_dir = dir;
        loop {
            let target = dir.offset(&pos);
            if !grid.in_bounds(&target) {
                break 'outer true;
            }

            if grid[target] == Tile::Obstacle {
                dir = dir.rotate_cw();
                if dir == initial_dir {
                    // we got stuck, can only happen if start pos is surrounded by obstacles
                    // -> treat this as a loop
                    break 'outer false;
                }
            } else {
                pos = target;
                break;
            }
        }
    };

    (all_pos, out_of_bounds)
}

#[aoc(day6, part1)]
pub fn part1(grid: &Grid<Tile>) -> usize {
    let (path, out_of_bounds) = get_path(grid);
    assert!(out_of_bounds);
    path.iter().map(|(pos, _)| *pos).unique().count()
}

#[aoc(day6, part2)]
pub fn part2(grid: &Grid<Tile>) -> usize {
    let (path, out_of_bounds) = get_path(grid);
    assert!(out_of_bounds);

    let mut loops = 0;
    let mut grid = grid.clone();
    for obstacle_candidate in path.iter().map(|(pos, _)| *pos).unique() {
        if grid[obstacle_candidate] == Tile::Empty {
            grid[obstacle_candidate] = Tile::Obstacle;

            if let (_, false) = get_path(&grid) {
                loops += 1;
            }

            grid[obstacle_candidate] = Tile::Empty;
        }
    }

    loops
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 6);
    }
}
