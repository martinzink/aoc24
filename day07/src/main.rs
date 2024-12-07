use itertools::Itertools;

struct Operation {
    result: i128,
    operands: Vec<i128>,
}

impl Operation {
    fn munch(&self, curr_sum: i128, i: usize, operator: char) -> Option<i128> {
        let mut sum = curr_sum;
        if i >= self.operands.len() {
            return None;
        }
        let rhs = self.operands[i];
        match operator {
            '+' => sum += rhs,
            '*' => sum *= rhs,
            '|' => {
                let sum_math = sum * i128::pow(10, f64::log(rhs as f64, 10f64).floor() as u32 + 1) + rhs;
                sum = sum_math;
            },
            _ => unreachable!(),
        };
        if sum > self.result {
            None
        } else if sum == self.result && i == self.operands.len()-1 {
           Some(sum)
        } else {
            self.munch(sum, i+1, '+').or(self.munch(sum, i+1, '*')).or(self.munch(sum, i+1, '|'))
        }
    }
    fn is_valid_recursive(&self) -> bool {
        let sum = *self.operands.first().unwrap();
        self.munch(sum, 1, '+').or(self.munch(sum, 1, '*')).or(self.munch(sum, 1, '|')) == Some(self.result)

    }
    fn is_valid(&self, supported_operations: Vec<char>) -> bool {
        assert!(self.operands.len() > 1);
        let num_of_operators = self.operands.len() - 1;
        let operator_permutations = itertools::repeat_n(supported_operations.iter(), num_of_operators).multi_cartesian_product();

        for operator_perm in operator_permutations {
            let mut sum = self.operands[0];
            for (i, operator) in operator_perm.iter().enumerate() {
                let rhs = self.operands[i+1];
                match operator {
                    '+' => sum += rhs,
                    '*' => sum *= rhs,
                    '|' => {
                        let digits_of_rhs = rhs.to_string().len();
                        sum *= 10_i128.pow(digits_of_rhs as u32);
                        sum += rhs;
                    },
                    _ => unreachable!(),
                }
                if sum > self.result {
                    break;
                }
            }
            if sum == self.result {
                return true;
            }
        }

        false
    }
}

fn parse(input: &str) -> Vec<Operation> {
    input.lines().map(|line| {
        let (res_str, operands_str) = line.split_once(':').unwrap();
        let res = res_str.parse::<i128>().unwrap();
        let operands = operands_str.trim().split(' ').map(|num| num.parse::<i128>().unwrap()).collect::<Vec<i128>>();
        Operation{result:res, operands}
    }).collect()
}

fn part_one(input: &str) -> i128 {
    let inputs = parse(input);
    let mut sum = 0;
    for operation in inputs {
        if operation.is_valid(['+', '*'].to_vec()) {
            sum += operation.result;
        }
    }
    sum
}

fn part_two(input: &str) -> i128 {
    let inputs = parse(input);
    let mut sum = 0;
    for operation in inputs {
        if operation.is_valid_recursive() {
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
