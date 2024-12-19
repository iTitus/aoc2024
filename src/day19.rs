use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::cell::RefCell;

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> (Vec<String>, Vec<String>) {
    let (a, b) = input.split("\n\n").collect_tuple().unwrap();
    let a = a
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map_into()
        .collect();
    let b = b
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map_into()
        .collect();
    (a, b)
}

#[derive(Debug)]
struct CachingTowelChecker<'a> {
    patterns: &'a Vec<String>,
    counts: RefCell<FxHashMap<&'a str, usize>>,
}

impl<'a> CachingTowelChecker<'a> {
    pub fn new(patterns: &'a Vec<String>) -> Self {
        Self {
            patterns,
            counts: Default::default(),
        }
    }

    pub fn check_design_possible(&self, design: &'a str) -> bool {
        self.count_possibilities(design) > 0
    }

    pub fn count_possibilities(&self, design: &'a str) -> usize {
        if design.is_empty() {
            return 1;
        } else if let Some(&count) = self.counts.borrow().get(design) {
            return count;
        }

        let mut count = 0;
        for pattern in self.patterns {
            if let Some(suffix) = design.strip_prefix(pattern) {
                count += self.count_possibilities(suffix);
            }
        }

        self.counts.borrow_mut().insert(design, count);
        count
    }
}

#[aoc(day19, part1)]
pub fn part1((patterns, designs): &(Vec<String>, Vec<String>)) -> usize {
    let checker = CachingTowelChecker::new(patterns);
    designs
        .iter()
        .filter(|design| checker.check_design_possible(design))
        .count()
}

#[aoc(day19, part2)]
pub fn part2((patterns, designs): &(Vec<String>, Vec<String>)) -> usize {
    let checker = CachingTowelChecker::new(patterns);
    designs
        .iter()
        .map(|design| checker.count_possibilities(design))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 16);
    }
}
