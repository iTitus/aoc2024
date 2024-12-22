use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;
use std::collections::VecDeque;
use tinyvec::array_vec;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map_into()
        .collect()
}

fn dpad_neighbor(c: u8, dir: u8) -> Option<u8> {
    match c {
        b'^' => match dir {
            b'v' => Some(b'v'),
            b'>' => Some(b'A'),
            _ => None,
        },
        b'v' => match dir {
            b'^' => Some(b'^'),
            b'>' => Some(b'>'),
            b'<' => Some(b'<'),
            _ => None,
        },
        b'<' => match dir {
            b'>' => Some(b'v'),
            _ => None,
        },
        b'>' => match dir {
            b'^' => Some(b'A'),
            b'<' => Some(b'v'),
            _ => None,
        },
        b'A' => match dir {
            b'v' => Some(b'>'),
            b'<' => Some(b'^'),
            _ => None,
        },
        _ => None,
    }
}

fn numpad_neighbor(c: u8, dir: u8) -> Option<u8> {
    match c {
        b'0' => match dir {
            b'^' => Some(b'2'),
            b'>' => Some(b'A'),
            _ => None,
        },
        b'1' => match dir {
            b'^' => Some(b'4'),
            b'>' => Some(b'2'),
            _ => None,
        },
        b'2' => match dir {
            b'^' => Some(b'5'),
            b'v' => Some(b'0'),
            b'>' => Some(b'3'),
            b'<' => Some(b'1'),
            _ => None,
        },
        b'3' => match dir {
            b'^' => Some(b'6'),
            b'v' => Some(b'A'),
            b'<' => Some(b'2'),
            _ => None,
        },
        b'4' => match dir {
            b'^' => Some(b'7'),
            b'v' => Some(b'1'),
            b'>' => Some(b'5'),
            _ => None,
        },
        b'5' => match dir {
            b'^' => Some(b'8'),
            b'v' => Some(b'2'),
            b'>' => Some(b'6'),
            b'<' => Some(b'4'),
            _ => None,
        },
        b'6' => match dir {
            b'^' => Some(b'9'),
            b'v' => Some(b'3'),
            b'<' => Some(b'5'),
            _ => None,
        },
        b'7' => match dir {
            b'v' => Some(b'4'),
            b'>' => Some(b'8'),
            _ => None,
        },
        b'8' => match dir {
            b'v' => Some(b'5'),
            b'>' => Some(b'9'),
            b'<' => Some(b'7'),
            _ => None,
        },
        b'9' => match dir {
            b'v' => Some(b'6'),
            b'<' => Some(b'8'),
            _ => None,
        },
        b'A' => match dir {
            b'^' => Some(b'3'),
            b'<' => Some(b'0'),
            _ => None,
        },
        _ => None,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State<const N: usize> {
    dpad: [u8; N],
    numpad: u8,
}

impl<const N: usize> Default for State<N> {
    fn default() -> Self {
        Self {
            dpad: [b'A'; N],
            numpad: b'A',
        }
    }
}

#[derive(Default)]
struct CachingSolver<const N: usize> {
    cache: RefCell<FxHashMap<(u8, u8, usize), usize>>,
}

impl<const N: usize> CachingSolver<N> {
    fn solve_codes<S>(&self, codes: &[S]) -> usize
    where
        S: AsRef<str>,
    {
        let mut result = 0;
        for code in codes {
            let code = code.as_ref();
            let cost = self.solve_code(code);
            let n: usize = code.trim_end_matches('A').parse().unwrap();
            result += n * cost;
        }

        result
    }

    fn solve_code(&self, code: &str) -> usize {
        self.solve_code_impl(code.as_bytes(), N)
    }

    fn solve_code_impl(&self, code: &[u8], layer: usize) -> usize {
        let mut total_cost = 0;
        let mut from = b'A';
        for &to in code {
            total_cost += self.solve(from, to, layer);
            from = to;
        }

        total_cost
    }

    fn solve(&self, from: u8, to: u8, layer: usize) -> usize {
        if let Some(&cost) = self.cache.borrow().get(&(from, to, layer)) {
            return cost;
        }

        const MAX_PATH_LENGTH: usize = 8usize;

        // BFS to find all paths from->to
        let mut min: Option<usize> = None;
        let mut visited: FxHashSet<u8> = FxHashSet::default();
        let mut q: VecDeque<_> = [(from, array_vec!([u8; MAX_PATH_LENGTH]))].into();
        while let Some((cur, path)) = q.pop_front() {
            if cur == to {
                let mut path = path;
                path.push(b'A');
                let cost = if layer == 0 {
                    path.len()
                } else {
                    // minimize lower layer
                    self.solve_code_impl(&path, layer - 1)
                };
                if let Some(min) = &mut min {
                    if cost < *min {
                        *min = cost;
                    }
                } else {
                    min = Some(cost)
                }

                continue;
            }

            for dir in [b'^', b'v', b'>', b'<'] {
                let Some(next) = (if layer < N {
                    dpad_neighbor(cur, dir)
                } else {
                    numpad_neighbor(cur, dir)
                }) else {
                    continue;
                };

                if visited.contains(&next) {
                    continue;
                }

                let mut path = path;
                path.push(dir);
                q.push_back((next, path));
            }

            visited.insert(cur);
        }

        let cost = min.unwrap();
        self.cache.borrow_mut().insert((from, to, layer), cost);
        cost
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &[String]) -> usize {
    let c = CachingSolver::<2>::default();
    c.solve_codes(input)
}

#[aoc(day21, part2)]
pub fn part2(input: &[String]) -> usize {
    let c = CachingSolver::<25>::default();
    c.solve_codes(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 126384);
    }

    #[test]
    fn test_code_1_simple() {
        let c = CachingSolver::<0>::default();
        assert_eq!(c.solve_code("0"), 2);
    }

    #[test]
    fn test_code_0() {
        let c = CachingSolver::<0>::default();
        assert_eq!(c.solve_code("029A"), 12);
    }

    #[test]
    fn test_code_0_simple() {
        let c = CachingSolver::<1>::default();
        assert_eq!(c.solve_code("0"), 8);
    }

    #[test]
    fn test_code_1() {
        let c = CachingSolver::<1>::default();
        assert_eq!(c.solve_code("029A"), 28);
    }

    #[test]
    fn test_code_2_simple() {
        let c = CachingSolver::<2>::default();
        assert_eq!(c.solve_code("0"), 18);
    }

    #[test]
    fn test_code_2_1() {
        let c = CachingSolver::<2>::default();
        assert_eq!(c.solve_code("029A"), 68);
    }

    #[test]
    fn test_code_2_2() {
        let c = CachingSolver::<2>::default();
        assert_eq!(c.solve_code("980A"), 60);
    }

    #[test]
    fn test_code_2_3() {
        let c = CachingSolver::<2>::default();
        assert_eq!(c.solve_code("179A"), 68);
    }

    #[test]
    fn test_code_2_4() {
        let c = CachingSolver::<2>::default();
        assert_eq!(c.solve_code("456A"), 64);
    }

    #[test]
    fn test_code_2_5() {
        let c = CachingSolver::<2>::default();
        assert_eq!(c.solve_code("379A"), 64);
    }
}
