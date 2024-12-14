use crate::common::{parse_vec, Vec2f, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robot {
    pos: Vec2i,
    vel: Vec2i,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_whitespace().collect_tuple().ok_or(())?;
        let p = p.strip_prefix("p=").ok_or(())?;
        let v = v.strip_prefix("v=").ok_or(())?;

        Ok(Robot {
            pos: parse_vec(p).map_err(|_| ())?,
            vel: parse_vec(v).map_err(|_| ())?,
        })
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Robot> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn check_grid_size(robots: &[Robot]) -> Vec2i {
    let (min_x, max_x) = robots
        .iter()
        .map(|r| r.pos.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = robots
        .iter()
        .map(|r| r.pos.y)
        .minmax()
        .into_option()
        .unwrap();
    assert!(min_x >= 0);
    assert!(max_x >= min_x);
    assert!(min_y >= 0);
    assert!(max_y >= min_y);

    Vec2i::new(max_x + 1, max_y + 1)
}

fn calc_safety_factor(robots: &[Robot], size: Vec2i) -> usize {
    let half_x = size.x / 2;
    let half_y = size.y / 2;
    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
    for r in robots {
        match (r.pos.x.cmp(&half_x), r.pos.y.cmp(&half_y)) {
            (Ordering::Less, Ordering::Less) => a += 1,
            (Ordering::Greater, Ordering::Less) => b += 1,
            (Ordering::Less, Ordering::Greater) => c += 1,
            (Ordering::Greater, Ordering::Greater) => d += 1,
            _ => {}
        }
    }

    a * b * c * d
}

fn calc_next_pos(r: &Robot, amount: i64, size: Vec2i) -> Vec2i {
    let mut next_pos = r.pos + amount * r.vel;
    next_pos.x = next_pos.x.rem_euclid(size.x);
    next_pos.y = next_pos.y.rem_euclid(size.y);
    next_pos
}

#[aoc(day14, part1)]
pub fn part1(input: &[Robot]) -> usize {
    let mut robots = input.to_vec();
    let size = check_grid_size(&robots);
    for r in &mut robots {
        r.pos = calc_next_pos(r, 100, size);
    }

    calc_safety_factor(&robots, size)
}

#[aoc(day14, part2)]
pub fn part2(input: &[Robot]) -> usize {
    let mut robots = input.to_vec();
    let size = check_grid_size(&robots);

    // this is just guesswork
    // calculate some measures about the position distribution
    let mut entropies = vec![];
    let mut means = vec![];
    let mut devs = vec![];
    for _ in 0..(size.x * size.y) {
        for r in &mut robots {
            r.pos = calc_next_pos(r, 1, size);
        }

        let counts = robots.iter().map(|r| r.pos).counts();
        entropies.push(
            -counts
                .values()
                .map(|count| {
                    let p = *count as f64 / robots.len() as f64;
                    p * p.log2()
                })
                .sum::<f64>(),
        );

        let mean: Vec2f = robots
            .iter()
            .map(|r| Vec2f::new(r.pos.x as _, r.pos.y as _))
            .sum::<Vec2f>()
            / robots.len() as f64;
        means.push(mean);
        devs.push(
            robots
                .iter()
                .map(|r| {
                    let d = Vec2f::new(r.pos.x as _, r.pos.y as _) - mean;
                    d.component_mul(&d)
                })
                .sum::<Vec2f>()
                / robots.len() as f64,
        );
    }

    // remember to add 1 when submitting

    println!(
        "entropies: {:?}",
        entropies.iter().position_minmax().into_option().unwrap()
    );

    println!(
        "means: {:?}",
        means.iter().position_minmax().into_option().unwrap()
    );

    println!(
        "devs: {:?}",
        devs.iter().position_minmax().into_option().unwrap()
    );

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 12);
    }
}
