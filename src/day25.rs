use crate::common::Grid;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<Grid<char>> {
    input.split("\n\n").map(|ls| ls.parse().unwrap()).collect()
}

#[aoc(day25, part1)]
pub fn part1(input: &[Grid<char>]) -> usize {
    input
        .iter()
        .tuple_combinations()
        .filter(|&(a, b)| {
            std::iter::zip(a.iter(), b.iter()).all(|(&a, &b)| (a == '.') || (b == '.'))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator(INPUT)), 3);
    }
}
