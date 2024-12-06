use std::collections::{ HashSet};
use std::error::Error;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Coord(i32, i32);

impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}


fn parse(input: &str) {
    let matrix = utils::parse_matrix(input);
}

fn has_obstacle(coord: &Coord, matrix: &Vec<Vec<char>>) -> bool {
    matrix[coord.0 as usize][coord.1 as usize] == '#'
}

fn count_distance(input: &str) -> HashSet<Coord> {
    let matrix = utils::parse_matrix(input);
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
    let mut visited_coords = HashSet::new();
    let mut diff = Coord(-1, 0);

    loop {
        let next_guard_cord = &guard_coord + &diff;
        if next_guard_cord.0 < 0 || next_guard_cord.1 < 0 {
            break;
        }
        if next_guard_cord.0 > max_i || next_guard_cord.1 > max_j {
            break;
        }

        if !has_obstacle(&next_guard_cord, &matrix) {
            visited_coords.insert(next_guard_cord);
            guard_coord = next_guard_cord;
        } else {
            diff = match diff {
                Coord(-1, 0) => { Coord(0, 1) }
                Coord(0, 1) => { Coord(1, 0) }
                Coord(1, 0) => { Coord(0, -1) }
                Coord(0, -1) => { Coord(-1, 0) }
                Coord(_, _) => { panic!("Invalid direction")}
            }
        }
    }
    visited_coords
}

fn part_one(input: &str) -> i32 {
    count_distance(input).len() as i32
}

fn check_for_loop (max_i: i32, max_j: i32, matrix: &Vec<Vec<char>>, guard_init_coord: &Coord) -> bool {
    let mut guard_coord = guard_init_coord.clone();
    let mut visited_coords = HashSet::new();
    let mut diff = Coord(-1, 0);
    loop {
        let next_guard_cord = &guard_coord + &diff;
        if next_guard_cord.0 < 0 || next_guard_cord.1 < 0 {
            return false;
        }
        if next_guard_cord.0 > max_i || next_guard_cord.1 > max_j {
            return false
        }

        let next_diff = match diff {
            Coord(-1, 0) => { Coord(0, 1) }
            Coord(0, 1) => { Coord(1, 0) }
            Coord(1, 0) => { Coord(0, -1) }
            Coord(0, -1) => { Coord(-1, 0) }
            Coord(_, _) => { panic!("Invalid direction")}
        };

        if !has_obstacle(&next_guard_cord, &matrix) {
            if !visited_coords.insert((next_guard_cord, diff)) {
                return true;
            }
            guard_coord = next_guard_cord;
        } else {
            diff = next_diff;
        }
    }
}

fn part_two(input: &str) -> i32 {
    let matrix = utils::parse_matrix(input);
    let mut guard_coord = Coord(0i32, 0i32);
    let mut max_i = 0;
    let mut max_j = 0;
    let coords_to_check = count_distance(input);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                guard_coord = Coord(i as i32, j as i32);
            }
            max_j = std::cmp::max(max_j, j as i32);
        }
        max_i = std::cmp::max(max_i, i as i32);
    }
    let mut sum = 0;

    for (i, coord_to_check) in coords_to_check.iter().enumerate() {
        let mut clone_matrix = matrix.clone();
        clone_matrix[coord_to_check.0 as usize][coord_to_check.1 as usize] = '#';
        if check_for_loop(max_i, max_j, &clone_matrix, &guard_coord) {
            sum += 1;
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
