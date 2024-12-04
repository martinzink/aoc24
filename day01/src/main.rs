use std::collections::HashMap;

fn parse_into_vecs(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(char::is_whitespace).unwrap();
            let au32 = a.trim().parse::<u32>().unwrap();
            let bu32 = b.trim().parse::<u32>().unwrap();
            (au32, bu32)
        }).unzip()
}

fn part_two(input: &str) -> u32 {
    let (first, second): (Vec<u32>, Vec<u32>) = parse_into_vecs(input);
    let counter = second.into_iter().fold(HashMap::new(), |mut acc: HashMap<u32, u32>, elt| {
        *acc.entry(elt).or_insert(0) += 1;
        acc
    });
    first.iter().fold(0, |a, &elt| a + counter.get(&elt).unwrap_or(&0) * elt)
}

fn part_one(input: &str) -> u32 {
    let (mut first, mut second): (Vec<u32>, Vec<u32>) = parse_into_vecs(input);
    first.sort();
    second.sort();
    first.iter().zip(second.iter()).fold(0, |acc, (a, b)| acc + a.abs_diff(*b))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 11);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 31);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    common_main(INPUT)
}
