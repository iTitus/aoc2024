use crate::common::{Direction, Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use tinyvec::ArrayVec;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Grid<char> {
    input.parse().unwrap()
}

fn neighbors(grid: &Grid<char>, pos: Vec2i) -> impl IntoIterator<Item = Vec2i> {
    let c = grid[pos];
    Direction::VALUES
        .into_iter()
        .map(|dir| dir.offset(&pos))
        .filter(|pos| grid.in_bounds(pos) && grid[*pos] == c)
        .collect::<ArrayVec<[Vec2i; 4]>>()
}

#[aoc(day12, part1)]
pub fn part1(input: &Grid<char>) -> usize {
    let starts = input.pos_iter().map(|(p, _)| p).collect_vec();
    let components = connected_components(&starts, |&p| neighbors(input, p));
    components
        .iter()
        .map(|c| {
            let area = c.len();
            let mut perimeter = 0;
            for &p in c {
                for d in Direction::VALUES {
                    if !c.contains(&(d.offset(&p))) {
                        perimeter += 1;
                    }
                }
            }

            area * perimeter
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &Grid<char>) -> usize {
    let starts = input.pos_iter().map(|(p, _)| p).collect_vec();
    let components = connected_components(&starts, |&p| neighbors(input, p));
    components
        .iter()
        .map(|c| {
            let area = c.len();

            let mut outer_edges: FxHashMap<Direction, FxHashSet<Vec2i>> = FxHashMap::default();
            for &p in c {
                for d in Direction::VALUES {
                    if !c.contains(&(d.offset(&p))) {
                        outer_edges.entry(d).or_default().insert(p);
                    }
                }
            }

            let mut sides = 0;
            for (&normal, edges) in &mut outer_edges {
                while let Some(&edge) = edges.iter().next() {
                    sides += 1;
                    edges.remove(&edge);

                    for parallel in [normal.rotate_cw(), normal.rotate_ccw()] {
                        let mut i = 1;
                        loop {
                            let next = parallel.offset_with_amount(&edge, i);
                            if !edges.remove(&next) {
                                break;
                            }

                            i += 1;
                        }
                    }
                }
            }

            area * sides
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT_1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;
    const INPUT_2: &str = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
    const INPUT_3: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
    const INPUT_4: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
    const INPUT_5: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 140);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 772);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1(&input_generator(INPUT_3)), 1930);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&input_generator(INPUT_1)), 80);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&input_generator(INPUT_2)), 436);
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(part2(&input_generator(INPUT_4)), 236);
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(part2(&input_generator(INPUT_5)), 368);
    }
}
