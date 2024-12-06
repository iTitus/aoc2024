use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let re = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    re.captures_iter(input)
        .map(|c| {
            let a: u32 = c.get(1).unwrap().as_str().parse().unwrap();
            let b: u32 = c.get(2).unwrap().as_str().parse().unwrap();
            (a, b)
        })
        .map(|(a, b)| a * b)
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let re = Regex::new(r#"(mul)\((\d+),(\d+)\)|(do|don't)\(\)"#).unwrap();
    let mut mul_enabled = true;
    re.captures_iter(input)
        .filter_map(|c| {
            match c.get(1).or_else(|| c.get(4)).unwrap().as_str() {
                "do" => {
                    mul_enabled = true;
                }
                "don't" => {
                    mul_enabled = false;
                }
                "mul" if mul_enabled => {
                    let a: u32 = c.get(2).unwrap().as_str().parse().unwrap();
                    let b: u32 = c.get(3).unwrap().as_str().parse().unwrap();
                    return Some((a, b));
                }
                _ => {}
            }
            None
        })
        .map(|(a, b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        const INPUT: &str =
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(part1(INPUT), 161);
    }

    #[test]
    fn test_part2() {
        const INPUT: &str =
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(part2(INPUT), 48);
    }
}
