use std::collections::HashMap;
use std::iter;
use utils::coord::Coord;
use itertools::Itertools;

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

fn calc_buttons_between(a: char, b: char) -> Vec<String> {
    let mut res = Vec::new();
    let curr_coord = get_dir_coord(a);
    let next_coord = get_dir_coord(b);
    let diff = &next_coord - &curr_coord;

    let x_landing = if diff.x.is_negative() { '<' } else { '>' };
    let y_landing = if diff.y.is_negative() { 'v' } else { '^' };

    let mut x_first = match (Some(b), x_landing, y_landing) {
        (Some('<'), '<', _) => { Some(false) }
        (Some('>'), '>', _) => { Some(false) }
        (Some('^'), _, '^') => { Some(true) }
        (Some('v'), _, 'v') => { Some(true) }
        (Some('v'), '<', '^') => { Some(true) }
        (Some('v'), '>', '^') => { Some(true) }
        (Some('A'), '>', _) => { Some(false) }
        (Some('A'), '<', _) => { Some(true) }
        _ => { None }
    };

    if next_coord.x == -2 && curr_coord.y == 0 {
        x_first = Some(false);
    }

    if next_coord.y == 0 && curr_coord.x == -2 {
        x_first = Some(true);
    }

    match x_first {
        Some(true) => {
            let mut work_str = String::new();
            move_x(&diff, &mut work_str);
            move_y(&diff, &mut work_str);
            work_str.push('A');
            res.push(work_str);
        }
        Some(false) => {
            let mut work_str = String::new();
            move_y(&diff, &mut work_str);
            move_x(&diff, &mut work_str);
            work_str.push('A');
            res.push(work_str);
        }
        None => {
            {
                let mut work_str = String::new();
                move_x(&diff, &mut work_str);
                move_y(&diff, &mut work_str);
                work_str.push('A');
                res.push(work_str);
            }
            {
                let mut work_str = String::new();
                move_y(&diff, &mut work_str);
                move_x(&diff, &mut work_str);
                work_str.push('A');
                res.push(work_str);
            }
        }
    }
    res
}

fn calc_buttons(input: &str, coord_getter: fn(char) -> Coord) -> Vec<String> {
    let mut overall_res = Vec::new();
    overall_res.push("".to_string());
    let mut curr_coord = Coord::new(0, 0);


    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();
        let next_c = input.chars().nth(i+1);

        let next_coord = coord_getter(c);
        let diff = &next_coord - &curr_coord;

        let x_landing = if diff.x.is_negative() { '<' } else { '>' };
        let y_landing = if diff.y.is_negative() { 'v' } else { '^' };

        let mut x_first = match (next_c, x_landing, y_landing) {
            (Some('<'), '<', _) => { Some(false) }
            (Some('>'), '>', _) => { Some(false) }
            (Some('^'), _, '^') => { Some(true) }
            (Some('v'), _, 'v') => { Some(true) }
            (Some('v'), '<', '^') => { Some(true) }
            (Some('v'), '>', '^') => { Some(true) }
            (Some('A'), '>', _) => { Some(false) }
            (Some('A'), '<', _) => { Some(true) }
            _ => { None }
        };

        if next_coord.x == -2 && curr_coord.y == 0 {
            x_first = Some(false);
        }

        if next_coord.y == 0 && curr_coord.x == -2 {
            x_first = Some(true);
        }

        match x_first {
            Some(true) => {
                let mut work_str = String::new();
                move_x(&diff, &mut work_str);
                move_y(&diff, &mut work_str);
                work_str.push('A');
                overall_res.iter_mut().for_each(|s: &mut String| s.push_str(&*work_str));
            }
            Some(false) => {
                let mut work_str = String::new();
                move_y(&diff, &mut work_str);
                move_x(&diff, &mut work_str);
                work_str.push('A');
                overall_res.iter_mut().for_each(|s: &mut String| s.push_str(&*work_str));
            }
            None => {
                let mut cloned_res = overall_res.clone();
                {
                    let mut work_str = String::new();
                    move_x(&diff, &mut work_str);
                    move_y(&diff, &mut work_str);
                    work_str.push('A');
                    cloned_res.iter_mut().for_each(|s: &mut String| s.push_str(&*work_str));
                }
                {
                    let mut work_str = String::new();
                    move_y(&diff, &mut work_str);
                    move_x(&diff, &mut work_str);
                    work_str.push('A');
                    overall_res.iter_mut().for_each(|s: &mut String| s.push_str(&*work_str));
                }
                overall_res.extend(cloned_res);
            }
        }

        curr_coord = next_coord;
    }
    overall_res
}

fn calc_recursive(previous_layer: &str, layer_count: usize, memo: &mut HashMap<(String, usize), usize>) -> usize {
    if let Some(&cached) = memo.get(&(previous_layer.to_string(), layer_count)) {
        return cached;
    }
    if layer_count == 0 {
        return previous_layer.len();
    }
    let res: usize = iter::once('A').chain(previous_layer.chars()).tuple_windows().map(|(a, b)| {
        calc_buttons_between(a, b).iter().map(|path| calc_recursive(path.as_str(), layer_count - 1, memo)).min().unwrap()
    }).sum();
    memo.insert((previous_layer.to_string(), layer_count), res);
    res
}

fn part_two_line(input: &str, number_of_middle_layers: usize) -> usize {
    let numerical_part = input[0..3].parse::<usize>().unwrap();
    let first_robots = calc_buttons(input, get_digit_coord);
    let mut memo = HashMap::new();
    let mut min = usize::MAX;
    for first_robot in first_robots {
        min = min.min(calc_recursive(&first_robot, number_of_middle_layers, &mut memo));
    }

    min * numerical_part
}

fn part_one(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        sum += part_two_line(line, 2);
    }
    sum
}

fn part_two(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        sum += part_two_line(line, 25);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_example_part_two() {
        assert_eq!(part_two_line("029A", 2,), 68*29);
        assert_eq!(part_two_line("980A", 2), 60*980);
        assert_eq!(part_two_line("179A", 2), 68*179);
        assert_eq!(part_two_line("456A", 2), 64*456);
        assert_eq!(part_two_line("379A", 2), 64*379);
    }

    #[test]
    fn calc_input_part_two() {
        assert_eq!(part_two_line("802A", 2), 802*70);
        assert_eq!(part_two_line("973A", 2), 973*68);
        assert_eq!(part_two_line("780A", 2), 780*66);
        assert_eq!(part_two_line("341A", 2), 341*72);
        assert_eq!(part_two_line("083A", 2), 83*66);
    }

}


fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
