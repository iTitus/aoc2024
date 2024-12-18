use crate::common::{parse_vec, Direction, Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::*;
use tinyvec::array_vec;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Vec2i> {
    input.lines().map(|l| parse_vec(l).unwrap()).collect()
}

fn solve_part1(input: &[Vec2i], size_x: usize, size_y: usize) -> Option<usize> {
    let mut grid = Grid::new_from_element(size_x, size_y, Tile::Empty);
    for &v in input {
        grid[v] = Tile::Wall;
    }

    bfs(
        &Vec2i::new(0, 0),
        |p| {
            let mut vec = array_vec!([Vec2i; 4]);
            for dir in Direction::VALUES {
                let next = dir.offset(p);
                if grid.in_bounds(&next) && grid[next] == Tile::Empty {
                    vec.push(next);
                }
            }

            vec
        },
        |&p| p == Vec2i::new(size_x as i64 - 1, size_y as i64 - 1),
    )
    .map(|path| path.len() - 1)
}

#[aoc(day18, part1)]
pub fn part1(input: &[Vec2i]) -> usize {
    solve_part1(&input[..1024], 71, 71).unwrap()
}

fn solve_part2(input: &[Vec2i], size_x: usize, size_y: usize) -> Vec2i {
    let mut min = 0;
    let mut max = input.len() - 1;
    while min <= max {
        let mid = (max + min) / 2;
        let has_path = solve_part1(&input[..=mid], size_x, size_y).is_some();
        if has_path {
            min = mid + 1;
        } else if min == max {
            return input[mid];
        } else {
            max = mid - 1;
        }
    }

    unreachable!();
}

#[aoc(day18, part2)]
pub fn part2(input: &[Vec2i]) -> String {
    let v = solve_part2(input, 71, 71);
    format!("{},{}", v.x, v.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)[..12], 7, 7), Some(22));
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT), 7, 7), Vec2i::new(6, 1));
    }
}
