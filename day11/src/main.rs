use std::collections::HashMap;

fn transform_stone(stone: u64) -> Vec<u64>{
    match stone {
        0 => { vec![1] }
        x => {
            let x_str = x.to_string();
            if x_str.len() % 2 == 0 {
                let (a,b) = x_str.split_at(x_str.len()/2);
                vec![a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()]
            } else {
                vec![x*2024]
            }
        }
    }
}

fn blink_at_stone(stone: u64, blinks: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if let Some(&cached) = memo.get(&(stone, blinks)) {
        return cached;
    }
    let mut count = 0;
    for stone in transform_stone(stone) {
        count += blink_at_stone(stone, blinks - 1, memo);
    }
    memo.insert((stone, blinks), count);
    count
}

fn part_one(input: &str) -> u64 {
    let stones = input.split_ascii_whitespace().map(|x_str| x_str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut cache = HashMap::new();
    let mut sum = 0;
    for stone in stones {
        sum += blink_at_stone(stone, 25, &mut cache);
    }
    sum
}

fn part_two(input: &str) -> u64 {
    let stones = input.split_ascii_whitespace().map(|x_str| x_str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut cache = HashMap::new();
    let mut sum = 0;
    for stone in stones {
        sum += blink_at_stone(stone, 75, &mut cache);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_stone_split() {
        assert_eq!(transform_stone(0), [1]);
        assert_eq!(transform_stone(1), [2024]);
        assert_eq!(transform_stone(10), [1, 0]);
        assert_eq!(transform_stone(99), [9, 9]);
        assert_eq!(transform_stone(999), [2021976]);
    }

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 55312);
        assert_eq!(part_one(INPUT), 187738);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 0);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}

