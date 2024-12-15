use crate::common::{Direction, Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::cmp::Reverse;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    BoxL,
    BoxR,
    Box,
    Robot,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            _ => {
                return Err(());
            }
        })
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> (Grid<Tile>, Vec<Direction>) {
    let (grid, directions) = input.split_once("\n\n").unwrap();
    (
        grid.parse().unwrap(),
        directions
            .chars()
            .filter_map(|c| Direction::try_from(c).ok())
            .collect(),
    )
}

fn run(grid: &mut Grid<Tile>, directions: &[Direction]) -> i64 {
    let (mut pos, _) = grid
        .pos_iter()
        .find(|(_, &tile)| tile == Tile::Robot)
        .unwrap();
    'outer: for d in directions {
        let mut to_move = FxHashSet::default();
        let mut q = vec![pos];
        while let Some(p) = q.pop() {
            if to_move.contains(&p) {
                continue;
            }

            match grid[p] {
                Tile::Robot | Tile::Box => {
                    to_move.insert(p);
                    q.push(d.offset(&p));
                }
                Tile::BoxL => {
                    let neighbor = Direction::East.offset(&p);
                    to_move.insert(p);
                    to_move.insert(neighbor);
                    q.push(d.offset(&p));
                    q.push(d.offset(&neighbor));
                }
                Tile::BoxR => {
                    let neighbor = Direction::West.offset(&p);
                    to_move.insert(p);
                    to_move.insert(neighbor);
                    q.push(d.offset(&p));
                    q.push(d.offset(&neighbor));
                }
                Tile::Empty => continue,
                Tile::Wall => continue 'outer,
            }
        }

        to_move
            .into_iter()
            .sorted_unstable_by_key(|p| Reverse(d.vec().dot(p)))
            .for_each(|p| {
                let neighbor = d.offset(&p);
                grid[neighbor] = grid[p];
                grid[p] = Tile::Empty;
            });
        pos = d.offset(&pos);
    }

    grid.pos_iter()
        .filter(|(_, &tile)| matches!(tile, Tile::Box | Tile::BoxL))
        .map(|(pos, _)| 100 * pos.y + pos.x)
        .sum()
}

#[aoc(day15, part1)]
pub fn part1((grid, directions): &(Grid<Tile>, Vec<Direction>)) -> i64 {
    let mut grid = grid.clone();
    run(&mut grid, directions)
}

#[aoc(day15, part2)]
pub fn part2((input_grid, directions): &(Grid<Tile>, Vec<Direction>)) -> i64 {
    let mut grid = Grid::new_from_element(2 * input_grid.size_x, input_grid.size_x, Tile::Empty);
    input_grid.pos_iter().for_each(|(pos, &tile)| {
        let (t1, t2) = match tile {
            Tile::Empty => (Tile::Empty, Tile::Empty),
            Tile::Wall => (Tile::Wall, Tile::Wall),
            Tile::Box => (Tile::BoxL, Tile::BoxR),
            Tile::Robot => (Tile::Robot, Tile::Empty),
            _ => unreachable!(),
        };
        grid[Vec2i::new(2 * pos.x, pos.y)] = t1;
        grid[Vec2i::new(2 * pos.x + 1, pos.y)] = t2;
    });

    run(&mut grid, directions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT_1: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
    const INPUT_2: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 10092);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 2028);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&input_generator(INPUT_1)), 9021);
    }
}
