use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::cmp::Ordering;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (FxHashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    (
        rules
            .lines()
            .map(|l| {
                l.split('|')
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
        pages
            .lines()
            .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
            .collect(),
    )
}

#[aoc(day5, part1)]
pub fn part1((rules, updates): &(FxHashSet<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    let mut n = 0;
    'outer: for update in updates {
        for (&a, &b) in update.iter().tuple_windows() {
            if rules.contains(&(b, a)) {
                continue 'outer;
            }
        }

        assert_eq!(update.len() % 2, 1);
        n += update[update.len() / 2]
    }

    n
}

#[aoc(day5, part2)]
pub fn part2((rules, updates): &(FxHashSet<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    let mut n = 0;
    for update in updates {
        let sorted = update
            .iter()
            .copied()
            .sorted_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .collect_vec();

        if &sorted != update {
            assert_eq!(sorted.len() % 2, 1);
            n += sorted[sorted.len() / 2]
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 123);
    }
}
