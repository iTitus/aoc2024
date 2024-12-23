use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

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

    // optimized algorithm to enumerate all 3-cliques
    // is fast because the initial node needs to have a name that starts with 't'
    graph
        .iter()
        .filter(|(n, _)| n.starts_with('t'))
        .flat_map(|(tn, neighbors)| {
            neighbors
                .iter()
                .filter(move |&n| !n.starts_with('t') || tn < n)
                .tuple_combinations()
                .filter(|&(a, b)| graph[a].contains(b))
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
        .map(|&initial| {
            // this only works because the input is nice
            // (all nodes have the same degree N and the maximal clique contains N nodes)
            // there might be case where this breaks:
            // all nodes of the clique have N-1 edges used for the inter-clique connections,
            // but 1 edge leads outside
            // if the iteration order always has this connection first for all nodes in the clique,
            // this greedy algorithm will not find that clique
            let mut clique = FxHashSet::default();
            let mut q = vec![initial];
            while let Some(n) = q.pop() {
                if clique.contains(n) {
                    continue;
                }

                let neighbors = &graph[n];
                if clique.difference(neighbors).next().is_some() {
                    continue;
                }

                clique.insert(n);
                q.extend(neighbors);
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
