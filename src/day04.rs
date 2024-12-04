use crate::common::{Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Grid<char> {
    input.parse().unwrap()
}

#[aoc(day4, part1)]
pub fn part1(input: &Grid<char>) -> usize {
    const NEEDLE: &str = "XMAS";
    const DIRS: [Vec2i; 8] = [
        Vec2i::new(1, 0),
        Vec2i::new(-1, 0),
        Vec2i::new(0, 1),
        Vec2i::new(0, -1),
        Vec2i::new(1, 1),
        Vec2i::new(-1, 1),
        Vec2i::new(1, -1),
        Vec2i::new(-1, -1),
    ];

    let mut count = 0;
    for (pos, &value) in input.pos_iter() {
        if value != NEEDLE.chars().next().unwrap() {
            continue;
        }

        if NEEDLE.len() > 1 {
            'dirs: for dir in DIRS {
                for (i, c) in NEEDLE.chars().enumerate().skip(1) {
                    let offset_pos = pos + (i as i64) * dir;
                    if !input.in_bounds(&offset_pos) || input[offset_pos] != c {
                        continue 'dirs;
                    }
                }

                count += 1;
            }
        } else {
            count += 1;
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn part2(input: &Grid<char>) -> usize {
    const NEEDLE: &str = "MAS";
    assert_eq!(NEEDLE.len() % 2, 1);
    const DIRS_1: [Vec2i; 2] = [Vec2i::new(1, 1), Vec2i::new(-1, -1)];
    const DIRS_2: [Vec2i; 2] = [Vec2i::new(1, -1), Vec2i::new(-1, 1)];

    let mut count = 0;
    'middle_pos: for (pos, &value) in input.pos_iter() {
        if value != NEEDLE.chars().nth(NEEDLE.len() / 2).unwrap() {
            continue;
        }

        if NEEDLE.len() > 1 {
            let mut found = false;
            'dirs: for dir in DIRS_1 {
                for (i, c) in NEEDLE.chars().enumerate() {
                    let offset_index = i as i64 - (NEEDLE.len() / 2) as i64;
                    let offset_pos = pos + offset_index * dir;
                    if !input.in_bounds(&offset_pos) || input[offset_pos] != c {
                        continue 'dirs;
                    }
                }

                found = true;
                break;
            }

            if !found {
                continue 'middle_pos;
            }

            found = false;
            'dirs: for dir in DIRS_2 {
                for (i, c) in NEEDLE.chars().enumerate() {
                    let offset_index = i as i64 - (NEEDLE.len() / 2) as i64;
                    let offset_pos = pos + offset_index * dir;
                    if !input.in_bounds(&offset_pos) || input[offset_pos] != c {
                        continue 'dirs;
                    }
                }

                found = true;
                break;
            }

            if found {
                count += 1
            }
        } else {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 9);
    }
}
