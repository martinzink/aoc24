use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
struct Coord(i32, i32);

impl Sub for &Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i32> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i32) -> Self::Output {
        Coord(self.0*rhs, self.1*rhs)
    }
}

impl Coord {
    fn calculate_antinodes(&self, rhs: &Coord) -> [Coord; 2] {
        let diff: Coord = self - rhs;
        [self+diff, rhs-&diff]
    }

    fn within_bounds (&self, max_i: i32, max_j: i32) -> bool {
        self.0 <= max_i && self.1 <= max_j && self.0 >= 0 && self.1 >= 0
    }

    fn calculate_antinodes_with_resonant_harmonics(&self, rhs: &Coord, max_i: i32, max_j: i32) -> Vec<Coord> {
        let diff: Coord = self - rhs;
        let mut res: Vec<Coord> = Vec::new();
        let mut added_antinode = true;
        let mut i = 0;
        while added_antinode {
            added_antinode = false;
            let diff_i = diff*i;
            let harmonics_1 = self+diff_i;
            let harmonics_2 = rhs-&diff_i;
            if harmonics_1.within_bounds(max_i, max_j) {
                res.push(harmonics_1);
                added_antinode = true;
            }
            if harmonics_2.within_bounds(max_i, max_j) {
                res.push(harmonics_2);
                added_antinode = true;
            }
            i += 1;
        }
        res
    }
}

fn part_one(input: &str) -> u64 {
    let matrix = utils::matrix::parse_matrix(input);
    let max_i = matrix.len() as i32 - 1;
    let max_j = matrix[0].len() as i32 - 1;
    let mut antenna_types = HashMap::new();
    for i in 0..matrix.len() {
        for j in  0..matrix[0].len() {
            let char = matrix[i][j];
            match char {
                '.' => {},
                x => antenna_types.entry(x).or_insert(vec![]).push(Coord(i as i32, j as i32)),
            }
        }
    }

    let mut anti_nodes = HashSet::new();

    for (_antenna_type, antennas) in &antenna_types {
        for (i, antenna_coord_i) in antennas.iter().enumerate() {
            for (j, antenna_coord_j) in antennas.iter().enumerate() {
                if i == j {
                    continue;
                }
                let anti_nodes_i_j = antenna_coord_i.calculate_antinodes(antenna_coord_j);
                anti_nodes.insert(anti_nodes_i_j[0]);
                anti_nodes.insert(anti_nodes_i_j[1]);
            }
        }
    }
    anti_nodes.iter().filter(|coord| {coord.0 >= 0 && coord.1 >= 0 && coord.0 <= max_i && coord.1 <= max_j}).count() as u64

}

fn part_two(input: &str) -> u64 {
    let matrix = utils::matrix::parse_matrix(input);
    let max_i = matrix.len() as i32 - 1;
    let max_j = matrix[0].len() as i32 - 1;
    let mut antenna_types = HashMap::new();
    for i in 0..matrix.len() {
        for j in  0..matrix[0].len() {
            let char = matrix[i][j];
            match char {
                '.' => {},
                x => antenna_types.entry(x).or_insert(vec![]).push(Coord(i as i32, j as i32)),
            }
        }
    }

    let mut anti_nodes = HashSet::new();

    for (_antenna_type, antennas) in &antenna_types {
        for (i, antenna_coord_i) in antennas.iter().enumerate() {
            for (j, antenna_coord_j) in antennas.iter().enumerate() {
                if i == j {
                    continue;
                }
                let anti_nodes_i_j = antenna_coord_i.calculate_antinodes_with_resonant_harmonics(antenna_coord_j, max_i, max_j);
                for anti_node_i_j in anti_nodes_i_j {
                    anti_nodes.insert(anti_node_i_j);
                }
            }
        }
    }
    anti_nodes.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE_SMALL: &str = include_str!("example_small.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 14);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE_SMALL), 9);
        assert_eq!(part_two(EXAMPLE), 34);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
