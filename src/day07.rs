use crate::common::parse_split_whitespace;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let (a, bs) = l.split_once(':').unwrap();
            (a.parse().unwrap(), parse_split_whitespace(bs).unwrap())
        })
        .collect()
}

fn is_valid(result: u64, numbers: &[u64], part2: bool) -> bool {
    fn concat(a: u64, b: u64) -> u64 {
        let log = if b < 10 { 1 } else { b.ilog10() + 1 };
        a * 10u64.pow(log) + b
    }

    fn is_valid_rec(current: u64, numbers: &[u64], result: u64, part2: bool) -> bool {
        if numbers.is_empty() {
            current == result
        } else if result == 0 && current == 0 {
            true
        } else {
            let n = numbers[0];
            let rest = &numbers[1..];
            if is_valid_rec(current + n, rest, result, part2)
                || is_valid_rec(current * n, rest, result, part2)
            {
                true
            } else {
                part2 && is_valid_rec(concat(current, n), rest, result, part2)
            }
        }
    }

    if result == 0 || result == 1 && numbers.is_empty() {
        true
    } else {
        is_valid_rec(numbers[0], &numbers[1..], result, part2)
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(result, numbers)| is_valid(*result, numbers, false))
        .map(|(result, _)| *result)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(result, numbers)| is_valid(*result, numbers, true))
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
}
