use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{BitXor};

trait Gate : Debug {
    fn getValue(&self, gates: &HashMap<String, Box<dyn Gate>>) -> bool;
    fn print(&self, gates: &HashMap<String, Box<dyn Gate>>) -> String;
    fn get_inputs(&self, gates: &HashMap<String, Box<dyn Gate>>) -> Vec<String>;
}

trait ComplexGate {
    fn lhs_gate<'a>(&self, gates: &'a HashMap<String, Box<dyn Gate>>) -> &'a Box<dyn Gate>;
    fn rhs_gate<'a>(&self, gates: &'a HashMap<String, Box<dyn Gate>>) -> &'a Box<dyn Gate>;
}

#[derive(Debug)]
struct ConstGate {
    name : String,
    value: bool
}

impl Gate for ConstGate {
    fn getValue(&self, _gates: &HashMap<String, Box<dyn Gate>>) -> bool { self.value }
    fn print(&self, _gates: &HashMap<String, Box<dyn Gate>>)  -> String {
        format!("{}", self.name)
    }
    fn get_inputs(&self, _gates: &HashMap<String, Box<dyn Gate>>) -> Vec<String> {
        vec![self.name.clone()]
    }
}

#[derive(Debug)]
struct XorGate {
    name : String,
    lhs : String,
    rhs : String,
}

impl ComplexGate for XorGate {
    fn lhs_gate<'a>(&self, gates: &'a HashMap<String, Box<dyn Gate>>) -> &'a Box<dyn Gate> {
        gates.get(&self.lhs).unwrap()
    }
    fn rhs_gate<'a>(&self, gates: &'a HashMap<String, Box<dyn Gate>>) -> &'a Box<dyn Gate> {
        gates.get(&self.rhs).unwrap()
    }
}

impl Gate for XorGate {
    fn getValue(&self, gates: &HashMap<String, Box<dyn Gate>>) -> bool {
        gates.get(&self.lhs).unwrap().getValue(gates).bitxor(gates.get(&self.rhs).unwrap().getValue(gates))
    }
    fn print(&self, gates: &HashMap<String, Box<dyn Gate>>)  -> String {
        format!("({} XOR {})", self.lhs_gate(gates).print(gates), self.rhs_gate(gates).print(gates))
    }
    fn get_inputs(&self, gates: &HashMap<String, Box<dyn Gate>>) -> Vec<String> {
        let mut res = self.lhs_gate(gates).get_inputs(gates);
        res.append(&mut self.rhs_gate(gates).get_inputs(gates));
        res
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

    fn print(&self, gates: &HashMap<String, Box<dyn Gate>>)  -> String {
        format!("({} OR {})", self.lhs_gate(gates).print(gates), self.rhs_gate(gates).print(gates))
    }
    fn get_inputs(&self, gates: &HashMap<String, Box<dyn Gate>>) -> Vec<String> {
        let mut res = self.lhs_gate(gates).get_inputs(gates);
        res.append(&mut self.rhs_gate(gates).get_inputs(gates));
        res
    }
}

impl ComplexGate for OrGate {
    fn lhs_gate<'a>(&self, gates: &'a HashMap<String, Box<dyn Gate>>) -> &'a Box<dyn Gate> {
        gates.get(&self.lhs).unwrap()
    }
    fn rhs_gate<'a>(&self, gates: &'a HashMap<String, Box<dyn Gate>>) -> &'a Box<dyn Gate> {
        gates.get(&self.rhs).unwrap()
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

    fn print(&self, gates: &HashMap<String, Box<dyn Gate>>) -> String {
        format!("({} AND {})", self.lhs_gate(gates).print(gates), self.rhs_gate(gates).print(gates))
    }

    fn get_inputs(&self, gates: &HashMap<String, Box<dyn Gate>>) -> Vec<String> {
        let mut res = self.lhs_gate(gates).get_inputs(gates);
        res.append(&mut self.rhs_gate(gates).get_inputs(gates));
        res
    }
}

impl ComplexGate for AndGate {
    fn lhs_gate<'a>(&self, gates: &'a HashMap<String, Box<dyn Gate>>) -> &'a Box<dyn Gate> {
        gates.get(&self.lhs).unwrap()
    }
    fn rhs_gate<'a>(&self, gates: &'a HashMap<String, Box<dyn Gate>>) -> &'a Box<dyn Gate> {
        gates.get(&self.rhs).unwrap()
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
    let res_bin = z_gates.iter().map(|(_gate_name, gate)| (gate.getValue(&gates) as u64).to_string()).collect::<Vec<_>>();
    u64::from_str_radix(&res_bin.join(""), 2).unwrap()
}

fn part_two(input: &str) -> u64 {
    let gates = parse(input);
    let mut z_gates = gates.iter().filter(|g| g.0.starts_with("z")).collect::<Vec<_>>();
    z_gates.sort_by(|(a, _), (b, _)| { b.cmp(a) });

    let mut x_gates = gates.iter().filter(|g| g.0.starts_with("x")).collect::<Vec<_>>();
    x_gates.sort_by(|(a, _), (b, _)| { b.cmp(a) });
    let mut y_gates = gates.iter().filter(|g| g.0.starts_with("y")).collect::<Vec<_>>();
    y_gates.sort_by(|(a, _), (b, _)| { b.cmp(a) });

    let x = u64::from_str_radix(&x_gates.iter().map(|(_gate_name, gate)| (gate.getValue(&gates) as u64).to_string()).collect::<Vec<_>>().join(""), 2).unwrap();
    let y = u64::from_str_radix(&y_gates.iter().map(|(_gate_name, gate)| (gate.getValue(&gates) as u64).to_string()).collect::<Vec<_>>().join(""), 2).unwrap();
    let z = u64::from_str_radix(&z_gates.iter().map(|(_gate_name, gate)| (gate.getValue(&gates) as u64).to_string()).collect::<Vec<_>>().join(""), 2).unwrap();

    println!("   {x:b}\n+  {y:b}\n= {z:b}");
    0
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {:?}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part one: {:?}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}