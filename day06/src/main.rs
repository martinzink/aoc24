use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::Add;
use petgraph::graph::NodeIndex;
use petgraph::visit::IntoNodeIdentifiers;

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

fn part_one(input: &str) -> i32 {
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
            println!("{:?}, {:?}", diff, guard_coord);
            diff = match diff {
                Coord(-1, 0) => { Coord(0, 1) }
                Coord(0, 1) => { Coord(1, 0) }
                Coord(1, 0) => { Coord(0, -1) }
                Coord(0, -1) => { Coord(-1, 0) }
                Coord(_, _) => { panic!("Invalid direction")}
            }
        }
    }
    visited_coords.len() as i32
}

fn part_two(input: &str) -> i32 {
    0
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
        assert_eq!(part_two(EXAMPLE), 123);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    //println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
