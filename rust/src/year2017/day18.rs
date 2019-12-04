use std::collections::HashMap;
use std::io;
use std::str::FromStr;

use crate::base::Part;

pub fn part1(r: &mut dyn io::Read) -> Result<String, String> {
    solve(r, Part::One)
}

fn solve(r: &mut dyn io::Read, _part: Part) -> Result<String, String> {
    let mut input = String::new();
    r.read_to_string(&mut input).map_err(|e| e.to_string())?;
    let instructions = parse_input(&input);
    let mut processor = Processor::from_instructions(&instructions);
    while processor.recovered_frequency.is_none() {
        processor.perform_instruction();
    }
    let recovered_frequency = processor.recovered_frequency.unwrap();
    Ok(recovered_frequency.to_string())
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(Instruction::from_str)
        .map(Result::unwrap)
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Processor {
    registers: HashMap<char, i64>,
    instruction_pointer: usize,
    instructions: Vec<Instruction>,
    last_frequency: Option<i64>,
    recovered_frequency: Option<i64>,
}

impl Processor {
    fn from_instructions(instructions: &[Instruction]) -> Processor {
        let registers = initialize_registers(instructions);
        let instruction_pointer = 0;
        Processor {
            registers,
            instruction_pointer,
            instructions: instructions.to_vec(),
            last_frequency: None,
            recovered_frequency: None,
        }
    }

    fn op_to_value(&self, op: Operand) -> i64 {
        match op {
            Operand::Register(reg) => self.registers[&reg],
            Operand::Literal(val) => val,
        }
    }

    fn perform_instruction(&mut self) {
        let instruction = self.instructions[self.instruction_pointer];
        if let Instruction::Jgz(op1, op2) = instruction {
            let val1 = self.op_to_value(op1);
            let val2 = self.op_to_value(op2);
            if val1 > 0 {
                if val2 > 0 {
                    self.instruction_pointer += val2 as usize;
                } else {
                    self.instruction_pointer -= val2.abs() as usize;
                }
            } else {
                self.instruction_pointer += 1;
            }
        } else {
            match instruction {
                Instruction::Sound(op) => self.last_frequency = Some(self.op_to_value(op)),
                Instruction::Set(reg, op) => {
                    *self.registers.get_mut(&reg).unwrap() = self.op_to_value(op)
                }
                Instruction::Add(reg, op) => {
                    *self.registers.get_mut(&reg).unwrap() += self.op_to_value(op)
                }
                Instruction::Mul(reg, op) => {
                    *self.registers.get_mut(&reg).unwrap() *= self.op_to_value(op)
                }
                Instruction::Mod(reg, op) => {
                    *self.registers.get_mut(&reg).unwrap() %= self.op_to_value(op)
                }
                Instruction::Recover(op) => {
                    if self.op_to_value(op) > 0 {
                        self.recovered_frequency = Some(self.last_frequency.unwrap());
                    }
                }
                _ => {}
            }
            self.instruction_pointer += 1;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Operand {
    Register(char),
    Literal(i64),
}

impl FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Operand, String> {
        let first_char = s.chars().next().unwrap();
        if first_char.is_digit(10) || first_char == '-' {
            Ok(Operand::Literal(i64::from_str(s).unwrap()))
        } else {
            Ok(Operand::Register(char::from_str(s).unwrap()))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Instruction {
    Sound(Operand),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Recover(Operand),
    Jgz(Operand, Operand),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Instruction, String> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() == 2 {
            let operand = Operand::from_str(parts[1])?;
            match parts[0] {
                "snd" => Ok(Instruction::Sound(operand)),
                "rcv" => Ok(Instruction::Recover(operand)),
                _ => Err(format!("invalid instruction: {}", s)),
            }
        } else if parts.len() == 3 {
            let op2 = Operand::from_str(parts[2])?;
            if parts[0] == "jgz" {
                let op1 = Operand::from_str(parts[1])?;
                Ok(Instruction::Jgz(op1, op2))
            } else {
                let op1 = char::from_str(parts[1]).map_err(|e| e.to_string())?;
                match parts[0] {
                    "set" => Ok(Instruction::Set(op1, op2)),
                    "add" => Ok(Instruction::Add(op1, op2)),
                    "mul" => Ok(Instruction::Mul(op1, op2)),
                    "mod" => Ok(Instruction::Mod(op1, op2)),
                    _ => Err(format!("invalid instruction: {}", s)),
                }
            }
        } else {
            Err(format!("invalid instruction: {}", s))
        }
    }
}

fn initialize_registers(instructions: &[Instruction]) -> HashMap<char, i64> {
    let mut map = HashMap::new();
    for &instruction in instructions {
        let operands = match instruction {
            Instruction::Sound(op) => vec![op],
            Instruction::Set(reg, op)
            | Instruction::Add(reg, op)
            | Instruction::Mul(reg, op)
            | Instruction::Mod(reg, op) => {
                map.insert(reg, 0);
                vec![op]
            }
            Instruction::Recover(op) => vec![op],
            Instruction::Jgz(op1, op2) => vec![op1, op2],
        };
        for op in operands.into_iter() {
            if let Operand::Register(reg) = op {
                map.insert(reg, 0);
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    mod part1 {
        use super::*;

        test!(example, include_str!("testdata/day18/ex"), "4", part1);
        test!(
            actual,
            include_str!("../../../inputs/2017/18"),
            "3188",
            part1
        );
    }
}
