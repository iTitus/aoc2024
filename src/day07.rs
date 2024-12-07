use crate::common::parse_split_whitespace;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::NonZeroU64;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(u64, Vec<NonZeroU64>)> {
    input
        .lines()
        .map(|l| {
            let (a, bs) = l.split_once(':').unwrap();
            (a.parse().unwrap(), parse_split_whitespace(bs).unwrap())
        })
        .collect()
}

const fn next_highest_power_of_10(n: NonZeroU64) -> NonZeroU64 {
    const TEN: NonZeroU64 = NonZeroU64::new(10).unwrap();
    TEN.checked_pow(n.ilog10() + 1).unwrap()
}

fn is_valid<const PART_2: bool>(result: u64, numbers: &[NonZeroU64]) -> bool {
    match numbers {
        [] => result == 0 || result == 1,
        [n] => n.get() == result,
        [rest @ .., last] => {
            if result < last.get() {
                return false;
            }

            if result % *last == 0 && is_valid::<PART_2>(result / *last, rest) {
                return true;
            }

            if PART_2 {
                let pow10 = next_highest_power_of_10(*last);
                if result % pow10 == last.get() && is_valid::<PART_2>(result / pow10, rest) {
                    return true;
                }
            }

            is_valid::<PART_2>(result - last.get(), rest)
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &[(u64, Vec<NonZeroU64>)]) -> u64 {
    input
        .iter()
        .filter(|(result, numbers)| is_valid::<false>(*result, numbers))
        .map(|(result, _)| *result)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[(u64, Vec<NonZeroU64>)]) -> u64 {
    input
        .iter()
        .filter(|(result, numbers)| is_valid::<true>(*result, numbers))
        .map(|(result, _)| *result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 11387);
    }

    #[test]
    fn test_power10() {
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(1).unwrap()),
            NonZeroU64::new(10).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(2).unwrap()),
            NonZeroU64::new(10).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(3).unwrap()),
            NonZeroU64::new(10).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(4).unwrap()),
            NonZeroU64::new(10).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(5).unwrap()),
            NonZeroU64::new(10).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(6).unwrap()),
            NonZeroU64::new(10).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(7).unwrap()),
            NonZeroU64::new(10).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(8).unwrap()),
            NonZeroU64::new(10).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(9).unwrap()),
            NonZeroU64::new(10).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(10).unwrap()),
            NonZeroU64::new(100).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(99).unwrap()),
            NonZeroU64::new(100).unwrap()
        );
        assert_eq!(
            next_highest_power_of_10(NonZeroU64::new(100).unwrap()),
            NonZeroU64::new(1000).unwrap()
        );
    }
}
