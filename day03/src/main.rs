use regex::Regex;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> i32 {
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

fn part2(input: &str) -> i32 {
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

fn main() {
    println!("{}", part1(EXAMPLE));
    println!("{}", part1(INPUT));

    println!("{}", part2(EXAMPLE));
    println!("{}", part2(INPUT));
}
