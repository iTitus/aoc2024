use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once('-').unwrap())
        .map(|(a, b)| (a.into(), b.into()))
        .collect()
}

#[aoc(day23, part1)]
pub fn part1(input: &[(String, String)]) -> usize {
    let mut graph: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
    for (a, b) in input {
        graph.entry(a.as_str()).or_default().insert(b.as_str());
        graph.entry(b.as_str()).or_default().insert(a.as_str());
    }

    graph
        .keys()
        .copied()
        .filter(|n| n.starts_with('t'))
        .flat_map(|tn| {
            graph[tn]
                .iter()
                .copied()
                .filter(move |&n| !n.starts_with('t') || tn < n)
                .tuple_combinations()
                .filter(|(a, b)| graph[a].contains(b))
        })
        .count()
}

#[aoc(day23, part2)]
pub fn part2(input: &[(String, String)]) -> String {
    let mut graph: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
    for (a, b) in input {
        graph.entry(a.as_str()).or_default().insert(b.as_str());
        graph.entry(b.as_str()).or_default().insert(a.as_str());
    }

    graph
        .keys()
        .map(|&n| {
            let mut clique = FxHashSet::default();
            let mut q: VecDeque<_> = [n].into();
            'outer: while let Some(n) = q.pop_front() {
                if clique.contains(n) {
                    continue 'outer;
                }

                let connected = &graph[n];
                for &c in &clique {
                    if !connected.contains(c) {
                        continue 'outer;
                    }
                }

                clique.insert(n);
                for &child in &graph[n] {
                    q.push_back(child);
                }
            }

            clique
        })
        .max_by_key(|clique| clique.len())
        .unwrap()
        .into_iter()
        .sorted_unstable()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), "co,de,ka,ta");
    }
}
