use crate::common::parse_split;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Program {
    reg: [i64; 3],
    instructions: Vec<u8>,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b, c, _, instructions) = s.lines().collect_tuple().ok_or(())?;
        Ok(Self {
            reg: [
                a.strip_prefix("Register A: ")
                    .ok_or(())?
                    .parse()
                    .map_err(|_| ())?,
                b.strip_prefix("Register B: ")
                    .ok_or(())?
                    .parse()
                    .map_err(|_| ())?,
                c.strip_prefix("Register C: ")
                    .ok_or(())?
                    .parse()
                    .map_err(|_| ())?,
            ],
            instructions: parse_split(instructions.strip_prefix("Program: ").ok_or(())?, ',')
                .map_err(|_| ())?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Device {
    reg: [i64; 3],
    instructions: Vec<u8>,
    instruction_pointer: usize,
    output: Vec<u8>,
}

impl From<Program> for Device {
    fn from(value: Program) -> Self {
        Self {
            reg: value.reg,
            instructions: value.instructions,
            instruction_pointer: 0,
            output: vec![],
        }
    }
}

impl Device {
    fn step(&mut self) {
        let instruction = self.instructions[self.instruction_pointer];
        let operand = self.instructions[self.instruction_pointer + 1];
        match instruction {
            0 => {
                // adv
                self.reg[0] >>= self.combo_operand(operand);
            }
            1 => {
                // bxl
                self.reg[1] ^= operand as i64;
            }
            2 => {
                // bst
                self.reg[1] = self.combo_operand(operand) & 0x7;
            }
            3 => {
                // jnz
                if self.reg[0] != 0 {
                    self.instruction_pointer = operand as usize;
                    return;
                }
            }
            4 => {
                // bxc
                self.reg[1] ^= self.reg[2];
            }
            5 => {
                // out
                self.output.push((self.combo_operand(operand) & 0x7) as u8)
            }
            6 => {
                // bdv
                self.reg[1] = self.reg[0] >> self.combo_operand(operand);
            }
            7 => {
                // cdv
                self.reg[2] = self.reg[0] >> self.combo_operand(operand);
            }
            _ => unreachable!(),
        }

        self.instruction_pointer += 2;
    }

    fn combo_operand(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4..=6 => self.reg[operand as usize - 4],
            _ => unreachable!(),
        }
    }

    pub fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            self.step();
        }
    }

    pub fn run_until_out(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            let out_len = self.output.len();
            self.step();
            if self.output.len() != out_len {
                break;
            }
        }
    }

    pub fn get_output(&self) -> String {
        self.output.iter().join(",")
    }

    pub fn reset(&mut self, p: &Program) {
        self.reg = p.reg;
        self.instruction_pointer = 0;
        self.output.clear();
    }

    pub fn set_a(&mut self, a: i64) {
        self.reg[0] = a;
    }

    pub fn dump_program(&self, mut w: impl io::Write) {
        fn dump_instruction(
            instruction: u8,
            operand: u8,
            w: &mut impl io::Write,
        ) -> io::Result<()> {
            match instruction {
                0 => {
                    write!(w, "A = A >> ")?;
                    dump_combo_operand(operand, false, w)?;
                }
                1 => write!(w, "B = B ^ {operand:#x}")?,
                2 => {
                    write!(w, "B = ")?;
                    dump_combo_operand(operand, true, w)?;
                }
                3 => write!(w, "IF A != 0: JMP {operand}")?,
                4 => write!(w, "B = B ^ C")?,
                5 => {
                    write!(w, "OUT ")?;
                    dump_combo_operand(operand, true, w)?;
                }
                6 => {
                    write!(w, "B = A >> ")?;
                    dump_combo_operand(operand, false, w)?;
                }
                7 => {
                    write!(w, "C = A >> ")?;
                    dump_combo_operand(operand, false, w)?;
                }
                _ => unreachable!(),
            }
            Ok(())
        }

        fn dump_combo_operand(
            operand: u8,
            lower_bits_only: bool,
            w: &mut impl io::Write,
        ) -> io::Result<()> {
            match operand {
                0..=3 => write!(w, "{operand}")?,
                4..=6 => {
                    write!(w, "{}", (b'A' + (operand - 4)) as char)?;
                    if lower_bits_only {
                        write!(w, " & 0x7")?;
                    }
                }
                _ => unreachable!(),
            }
            Ok(())
        }

        for ((i, &instruction), (_, &operand)) in self.instructions.iter().enumerate().tuples() {
            write!(&mut w, "{i:>2}: ").unwrap();
            dump_instruction(instruction, operand, &mut w).unwrap();
            writeln!(&mut w).unwrap();
        }

        w.flush().unwrap();
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Program {
    input.parse().unwrap()
}

#[aoc(day17, part1)]
pub fn part1(input: &Program) -> String {
    let mut device: Device = input.clone().into();
    device.run();
    device.get_output()
}

#[aoc(day17, part2)]
pub fn part2(input: &Program) -> i64 {
    assert_eq!(input.instructions.len() % 2, 0);

    // only one JNZ
    let (jnz_pos, (_, jnz_target)) = input
        .instructions
        .iter()
        .copied()
        .tuples()
        .enumerate()
        .filter(|&(_, (insn, _))| insn == 3)
        .exactly_one()
        .unwrap();
    assert_eq!(jnz_pos, (input.instructions.len() - 2) / 2);
    assert_eq!(jnz_target, 0);

    // only one ADV
    let (_, bits) = input
        .instructions
        .iter()
        .copied()
        .tuples()
        .filter(|&(insn, _)| insn == 0)
        .exactly_one()
        .unwrap();
    assert!((1..=3).contains(&bits));

    let mut device: Device = input.clone().into();
    device.dump_program(io::stdout());

    let mut q: VecDeque<_> = [(0, input.instructions.len() - 1)].into();
    while let Some((a, idx)) = q.pop_front() {
        for i in 0..(1 << bits) {
            let a = a << bits | i;
            device.reset(input);
            device.set_a(a);
            device.run_until_out();

            let out = *device.output.last().unwrap();
            if out == input.instructions[idx] {
                if idx == 0 {
                    return a;
                }

                q.push_back((a, idx - 1));
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT_1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
    const INPUT_2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT_1)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT_2)), 117440);
    }
}
