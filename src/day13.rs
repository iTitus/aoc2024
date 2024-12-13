use crate::common::{Mat2i, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::{Rational64, Zero};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArcadeMachine {
    a_dir: Vec2i,
    b_dir: Vec2i,
    prize_pos: Vec2i,
}

impl FromStr for ArcadeMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos(s: &str) -> Result<Vec2i, ()> {
            let (x, y) = s.split(",").map(str::trim).collect_tuple().ok_or(())?;
            let x = x.trim_start_matches(['X', '=', '+']);
            let y = y.trim_start_matches(['Y', '=', '+']);
            Ok(Vec2i::new(
                x.parse().map_err(|_| ())?,
                y.parse().map_err(|_| ())?,
            ))
        }

        let (a, b, price) = s.lines().collect_tuple().ok_or(())?;
        let a = a.strip_prefix("Button A: ").ok_or(())?;
        let b = b.strip_prefix("Button B: ").ok_or(())?;
        let prize = price.strip_prefix("Prize: ").ok_or(())?;

        Ok(ArcadeMachine {
            a_dir: parse_pos(a)?,
            b_dir: parse_pos(b)?,
            prize_pos: parse_pos(prize)?,
        })
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<ArcadeMachine> {
    input.split("\n\n").map(|ls| ls.parse().unwrap()).collect()
}

fn solve<const PART2: bool>(m: &ArcadeMachine) -> usize {
    fn det_r(m: &Mat2i) -> Rational64 {
        (m.m11 * m.m22 - m.m12 * m.m21).into()
    }

    const A_COST: usize = 3;
    const B_COST: usize = 1;

    let prize_pos = if PART2 {
        m.prize_pos + Vec2i::from_element(10000000000000)
    } else {
        m.prize_pos
    };

    // linear system:
    // res_a * m.a_dir.x + res_b * m.b_dir.x = prize_pos.x
    // res_a * m.a_dir.y + res_b * m.b_dir.y = prize_pos.y

    // solve using cramer's rule
    let det = det_r(&Mat2i::from_columns(&[m.a_dir, m.b_dir]));
    if !det.is_zero() {
        let res_a = det_r(&Mat2i::from_columns(&[prize_pos, m.b_dir])) / det;
        let res_b = det_r(&Mat2i::from_columns(&[m.a_dir, prize_pos])) / det;
        if res_a.is_integer() && res_b.is_integer() {
            let res_a = res_a.to_integer();
            let res_b = res_b.to_integer();
            if res_a >= 0 && res_b >= 0 {
                return A_COST * res_a as usize + B_COST * res_b as usize;
            }
        }
    }

    0
}

#[aoc(day13, part1)]
pub fn part1(input: &[ArcadeMachine]) -> usize {
    input.iter().map(solve::<false>).sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[ArcadeMachine]) -> usize {
    input.iter().map(solve::<true>).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 480);
    }
}
