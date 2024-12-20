fn check_for_xmas(chars: &[char]) -> i32 {
    chars.windows(4).fold(0, |sum, window| {
        if window == ['X', 'M', 'A', 'S'] || window == ['S', 'A', 'M', 'X'] {
            sum + 1
        } else {
            sum
        }
    })
}

fn sum_horizontal(vec: &Vec<Vec<char>>) -> i32 {
    let mut left_to_right = 0;
    for row in vec.iter() {
        left_to_right += check_for_xmas(row);
    }
    left_to_right
}

fn sum_diagonal(vec: &Vec<Vec<char>>) -> i32 {
    let mut diags = Vec::new();
    for i in -(vec.len() as i32)..vec.len() as i32 {
        diags.push(Vec::new());
        for j in 0..vec.len() as i32 {
            let k = j - i;
            if k >= 0 && k < vec[0].len() as i32 {
                diags.last_mut().unwrap().push(vec[j as usize][k as usize]);
            }
        }
    }
    let rows = vec.len();
    let cols = vec[0].len();

    let anti_diagonals: Vec<Vec<_>> = (0..cols)
        .map(|k| (0..rows.min(k + 1)).map(|i| vec[i][k - i]).collect())
        .chain((1..rows).map(|k| {
            (0..cols.min(rows - k))
                .map(|i| vec[i + k][cols - 1 - i])
                .collect()
        }))
        .collect();
    let mut sum = 0;
    for diag in diags.iter() {
        sum += check_for_xmas(diag);
    }
    for anti_diag in anti_diagonals.iter() {
        sum += check_for_xmas(anti_diag);
    }
    sum
}

fn valid_pair(a: char, b: char) -> bool {
    a == 'M' && b == 'S' || a == 'S' && b == 'M'
}

fn check_x(vec: &Vec<Vec<char>>, center_i: usize, center_j: usize) -> bool {
    if vec[center_i][center_j] != 'A' {
        return false;
    }
    let top_left = vec[center_i - 1][center_j - 1];
    let bottom_right = vec[center_i + 1][center_j + 1];

    let top_right = vec[center_i + 1][center_j - 1];
    let bottom_left = vec[center_i - 1][center_j + 1];

    valid_pair(top_left, bottom_right) && valid_pair(top_right, bottom_left)
}

fn sum_x(vec: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for i in 1..vec.len() - 1 {
        for j in 1..vec[0].len() - 1 {
            if check_x(vec, i, j) {
                sum += 1;
            }
        }
    }
    sum
}

fn part_two(input: &str) -> i32 {
    let word_searcher = utils::matrix::parse_matrix(input);
    sum_x(&word_searcher)
}

fn part_one(input: &str) -> i32 {
    let word_searcher = utils::matrix::parse_matrix(input);
    let mut full_sum = 0;
    full_sum += sum_horizontal(&word_searcher);
    full_sum += sum_diagonal(&word_searcher);
    let transposed = utils::matrix::transpose_matrix(word_searcher);
    full_sum += sum_horizontal(&transposed);

    full_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE_2: &str = include_str!("example_p2.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 18);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE_2), 9);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
