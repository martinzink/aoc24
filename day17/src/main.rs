use regex::Regex;
use std::ops::BitXor;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Registers {
    a: u128,
    b: u128,
    c: u128,
    instruction_pointer: u128,
    output: String,
    output_vec: Vec<u32>,
}

impl Registers {
    fn new(input: &str) -> Registers {
        let reg_regex = "^Register A: (.*)\nRegister B: (.*)\nRegister C: (.*)$";
        let re = Regex::new(reg_regex).expect("invalid regex");
        let captures = re.captures(input).unwrap();
        let a = captures[1].parse::<u128>().unwrap();
        let b = captures[2].parse::<u128>().unwrap();
        let c = captures[3].parse::<u128>().unwrap();
        Self {
            a,
            b,
            c,
            instruction_pointer: 0,
            output: String::new(),
            output_vec: Vec::new(),
        }
    }
}

fn get_combo_operand(operand: u128, registers: &Registers) -> u32 {
    match operand {
        0..4 => operand as u32,
        4 => registers.a as u32,
        5 => registers.b as u32,
        6 => registers.c as u32,
        _ => {
            panic!("Invalid combo operands");
        }
    }
}

fn adv(operand: u128, registers: &mut Registers) {
    let numerator = registers.a;
    let combo_operand_value = get_combo_operand(operand, registers);
    registers.a = numerator / 2u128.pow(combo_operand_value);
    registers.instruction_pointer += 2;
}

fn bxl(operand: u128, registers: &mut Registers) {
    registers.b = registers.b.bitxor(operand);
    registers.instruction_pointer += 2;
}

fn bst(operand: u128, registers: &mut Registers) {
    registers.b = (get_combo_operand(operand, registers) % 8) as u128;
    registers.instruction_pointer += 2;
}

fn jnz(operand: u128, registers: &mut Registers) {
    if registers.a == 0 {
        registers.instruction_pointer += 2;
        return;
    }
    registers.instruction_pointer = operand;
}

fn bxc(_operand: u128, registers: &mut Registers) {
    registers.b = registers.b.bitxor(registers.c);
    registers.instruction_pointer += 2;
}

fn out(operand: u128, registers: &mut Registers) {
    if registers.output.is_empty() {
        registers.output += format!("{}", get_combo_operand(operand, registers) % 8).as_str();
    } else {
        registers.output += format!(",{}", get_combo_operand(operand, registers) % 8).as_str();
    }
    registers
        .output_vec
        .push(get_combo_operand(operand, registers) % 8);
    registers.instruction_pointer += 2;
}

fn bdv(operand: u128, registers: &mut Registers) {
    let numerator = registers.a;
    let combo_operand_value = get_combo_operand(operand, registers);
    registers.b = numerator / 2u128.pow(combo_operand_value);
    registers.instruction_pointer += 2;
}

fn cdv(operand: u128, registers: &mut Registers) {
    let numerator = registers.a;
    let combo_operand_value = get_combo_operand(operand, registers);
    registers.c = numerator / 2u128.pow(combo_operand_value);
    registers.instruction_pointer += 2;
}

fn calc(mut registers: Registers, codes: &Vec<u128>) -> (String, Vec<u32>) {
    let mut instruction_pointer = registers.instruction_pointer;
    while instruction_pointer < codes.len() as u128 {
        let instruction_opcode = codes[instruction_pointer as usize];
        let operand = codes[instruction_pointer as usize + 1];
        match instruction_opcode {
            0 => adv(operand, &mut registers),
            1 => bxl(operand, &mut registers),
            2 => bst(operand, &mut registers),
            3 => jnz(operand, &mut registers),
            4 => bxc(operand, &mut registers),
            5 => out(operand, &mut registers),
            6 => bdv(operand, &mut registers),
            7 => cdv(operand, &mut registers),
            _ => {
                panic!("Invalid instruction opcode");
            }
        }
        instruction_pointer = registers.instruction_pointer;
    }
    (registers.output, registers.output_vec)
}

fn part_one(input: &str) -> String {
    let (regs, code_str) = input.split_once("\n\n").unwrap();
    let registers = Registers::new(regs);
    let (_, only_code_str) = code_str.split_once(" ").unwrap();
    let codes = only_code_str
        .split(",")
        .map(|a| a.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    calc(registers, &codes).0
}

fn part_two(input: &str) -> u128 {
    let (regs, code_str) = input.split_once("\n\n").unwrap();
    let (_, only_code_str) = code_str.split_once(" ").unwrap();
    let codes = only_code_str
        .split(",")
        .map(|a| a.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    let mut registers = Registers::new(regs);

    let mut valid = Vec::new();
    valid.push(0);
    for _ in 0..codes.len() {
        let old_valid = valid;
        valid = Vec::new();
        for num in old_valid {
            for offset in 0..8u128 {
                let new_num = 8 * num + offset;
                registers.a = new_num as u128;
                let res = calc(registers.clone(), &codes).1;
                let a = codes.iter().rev().take(res.len()).rev().collect::<Vec<_>>();
                if a.iter().zip(res).all(|(a, b)| **a == b as u128) {
                    valid.push(new_num);
                }
            }
        }
    }

    *valid.iter().min().unwrap()
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE_2: &str = include_str!("example_2.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE_2), 117440);
    }
}
