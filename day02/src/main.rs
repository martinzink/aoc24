fn is_valid(report: &Vec<i32>) -> bool {
    report.windows(2).all(|w| {
        1 <= (w[1] - w[0]) && 3 >= (w[1] - w[0])
    }) || report.windows(2).all(|w| {
        1 <= (w[0] - w[1]) && 3 >= (w[0] - w[1])
    })
}

fn is_valid_p2(report: &Vec<i32>) -> bool {
    for (i, _) in report.iter().enumerate() {
        let mut perm = (*report).clone();
        perm.remove(i);
        if is_valid(&perm) {
            return true;
        }
    }
    false
}


fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace();
            parts.map(|w| w.parse::<i32>().unwrap()).collect()
        }).collect()
}

fn part_two(input: &str) -> i32 {
    let reports = parse_reports(input);
    reports.iter().filter(|report: &&Vec<i32>| is_valid_p2(*report)).count() as i32
}

fn part_one(input: &str) -> i32 {
    let reports = parse_reports(input);
    reports.iter().filter(|report: &&Vec<i32>| is_valid(*report)).count() as i32
}


#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 2);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 4);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
