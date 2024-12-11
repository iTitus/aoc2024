use crate::common::parse_split_whitespace;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use tinyvec::{array_vec, ArrayVec};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<u64> {
    parse_split_whitespace(input).unwrap()
}

fn split(s: u64) -> ArrayVec<[u64; 2]> {
    let mut r = array_vec!([u64; 2]);
    if s == 0 {
        r.push(1);
    } else {
        let num_digits = s.ilog10() + 1;
        if num_digits % 2 == 0 {
            let p = 10u64.pow(num_digits / 2);
            r.push(s / p);
            r.push(s % p);
        } else {
            r.push(s * 2024);
        }
    }

    r
}

fn solve<const N: usize>(stones: &[u64]) -> usize {
    let mut cache: FxHashMap<(u64, usize), usize> = HashMap::default();

    let mut stack = stones.iter().map(|&n| (n, N)).collect_vec();
    while let Some((s, steps)) = stack.pop() {
        if steps == 0 {
            cache.insert((s, steps), 1);
        } else {
            let split = split(s);
            let next_steps = steps - 1;

            let mut amount = 0;
            let mut all_children_exist = true;
            for &child in &split {
                if let Some(&child_amount) = cache.get(&(child, next_steps)) {
                    amount += child_amount;
                } else {
                    all_children_exist = false;
                    break;
                }
            }

            if all_children_exist {
                cache.insert((s, steps), amount);
            } else {
                stack.push((s, steps));
                for &child in &split {
                    stack.push((child, next_steps));
                }
            }
        }
    }

    stones.iter().map(|&n| cache[&(n, N)]).sum()
}

#[aoc(day11, part1)]
pub fn part1(input: &[u64]) -> usize {
    solve::<25>(input)
}

#[aoc(day11, part2)]
pub fn part2(input: &[u64]) -> usize {
    solve::<75>(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"125 17"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 55312);
    }
}
