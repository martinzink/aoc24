use regex::Regex;

fn part_one(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(input).fold(0, |acc, caps| {
        acc + &caps[1].parse::<i32>().unwrap() * &caps[2].parse::<i32>().unwrap()
    })
}

#[derive(Debug)]
struct Acc {
    sum: i32,
    enabled: bool,
}

impl Acc {
    fn new() -> Self {
        Self {
            sum: 0,
            enabled: true,
        }
    }
}

fn part_two(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let res = re.captures_iter(input).fold(Acc::new(), |mut acc, caps| {
        match &caps[0] {
            "do()" => {
                acc.enabled = true;
            }
            "don't()" => {
                acc.enabled = false;
            }
            _ => {
                if acc.enabled {
                    acc.sum += &caps[1].parse::<i32>().unwrap() * &caps[2].parse::<i32>().unwrap();
                }
            }
        }
        acc
    });
    res.sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 161);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 48);
    }
}
fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
