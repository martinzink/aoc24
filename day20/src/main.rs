use std::collections::{HashMap, HashSet};
use utils::coord::Coord;

pub fn get_coord_within_range(coord: &Coord, range: i32) -> Vec<Coord> {
    let mut res = Vec::new();
    for i in -range..=range {
        for j in -range..=range {
            let p = Coord::new(coord.x + i, coord.y + j);
            let dx = coord.x.abs_diff(p.x);
            let dy = coord.y.abs_diff(p.y);
            if dx + dy <= range as u32 {
                res.push(p);
            }
        }
    }
    res
}

fn get_start_coord(matrix: &Vec<Vec<char>>) -> Coord {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &elem)| (i, j, elem)))
        .find(|&(_, _, elem)| elem == 'S')
        .map(|(i, j, _)| Coord::new(i as i32, j as i32))
        .unwrap()
}

fn get_race_path(matrix: &Vec<Vec<char>>, start_coord: Coord) -> Vec<Coord> {
    let mut path = vec![start_coord];
    let mut race_ended = false;
    while !race_ended {
        for neighbour in path.last().unwrap().get_neighbours() {
            if let Some(neighbour_val) = matrix
                .get(neighbour.x as usize)
                .and_then(|r| r.get(neighbour.y as usize))
            {
                if *neighbour_val == '#' {
                    continue;
                }
                if Some(&neighbour)
                    == if path.len() > 1 {
                        path.get(path.len() - 2)
                    } else {
                        None
                    }
                {
                    continue;
                }

                path.push(neighbour);
                if *neighbour_val == 'E' {
                    race_ended = true;
                }
                break;
            }
        }
    }
    path
}

fn solver(input: &str, cheat_range: i32, min_cheat_to_consider: i32) -> usize {
    let matrix = utils::matrix::parse_matrix(input);

    let mut path = get_race_path(&matrix, get_start_coord(&matrix));

    let mut hashmap = HashMap::new();
    for (i, coord) in path.iter().enumerate() {
        hashmap.insert(*coord, i as i32);
    }

    let mut cheats: HashSet<(Coord, Coord)> = HashSet::new();

    for (i, coord) in (0i32..path.len() as i32).zip(path) {
        let cheated_neighbours = get_coord_within_range(&coord, cheat_range);
        for cheated_neighbour in cheated_neighbours {
            if let Some(cheated_value) = hashmap.get(&cheated_neighbour) {
                let distance_during_cheat = cheated_neighbour.distance(&coord) as i32;
                if cheated_value <= &i {
                    continue;
                }
                let shortcut = cheated_value - i - distance_during_cheat;
                if shortcut >= min_cheat_to_consider {
                    cheats.insert((coord, cheated_neighbour));
                }
            }
        }
    }
    cheats.len()
}

fn part_one(input: &str, min_cheat_to_consider: i32) -> usize {
    solver(input, 2, min_cheat_to_consider)
}

fn part_two(input: &str, min_cheat_to_consider: i32) -> usize {
    solver(input, 20, min_cheat_to_consider)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE, 1), 44);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE, 50), 285);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!(
        "{} part one: {}",
        env!("CARGO_PKG_NAME"),
        part_one(INPUT, 100)
    );
    println!(
        "{} part two: {:?}",
        env!("CARGO_PKG_NAME"),
        part_two(INPUT, 100)
    );
}
