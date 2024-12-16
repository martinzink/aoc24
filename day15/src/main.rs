use std::collections::HashSet;
use utils::coord::Coord;

fn parse(input: &str, widen: bool) -> (Vec<Vec<char>>, Vec<Coord>, Coord) {
    let (matrix_str, dirs_str) = input.split_once("\n\n").expect("invalid input");
    let mut matrix = if widen {
        let wide_matrix_str = matrix_str.replace('#', "##").replace("O", "[]").replace(".", "..").replace("@", "@.");
        utils::matrix::parse_matrix(wide_matrix_str.as_str())
    } else {
        utils::matrix::parse_matrix(matrix_str.trim())
    };

    let dirs = dirs_str.chars().filter(|c| { *c != '\n' }).map(|c|{ match c {
        '<' => {Coord::new(0, -1)}
        '>' => {Coord::new(0, 1)}
        '^' => {Coord::new(-1, 0)}
        'v' => {Coord::new(1, 0)}
        _ => { panic!("unknown direction {}", c); }
    }}).collect();
    let mut user_coord = Coord::new(0, 0);
    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, c) in row.iter_mut().enumerate() {
            if *c == '@' {
                user_coord.x = i as i32;
                user_coord.y = j as i32;
                *c = '.';
            }
        }
    }
    assert_ne!(user_coord, Coord::new(0, 0));
    (matrix, dirs, user_coord)
}

fn get_matrix_score(matrix: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                sum += 100*i + j;
            }
            if *c == '[' {
                sum += 100*i + j;
            }
        }
    }
    sum as i32
}


fn move_boxes(matrix: &mut Vec<Vec<char>>, dirs: &Vec<Coord>, mut user_coord: Coord) {
    for dir in dirs.iter() {
        let mut clear_coords = HashSet::new();
        let mut coords_that_move = HashSet::new();
        let mut move_possible = true;
        coords_that_move.insert((user_coord, '.'));
        let mut work_coords = coords_that_move.clone();
        while move_possible && work_coords.len() > 0 {
            let mut more_coords_that_move = HashSet::new();
            for (coord_that_move, _) in work_coords.iter() {

                let pointer = coord_that_move + *dir;
                let val = matrix.get(pointer.x as usize).and_then(|c| {c.get(pointer.y as usize)});
                match val {
                    None => { panic!("Should be a border of walls") }
                    Some('O') => {
                        more_coords_that_move.insert((pointer, 'O'));
                    }
                    Some('[') => {
                        more_coords_that_move.insert((pointer, '['));
                        more_coords_that_move.insert((Coord::new(pointer.x, pointer.y + 1), ']'));
                        clear_coords.insert(Coord::new(coord_that_move.x, coord_that_move.y+1));
                    }
                    Some(']') => {
                        more_coords_that_move.insert((pointer, ']'));
                        more_coords_that_move.insert((Coord::new(pointer.x, pointer.y - 1), '['));
                        clear_coords.insert(Coord::new(coord_that_move.x, coord_that_move.y-1));

                    }

                    Some('#') => {
                        move_possible = false;
                    }

                    Some('.') => {

                    }
                    _ => {
                        panic!("Unknown cell {:?}", val);
                    }
                }
            }

            work_coords = more_coords_that_move.clone();
            work_coords.retain(|c| !coords_that_move.contains(c));
            coords_that_move.extend(more_coords_that_move);
        }
        if move_possible {
            for coord in clear_coords {
                matrix[(coord.x + dir.x) as usize][(coord.y + dir.y) as usize] = '.';
            }
            for (coord, new_value) in coords_that_move.iter() {
                matrix[(coord.x + dir.x) as usize][(coord.y + dir.y) as usize] = *new_value;
            }
            user_coord += *dir;
        }
    }
}

fn part_one(input: &str) -> i32 {
    let (mut matrix, dirs, user_coord) = parse(input, false);

    move_boxes(&mut matrix, &dirs, user_coord);
    get_matrix_score(matrix)
}

fn part_two(input: &str) -> i32 {
    let (mut matrix, dirs, user_coord) = parse(input, true);
    move_boxes(&mut matrix, &dirs, user_coord);
    get_matrix_score(matrix)
}


fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE_SMALL: &str = include_str!("example_small.txt");
    const EXAMPLE_SMALL_P2: &str = include_str!("example_small_p2.txt");


    #[test]
    fn example_part_one() {

        assert_eq!(part_one(EXAMPLE_SMALL), 2028);
        assert_eq!(part_one(EXAMPLE), 10092);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 9021);
    }
}
