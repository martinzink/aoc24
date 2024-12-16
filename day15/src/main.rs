use regex::Regex;
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
        }
    }
    sum as i32
}


fn move_boxes(matrix: &mut Vec<Vec<char>>, dirs: &Vec<Coord>, mut user_coord: Coord) {
    for dir in dirs.iter() {
        let wide_box = dir.y != 0;
        let mut first_box : Option<Coord> = None;
        let mut first_non_box = None;
        let mut pointer = user_coord;
        while first_non_box.is_none() {
            pointer += *dir;
            let val = matrix.get(pointer.x as usize).and_then(|c| {c.get(pointer.y as usize)});
            match val {
                None => { panic!("Should be a border of walls") }
                Some('O') => {
                    if first_box.is_none() {
                        first_box = Some(pointer);
                    }
                }
                Some(c) => { first_non_box = Some(*c);}
            }
            match first_non_box {
                None => { }
                Some('.') => {
                    if (first_box.is_some()) {
                        matrix[first_box.unwrap().x as usize][first_box.unwrap().y as usize] = '.';
                        matrix[pointer.x as usize][pointer.y as usize] = 'O';
                    }
                    user_coord += *dir;
                }
                Some('#') => {

                }
                _ => {
                    panic!("Should have found a non-box");
                }
            }
        }
    }
}

fn part_one(input: &str) -> i32 {
    let (mut matrix, dirs, user_coord) = parse(input, false);

    move_boxes(&mut matrix, &dirs, user_coord);
    get_matrix_score(matrix)
}

fn part_two(input: &str) -> i32 {
    let (mut matrix, dirs, mut user_coord) = parse(input, true);
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


    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 10092);
    }

}
