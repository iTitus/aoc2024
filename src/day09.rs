use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Disk {
    File { id: u32, length: u32 },
    Free { length: u32 },
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Disk> {
    let mut disk = vec![];
    let mut file = true;
    let mut id = 0;
    for c in input.chars() {
        let length = c.to_digit(10).unwrap();
        let d = if file {
            let d = Disk::File { id, length };
            id += 1;
            d
        } else {
            Disk::Free { length }
        };
        file = !file;
        disk.push(d);
    }

    disk
}

fn compact_disk<const PART2: bool>(disk_map: &[Disk]) -> Vec<Disk> {
    let mut compact = disk_map.to_vec();
    if compact.len() > 1 {
        let mut i = compact.len() - 1;
        while i > 0 {
            let mut last = compact[i];
            match &mut last {
                Disk::File { id, length } if *length > 0 => {
                    let mut j = 0;
                    while *length > 0 && j < i {
                        match &mut compact[j] {
                            Disk::Free {
                                length: free_length,
                            } if if PART2 {
                                *free_length >= *length
                            } else {
                                *free_length > 0
                            } =>
                            {
                                let how_much = (*length).min(*free_length);
                                assert!(how_much > 0);
                                *length -= how_much;
                                *free_length -= how_much;

                                let new_block = Disk::File {
                                    id: *id,
                                    length: how_much,
                                };
                                if *free_length == 0 {
                                    compact[j] = new_block;
                                } else {
                                    compact.insert(j, new_block);
                                    j += 1;
                                    i += 1;
                                }
                                compact.insert(i + 1, Disk::Free { length: how_much });
                            }
                            _ => {}
                        }

                        j += 1;
                    }

                    if *length == 0 {
                        compact.remove(i);
                    } else {
                        compact[i] = last;
                    }
                }
                _ => {}
            }

            i -= 1;
        }
    }

    compact
}

fn checksum(disk_map: &[Disk]) -> u64 {
    let mut checksum = 0;
    let mut i = 0;
    for d in disk_map {
        match d {
            Disk::File { id, length } => {
                for _ in 0..*length {
                    checksum += i * (*id as u64);
                    i += 1;
                }
            }
            Disk::Free { length } => {
                i += *length as u64;
            }
        }
    }

    checksum
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
