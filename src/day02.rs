use crate::common::parse_split_whitespace;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Reverse;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(parse_split_whitespace)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn is_safe(report: &[u32]) -> bool {
    if !report.is_sorted() && !report.is_sorted_by_key(Reverse) {
        return false;
    }

    report
        .iter()
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&a.abs_diff(*b)))
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<u32>]) -> usize {
    input.iter().filter(|r| is_safe(r)).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<u32>]) -> usize {
    input
        .iter()
        .filter(|&r| {
            for i in 0..r.len() {
                let mut removed = r.clone();
                removed.remove(i);
                if is_safe(&removed) {
                    return true;
                }
            }

            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 4);
    }
}
