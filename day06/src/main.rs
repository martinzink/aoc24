use std::collections::HashSet;
use std::error::Error;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
struct Coord(i32, i32);

impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn has_obstacle(coord: &Coord, matrix: &Vec<Vec<char>>) -> bool {
    matrix
        .get(coord.0 as usize)
        .map(|r| r.get(coord.1 as usize))
        .unwrap_or(Some(&'.'))
        .unwrap_or(&'.')
        == &'#'
}

fn get_path_no_cycle(input: &str) -> Vec<(Coord, Coord)> {
    let matrix = utils::matrix::parse_matrix(input);
    let mut guard_coord = Coord(0i32, 0i32);
    let mut max_i = 0;
    let mut max_j = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                guard_coord = Coord(i as i32, j as i32);
            }
            max_j = std::cmp::max(max_j, j as i32);
        }
        max_i = std::cmp::max(max_i, i as i32);
    }
    let mut path = Vec::new();
    let mut diff = Coord(-1, 0);

    loop {
        let next_guard_cord = &guard_coord + &diff;
        if next_guard_cord.0 < 0 || next_guard_cord.1 < 0 {
            path.push((guard_coord, diff));
            break;
        }
        if next_guard_cord.0 > max_i || next_guard_cord.1 > max_j {
            path.push((guard_coord, diff));
            break;
        }

        if !has_obstacle(&next_guard_cord, &matrix) {
            path.push((guard_coord, diff));
            guard_coord = next_guard_cord;
        } else {
            diff = match diff {
                Coord(-1, 0) => Coord(0, 1),
                Coord(0, 1) => Coord(1, 0),
                Coord(1, 0) => Coord(0, -1),
                Coord(0, -1) => Coord(-1, 0),
                Coord(_, _) => {
                    panic!("Invalid direction")
                }
            }
        }
    }
    path
}

fn part_one(input: &str) -> i32 {
    let mut path = get_path_no_cycle(input)
        .into_iter()
        .map(|(coord, _)| coord)
        .collect::<Vec<_>>();
    path.sort();
    path.dedup();
    path.len() as i32
}

fn move_guard(guard_coord: &mut Coord, guard_dir: &mut Coord, matrix: &Vec<Vec<char>>) {
    let next_guard_cord = &*guard_coord + &*guard_dir;

    let rotated_guard_dir = match guard_dir {
        Coord(-1, 0) => Coord(0, 1),
        Coord(0, 1) => Coord(1, 0),
        Coord(1, 0) => Coord(0, -1),
        Coord(0, -1) => Coord(-1, 0),
        Coord(_, _) => {
            panic!("Invalid direction")
        }
    };

    if !has_obstacle(&next_guard_cord, &matrix) {
        guard_coord.0 = next_guard_cord.0;
        guard_coord.1 = next_guard_cord.1;
    } else {
        guard_dir.0 = rotated_guard_dir.0;
        guard_dir.1 = rotated_guard_dir.1;
    }
}

fn check_for_loop(
    max_i: i32,
    max_j: i32,
    matrix: &Vec<Vec<char>>,
    guard_init_coord: &(Coord, Coord),
) -> bool {
    let mut fast_guard_coord = guard_init_coord.0;
    let mut guard_coord = guard_init_coord.0;
    let mut diff = guard_init_coord.1;
    let mut fast_diff = guard_init_coord.1;
    loop {
        move_guard(&mut guard_coord, &mut diff, matrix);
        move_guard(&mut fast_guard_coord, &mut fast_diff, matrix);
        move_guard(&mut fast_guard_coord, &mut fast_diff, matrix);

        if fast_guard_coord.0 < 0 || fast_guard_coord.1 < 0 {
            return false;
        }
        if fast_guard_coord.0 > max_i || fast_guard_coord.1 > max_j {
            return false;
        }

        if fast_guard_coord == guard_coord && fast_diff == diff {
            return true;
        }
    }
}

fn part_two(input: &str) -> i32 {
    let mut matrix = utils::matrix::parse_matrix(input);
    let max_i = matrix.len() - 1;
    let max_j = matrix[0].len() - 1;
    let coords_to_check = get_path_no_cycle(input);

    let mut filtered_coords_to_check: Vec<(Coord, Coord)> = Vec::new();
    for coord_to_check in coords_to_check {
        let a = filtered_coords_to_check
            .iter()
            .find(|(a, _b)| *a == coord_to_check.0);
        if a.is_none() {
            filtered_coords_to_check.push(coord_to_check);
        }
    }

    let mut obstacles_that_make_cycle = HashSet::new();
    for ctc in filtered_coords_to_check.windows(2) {
        let guard_coord = ctc[0];
        let possible_obstacle = ctc[1].0;
        matrix[possible_obstacle.0 as usize][possible_obstacle.1 as usize] = '#';
        if check_for_loop(max_i as i32, max_j as i32, &matrix, &guard_coord) {
            obstacles_that_make_cycle.insert(possible_obstacle);
        }
        matrix[possible_obstacle.0 as usize][possible_obstacle.1 as usize] = '.';
    }

    obstacles_that_make_cycle.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 41);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 6);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
