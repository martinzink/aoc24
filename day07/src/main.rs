struct Operation {
    result: u64,
    operands: Vec<u64>,
}

impl Operation {
    fn munch(&self, curr_sum: u64, i: usize, operator: char, concat_enabled: bool) -> Option<u64> {
        let mut sum = curr_sum;
        if i >= self.operands.len() {
            return None;
        }
        let rhs = self.operands[i];
        match operator {
            '+' => sum += rhs,
            '*' => sum *= rhs,
            '|' => {
                let sum_math =
                    sum * u64::pow(10, f64::log(rhs as f64, 10f64).floor() as u32 + 1) + rhs;
                sum = sum_math;
            }
            _ => unreachable!(),
        };
        if sum > self.result {
            None
        } else if sum == self.result && i == self.operands.len() - 1 {
            Some(sum)
        } else {
            if concat_enabled {
                self.munch(sum, i + 1, '+', concat_enabled)
                    .or(self.munch(sum, i + 1, '*', concat_enabled))
                    .or(self.munch(sum, i + 1, '|', concat_enabled))
            } else {
                self.munch(sum, i + 1, '+', concat_enabled).or(self.munch(
                    sum,
                    i + 1,
                    '*',
                    concat_enabled,
                ))
            }
        }
    }
    fn is_valid_recursive(&self, concat_enabled: bool) -> bool {
        let sum = *self.operands.first().unwrap();
        if concat_enabled {
            self.munch(sum, 1, '+', concat_enabled)
                .or(self.munch(sum, 1, '*', concat_enabled))
                .or(self.munch(sum, 1, '|', concat_enabled))
                == Some(self.result)
        } else {
            self.munch(sum, 1, '+', concat_enabled)
                .or(self.munch(sum, 1, '*', concat_enabled))
                == Some(self.result)
        }
    }
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            let (res_str, operands_str) = line.split_once(':').unwrap();
            let res = res_str.parse::<u64>().unwrap();
            let operands = operands_str
                .trim()
                .split(' ')
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            Operation {
                result: res,
                operands,
            }
        })
        .collect()
}

fn part_one(input: &str) -> u64 {
    let inputs = parse(input);
    let mut sum = 0;
    for operation in inputs {
        if operation.is_valid_recursive(false) {
            sum += operation.result;
        }
    }
    sum
}

fn part_two(input: &str) -> u64 {
    let inputs = parse(input);
    let mut sum = 0;
    for operation in inputs {
        if operation.is_valid_recursive(true) {
            sum += operation.result;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 3749);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 11387);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
