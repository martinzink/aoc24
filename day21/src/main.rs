use utils::coord::Coord;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

fn get_digit_coord(digit: char) -> Coord {
    match digit {
        'A' => Coord::new(0, 0),
        '0' => Coord::new(-1, 0),
        '1' => Coord::new(-2, 1),
        '2' => Coord::new(-1, 1),
        '3' => Coord::new(0, 1),
        '4' => Coord::new(-2, 2),
        '5' => Coord::new(-1, 2),
        '6' => Coord::new(0, 2),
        '7' => Coord::new(-2, 3),
        '8' => Coord::new(-1, 3),
        '9' => Coord::new(0, 3),
        _ => panic!()
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

fn get_dir_coord(direction: char) -> Coord {
    match direction {
        'A' => Coord::new(0, 0),
        '^' => Coord::new(-1, 0),
        'v' => Coord::new(-1, -1),
        '>' => Coord::new(0, -1),
        '<' => Coord::new(-2, -1),
        _ => panic!()
    }
}

fn move_x(diff: &Coord, res: &mut String) {
    for _i in 0..diff.x.abs() {
        if diff.x.is_negative() {
            res.push('<');
        } else {
            res.push('>');
        }
    }
}

fn move_y(diff: &Coord, res: &mut String) {
    for _j in 0..diff.y.abs() {
        if diff.y.is_negative() {
            res.push('v');
        } else {
            res.push('^');
        }
    }
}

fn calc_buttons(input: &str, coord_getter: fn(char) -> Coord) -> String {
    let mut res = String::new();
    let mut curr_coord = Coord::new(0, 0);

    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();
        let next_c = input.chars().nth(i+1);

        let next_coord = coord_getter(c);
        let diff = &next_coord - &curr_coord;
        //let x_first = next_coord.x != -2;

        let x_landing = if diff.x.is_negative() { '<' } else { '>' };
        let y_landing = if diff.y.is_negative() { 'v' } else { '^' };
        let x_first_1 = match (next_c, x_landing, y_landing) {
            (Some('<'), '<', _) => { false }
            (Some('>'), '>', _) => { false }
            (Some('^'), _, '^') => { true }
            (Some('v'), _, 'v') => { true }
            _ => { true }
        }
        let x_first = match next_c {
            Some('<') | Some('>') => { false }
            Some('v') | Some('^') => { true }
            _ => { true }
        };
        let x_first = true;
        if (x_first) {
            move_x(&diff, &mut res);
            move_y(&diff, &mut res);
        } else {
            move_y(&diff, &mut res);
            move_x(&diff, &mut res);
        }

        res.push('A');
        curr_coord = next_coord;
    }
    res
}

fn part_one_line(input: &str) -> usize {
    let numerical_part = input[0..3].parse::<usize>().unwrap();
    let first_robot = calc_buttons(input, get_digit_coord);
    let second_robot = calc_buttons(first_robot.as_str(), get_dir_coord);
    let third_robot = calc_buttons(second_robot.as_str(), get_dir_coord);
    println!("{}\n{}\n{}\n{}", input, first_robot, second_robot, third_robot);
    third_robot.len() * numerical_part
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_first_numpad_test() {
        assert_eq!(calc_buttons("029A", get_digit_coord), "<A^A>^^AvvvA");
        assert_eq!(calc_buttons("<A^A>^^AvvvA", get_dir_coord), "<A^A>^^AvvvA");
    }

    #[test]
    fn calc_example_part_one() {
        assert_eq!(part_one_line("029A"), 68*29);
        assert_eq!(part_one_line("980A"), 60*980);
        //assert_eq!(part_one_line("179A"), 68*179);
        //assert_eq!(part_one_line("456A"), 64*456);
        assert_eq!(part_one_line("379A"), 64*379);
    }

}


fn main() {
    println!("Hello, world!");
}
