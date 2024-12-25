use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{BitOr, BitXor};
use std::path::Component::ParentDir;

trait Gate : Debug {
    fn getValue(&self, gates: &HashMap<String, Box<dyn Gate>>) -> bool;
}

#[derive(Debug)]
struct ConstGate {
    name : String,
    value: bool
}

impl Gate for ConstGate {
    fn getValue(&self, _gates: &HashMap<String, Box<dyn Gate>>) -> bool { self.value }
}

#[derive(Debug)]
struct XorGate {
    name : String,
    lhs : String,
    rhs : String,
}

impl Gate for XorGate {
    fn getValue(&self, gates: &HashMap<String, Box<dyn Gate>>) -> bool {
        gates.get(&self.lhs).unwrap().getValue(gates).bitxor(gates.get(&self.rhs).unwrap().getValue(gates))
    }
}

#[derive(Debug)]
struct OrGate {
    name : String,
    lhs : String,
    rhs : String,
}

impl Gate for OrGate {
    fn getValue(&self, gates: &HashMap<String, Box<dyn Gate>>) -> bool {
        gates.get(&self.lhs).unwrap().getValue(gates) || gates.get(&self.rhs).unwrap().getValue(gates)
    }
}

#[derive(Debug)]
struct AndGate {
    name : String,
    lhs : String,
    rhs : String,
}

impl Gate for AndGate {
    fn getValue(&self, gates: &HashMap<String, Box<dyn Gate>>) -> bool {
        gates.get(&self.lhs).unwrap().getValue(gates) && gates.get(&self.rhs).unwrap().getValue(gates)
    }
}

fn parse(input: &str) -> HashMap<String, Box<dyn Gate>> {
    let mut gates: HashMap<String, Box<dyn Gate>> = HashMap::new();
    let (const_gates_str, complex_gates_str) = input.split_once("\n\n").unwrap();
    for const_gate_str in const_gates_str.lines() {
        let (name_str, value_str) = const_gate_str.split_once(": ").unwrap();
        let value = match value_str {
            "0" => false,
            "1" => true,
            &_ => { panic!("Unknown value {}", value_str) }
        };
        gates.insert(name_str.to_string(), Box::new(ConstGate { name: name_str.to_string(), value }));
    }
    for complex_gate_str in complex_gates_str.lines() {
        let (operation_str, name_str) = complex_gate_str.split_once(" -> ").unwrap();
        if let Some((and_lhs, and_rhs)) = operation_str.split_once(" AND ") {
            gates.insert(name_str.to_string(), Box::new(AndGate { name: name_str.to_string(), lhs: and_lhs.to_string(), rhs: and_rhs.to_string() }));
        }
        if let Some((or_lhs, or_rhs)) = operation_str.split_once(" OR ") {
            gates.insert(name_str.to_string(), Box::new(OrGate { name: name_str.to_string(), lhs: or_lhs.to_string(), rhs: or_rhs.to_string() }));

        }
        if let Some((xor_lhs, xor_rhs)) = operation_str.split_once(" XOR ") {
            gates.insert(name_str.to_string(), Box::new(XorGate { name: name_str.to_string(), lhs: xor_lhs.to_string(), rhs: xor_rhs.to_string() }));
        }
    }
    gates
}

fn part_one(input: &str) -> u64 {
    let gates = parse(input);
    let mut z_gates = gates.iter().filter(|g| g.0.starts_with("z")).collect::<Vec<_>>();
    z_gates.sort_by(|(a, _), (b, _)| { b.cmp(a) });
    let asd = z_gates.iter().map(|(gate_name, gate)| (gate.getValue(&gates) as u64).to_string()).collect::<Vec<_>>();
    println!("{:?}", asd.join(""));
    u64::from_str_radix(&asd.join(""), 2).unwrap()
}

fn part_two(input: &str) -> u64 {
    0
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {:?}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part one: {:?}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}