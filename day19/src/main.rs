use std::collections::{HashMap};

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towel_pattern_list_str, design_to_display_str) = input.split_once("\n\n").unwrap();
    let towel_patterns = towel_pattern_list_str.split(", ").collect::<Vec<_>>();
    let designs = design_to_display_str.split("\n").collect::<Vec<_>>();
    (towel_patterns, designs)
}

fn check_design_combo(design: &str, towels: &Vec<&str>) -> usize {
    let mut possible_start_positions = HashMap::new();
    possible_start_positions.insert(0usize, 1usize);
    for i in 0..design.len() {
        if !possible_start_positions.contains_key(&i) {
            continue;
        }
        let reachable_by = *possible_start_positions.get(&i).unwrap();

        for towel in towels {
            if design[i..].starts_with(towel) {
                let next_possible_start_position = i + towel.len();
                *possible_start_positions.entry(next_possible_start_position).or_default() += reachable_by;
            }
        }

    }
    *possible_start_positions.get(&design.len()).unwrap_or(&0)
}

fn part_one(input: &str) -> usize {
    let (towel_patterns, designs) = parse(input);
    designs.iter().fold(0, |acc, design| { acc + if check_design_combo(design, &towel_patterns) > 0 { 1 } else { 0 } })
}

fn part_two(input: &str) -> usize {
    let (towel_patterns, designs) = parse(input);
    designs.iter().fold(0, |acc, design| { acc + check_design_combo(design, &towel_patterns) })
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");


    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 6);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 16);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {:?}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
