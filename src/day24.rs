use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use pathfinding::prelude::topological_sort;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
use std::ops::{BitAnd, BitOr, BitXor};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GateType {
    And,
    Or,
    Xor,
}

impl FromStr for GateType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(GateType::And),
            "OR" => Ok(GateType::Or),
            "XOR" => Ok(GateType::Xor),
            _ => Err(()),
        }
    }
}

impl GateType {
    pub fn eval<T: BitAnd<Output = R> + BitOr<Output = R> + BitXor<Output = R>, R>(
        &self,
        in1: T,
        in2: T,
    ) -> R {
        match self {
            GateType::And => in1 & in2,
            GateType::Or => in1 | in2,
            GateType::Xor => in1 ^ in2,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Gate {
    in1: String,
    in2: String,
    out: String,
    op: GateType,
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (in1, op, in2, _, out) = s.split_whitespace().collect_tuple().ok_or(())?;
        Ok(Self {
            in1: in1.into(),
            in2: in2.into(),
            out: out.into(),
            op: op.parse()?,
        })
    }
}

impl Gate {
    pub fn eval(&self, wires: &mut FxHashMap<String, bool>) -> Result<(), ()> {
        let in1 = *wires.get(&self.in1).ok_or(())?;
        let in2 = *wires.get(&self.in2).ok_or(())?;
        match wires.entry(self.out.clone()) {
            Entry::Occupied(_) => {
                return Err(());
            }
            Entry::Vacant(e) => {
                e.insert(self.op.eval(in1, in2));
            }
        }
        Ok(())
    }

    pub fn in_matches(&self, op: GateType, in1: impl AsRef<str>, in2: impl AsRef<str>) -> bool {
        self.op == op && self.has_in(in1) && self.has_in(in2)
    }

    pub fn half_in_matches(&self, op: GateType, in1: impl AsRef<str>) -> bool {
        self.op == op && self.has_in(in1)
    }

    pub fn out_matches(&self, op: GateType, out: impl AsRef<str>) -> bool {
        let out = out.as_ref();
        self.op == op && self.out == out
    }

    pub fn has_in(&self, in1: impl AsRef<str>) -> bool {
        let in1 = in1.as_ref();
        self.in1 == in1 || self.in2 == in1
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> (FxHashMap<String, bool>, Vec<Gate>) {
    let (wires, gates) = input.split_once("\n\n").unwrap();
    (
        wires
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| {
                let (wire, value) = l.split_once(": ").unwrap();
                (wire.into(), value.parse::<u8>().unwrap() != 0)
            })
            .collect(),
        parse_lines(gates).unwrap(),
    )
}

#[aoc(day24, part1)]
pub fn part1((wires, gates): &(FxHashMap<String, bool>, Vec<Gate>)) -> u64 {
    let sorted_gates = topological_sort(&gates.iter().collect_vec(), |&g| {
        gates.iter().filter(|&c| c.in1 == g.out || c.in2 == g.out)
    })
    .unwrap();

    let mut wires = wires.clone();
    for gate in sorted_gates {
        gate.eval(&mut wires).unwrap();
    }

    wires
        .iter()
        .filter(|(n, _)| n.starts_with('z'))
        .sorted_unstable_by_key(|(w, _)| w.as_str())
        .rfold(0u64, |a, (_, &value)| (a << 1) | value as u64)
}

#[allow(dead_code)]
fn check_adder(sorted_gates: &[&Gate], bit: usize, bit_amount: usize) -> Result<(), &'static str> {
    assert!(bit < bit_amount);
    for (x, y, carry) in iproduct!([false, true], [false, true], [false, true]) {
        let mut wires = FxHashMap::default();
        for i in 0..bit_amount {
            wires.insert(
                format!("x{i:02}"),
                if i + 1 == bit {
                    carry
                } else if i == bit {
                    x
                } else {
                    false
                },
            );
            wires.insert(
                format!("y{i:02}"),
                if i + 1 == bit {
                    carry
                } else if i == bit {
                    y
                } else {
                    false
                },
            );
        }

        let mut wires = wires.clone();
        for gate in sorted_gates {
            gate.eval(&mut wires).unwrap();
        }

        let carry_enabled = carry && bit > 0;
        if carry_enabled {
            let prev = wires[&format!("z{:02}", bit - 1)];
            if prev {
                return Err("carry source (bit-1) should be 0");
            }
        }
        let result = wires[&format!("z{bit:02}")];
        let next = wires[&format!("z{:02}", bit + 1)];
        if carry_enabled {
            if result != (((x as u8 + y as u8 + 1) & 0x1) != 0) {
                return Err("half-adder with carry-in is wrong");
            }
            if next != (((x as u8 + y as u8 + 1) >> 1) & 0x1 != 0) {
                return Err("carry-out with carry-in is wrong");
            }
        } else {
            if result != (((x as u8 + y as u8) & 0x1) != 0) {
                return Err("half-adder is wrong");
            }
            if next != (((x as u8 + y as u8) >> 1) & 0x1 != 0) {
                return Err("carry-out to next bit is wrong");
            }
        }
    }

    Ok(())
}

#[aoc(day24, part2)]
pub fn part2((wires, gates): &(FxHashMap<String, bool>, Vec<Gate>)) -> String {
    println!(
        "{} gates, {} wires",
        gates.len(),
        wires
            .keys()
            .map(|s| s.as_str())
            .chain(
                gates
                    .iter()
                    .flat_map(|g| [g.in1.as_str(), g.in2.as_str(), g.out.as_str()])
            )
            .unique()
            .count()
    );

    // check each full-adder
    let mut carry = None;
    for i in 0..wires.len() / 2 {
        let x = format!("x{i:02}");
        let y = format!("y{i:02}");
        let z = format!("z{i:02}");

        // first half-adder with x and y
        let xor1 = gates
            .iter()
            .filter(|&g| g.in_matches(GateType::Xor, &x, &y))
            .exactly_one()
            .unwrap();
        let carry1 = gates
            .iter()
            .filter(|&g| g.in_matches(GateType::And, &x, &y))
            .exactly_one()
            .unwrap();

        println!("\nbit {i}:");
        if i == 0 {
            let xor1_from_out = gates
                .iter()
                .filter(|&g| g.out_matches(GateType::Xor, &z))
                .at_most_one()
                .unwrap();
            if let Some(xor1_from_out) = xor1_from_out {
                if xor1 != xor1_from_out {
                    println!("  xor1 mismatch:");
                    println!("    {xor1:?}");
                    println!("    {xor1_from_out:?}");
                }

                carry = Some(carry1.out.as_str());
                println!("  carry-out: {}", carry1.out);
            } else {
                println!("  {xor1:?} vs {xor1_from_out:?}");
                println!("  {carry1:?}");
            }
        } else {
            // second half-adder with result of the first half-adder and carry-in
            let xor2 = gates
                .iter()
                .filter(|&g| g.half_in_matches(GateType::Xor, &xor1.out))
                .at_most_one()
                .unwrap();
            let carry2 = gates
                .iter()
                .filter(|&g| g.half_in_matches(GateType::And, &xor1.out))
                .at_most_one()
                .unwrap();
            let xor2_from_out = gates
                .iter()
                .filter(|&g| g.out_matches(GateType::Xor, &z))
                .at_most_one()
                .unwrap();

            // final combination: carry-out
            let or = gates
                .iter()
                .filter(|&g| {
                    g.op == GateType::Or
                        && (g.has_in(&carry1.out)
                            || carry2.is_some_and(|carry2| g.has_in(&carry2.out)))
                })
                .collect_vec();

            if let (Some(xor2), Some(xor2_from_out), Some(carry2), &[or]) =
                (xor2, xor2_from_out, carry2, or.as_slice())
            {
                let carry_in = if xor2.in1 == xor1.out {
                    Some(xor2.in2.as_str())
                } else if xor2.in2 == xor1.out {
                    Some(xor2.in1.as_str())
                } else {
                    println!("  incorrect carry-in input in xor2: {xor2:?}");
                    None
                };
                if let Some(carry_in) = carry_in {
                    if !carry2.has_in(carry_in) {
                        println!("  incorrect carry-in input in carry2: {carry2:?}");
                    }

                    if carry_in != carry.unwrap() {
                        println!("  wrong carry-in: {carry_in} (expected {})", carry.unwrap());
                    } else {
                        println!("  carry-in: {carry_in}");
                    }
                }

                if xor2 != xor2_from_out {
                    println!("  xor2 mismatch:");
                    println!("    {xor2:?}");
                    println!("    {xor2_from_out:?}");
                }

                let out = xor2.out.as_str();
                if out != z {
                    println!("  wrong out from xor2: {xor2:?}");
                }

                carry = Some(or.out.as_str());
                println!("  carry-out: {}", or.out);
            } else {
                println!("  xor1: {xor1:?}");
                gates
                    .iter()
                    .filter(|&g| g.has_in(&xor1.out))
                    .for_each(|g| println!("    - {g:?}"));
                println!(
                    "    => should have 2 children (XOR with out=z{i:02} + AND with out=in1 of or)"
                );
                println!("  carry1: {carry1:?}");
                gates
                    .iter()
                    .filter(|&g| g.has_in(&carry1.out))
                    .for_each(|g| println!("    - {g:?}"));
                println!("    => should have 1 child (OR with out=carry-out)");
                println!("  xor2: {xor2:?} vs {xor2_from_out:?}");
                println!("  carry2: {carry2:?}");
                println!("  or: {or:?}");
            }
        }
    }

    [
        "qff", "qnw", // bit 11, outputs of first half-adder switched around
        "pbv", "z16", // switched output
        "qqp", "z23", // switched output
        "fbq", "z36", // switched output
    ]
    .into_iter()
    .sorted_unstable()
    .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT_1: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;
    const INPUT_2: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 4);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 2024);
    }
}
