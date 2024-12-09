use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Disk {
    File { id: u32, length: u32 },
    Free { length: u32 },
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Disk> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, length)| {
            if i % 2 == 0 {
                Disk::File {
                    id: i as u32 / 2,
                    length,
                }
            } else {
                Disk::Free { length }
            }
        })
        .collect()
}

fn compact_disk<const PART2: bool>(disk_map: &[Disk]) -> Vec<Disk> {
    println!("compact_disk");
    let mut compact = disk_map.to_vec();
    if compact.len() <= 1 {
        return compact;
    }

    let mut i = compact.len() - 1;
    while i > 0 {
        let Disk::File {
            id,
            length: mut file_length,
        } = compact[i]
        else {
            i -= 1;
            continue;
        };
        if file_length == 0 {
            i -= 1;
            continue;
        }

        let mut j = 0;
        while j < i {
            let Disk::Free {
                length: free_length,
            } = &mut compact[j]
            else {
                j += 1;
                continue;
            };
            if (PART2 && *free_length < file_length) || *free_length == 0 {
                j += 1;
                continue;
            }

            let blocks = file_length.min(*free_length);
            assert!(blocks > 0);
            file_length -= blocks;
            *free_length -= blocks;

            let new_block = Disk::File { id, length: blocks };
            if *free_length == 0 {
                compact[j] = new_block;
            } else {
                compact.insert(j, new_block);
                j += 1;
                i += 1;
            }

            if file_length == 0 {
                compact[i] = Disk::Free { length: blocks };
                break;
            } else if let Some(Disk::Free { length }) = compact.get_mut(i + 1) {
                *length += blocks;
            } else {
                compact.insert(i + 1, Disk::Free { length: blocks });
            }

            j += 1;
        }

        if file_length > 0 {
            compact[i] = Disk::File {
                id,
                length: file_length,
            };
        }

        i -= 1;
    }

    compact
}

fn checksum(disk_map: &[Disk]) -> u64 {
    disk_map
        .iter()
        .flat_map(|d| {
            let (length, id) = match d {
                Disk::File { id, length } => (*length as u64, *id as u64),
                Disk::Free { length } => (*length as u64, 0),
            };
            (0..length).map(move |_| id)
        })
        .enumerate()
        .map(|(i, id)| i as u64 * id)
        .sum()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Disk]) -> u64 {
    checksum(&compact_disk::<false>(input))
}

#[aoc(day9, part2)]
pub fn part2(input: &[Disk]) -> u64 {
    checksum(&compact_disk::<true>(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 2858);
    }
}
