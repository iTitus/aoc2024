use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<u32> {
    parse_lines(input).unwrap()
}

fn next_secret(n: u32) -> u32 {
    const SHL1: u8 = 6;
    const SHR: u8 = 5;
    const SHL2: u8 = 11;
    const MASK: u32 = (1 << 24) - 1;

    let n = (n ^ (n << SHL1)) & MASK;
    let n = (n ^ (n >> SHR)) & MASK;
    (n ^ (n << SHL2)) & MASK
}

fn nth_secret(mut num: u32, nth: usize) -> u32 {
    for _ in 0..nth {
        num = next_secret(num);
    }
    num
}

#[aoc(day22, part1)]
pub fn part1(input: &[u32]) -> u32 {
    input.iter().map(|&n| nth_secret(n, 2000)).sum()
}

#[aoc(day22, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let mut diff_prices: FxHashMap<(i8, i8, i8, i8), u32> = FxHashMap::default();
    for &secret in input {
        let mut seen = FxHashSet::default();
        std::iter::successors(Some(secret), |&n| Some(next_secret(n)))
            .take(2001)
            .map(|n| (n % 10) as i8)
            .tuple_windows()
            .map(|(n1, n2)| (n2 - n1, n2))
            .tuple_windows()
            .for_each(|((d1, _), (d2, _), (d3, _), (d4, p4))| {
                let diffs = (d1, d2, d3, d4);
                if !seen.insert(diffs) {
                    return;
                }

                *diff_prices.entry(diffs).or_default() += p4 as u32;
            });
    }

    diff_prices.values().copied().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT_1: &str = r#"1
10
100
2024"#;
    const INPUT_2: &str = r#"1
2
3
2024"#;

    #[test]
    fn test_next_secret() {
        let mut it = std::iter::successors(Some(123), |&n| Some(next_secret(n)));
        assert_eq!(it.next(), Some(123));
        assert_eq!(it.next(), Some(15887950));
        assert_eq!(it.next(), Some(16495136));
        assert_eq!(it.next(), Some(527345));
        assert_eq!(it.next(), Some(704524));
        assert_eq!(it.next(), Some(1553684));
        assert_eq!(it.next(), Some(12683156));
        assert_eq!(it.next(), Some(11100544));
        assert_eq!(it.next(), Some(12249484));
        assert_eq!(it.next(), Some(7753432));
        assert_eq!(it.next(), Some(5908254));
    }

    #[test]
    fn test_2000th_secret() {
        assert_eq!(nth_secret(1, 2000), 8685429);
        assert_eq!(nth_secret(10, 2000), 4700978);
        assert_eq!(nth_secret(100, 2000), 15273692);
        assert_eq!(nth_secret(2024, 2000), 8667524);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 37327623);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT_2)), 23);
    }
}
