use regex::Regex;

#[derive(Debug)]
struct ClawMachine {
    button_a_x_movement: i128,
    button_a_y_movement: i128,
    button_b_x_movement: i128,
    button_b_y_movement: i128,

    prize_x: i128,
    prize_y: i128,
}

impl ClawMachine {
    fn new(input: &str, prize_offset: i128) -> ClawMachine {
        let claw_machine_regex = "^Button A: X\\+(?<a_x>.*), Y\\+(?<a_y>.*)\nButton B: X\\+(?<b_x>.*), Y\\+(?<b_y>.*)\nPrize: X=(?<p_x>.*), Y=(?<p_y>.*)$";
        let re = Regex::new(claw_machine_regex).expect("invalid regex");
        let captures = re.captures(input).unwrap();
        Self{button_a_x_movement:*&captures["a_x"].parse::<i128>().unwrap(),
            button_a_y_movement: *&captures["a_y"].parse::<i128>().unwrap(),
            button_b_x_movement:*&captures["b_x"].parse::<i128>().unwrap(),
            button_b_y_movement: *&captures["b_y"].parse::<i128>().unwrap(),
            prize_x:*&captures["p_x"].parse::<i128>().unwrap() + prize_offset,
            prize_y: *&captures["p_y"].parse::<i128>().unwrap() + prize_offset}
    }

    fn parse_machines(input: &str, prize_offset: i128) -> Vec<ClawMachine> {
        input.split("\n\n").map(|l| ClawMachine::new(l, prize_offset)).collect()
    }
}

fn gaussian_elimination_2x2(a: [[i128; 2]; 2], b: [i128; 2]) -> Option<[i128; 2]> {
    let mut a = a.map(|row| row.map(|val| val as f64));
    let mut b = b.map(|val| val as f64);

    let factor = a[1][0] / a[0][0];
    a[1][0] -= factor * a[0][0];
    a[1][1] -= factor * a[0][1];

    b[1] -= factor * b[0];

    if a[1][1] == 0.0 {
        return None;
    }

    let x2 = b[1] / a[1][1];
    let x1 = (b[0] - a[0][1] * x2) / a[0][0];

    let x1_i128 = x1.round();
    let x2_i128 = x2.round();

    if (x1 - x1_i128).abs() > 0.001 || (x2 - x2_i128).abs() > 0.001 {
        return None;
    }

    Some([x1_i128 as i128, x2_i128 as i128])
}

fn solver(input: &str, prize_offset: i128) -> i128 {
    let machines = ClawMachine::parse_machines(input, prize_offset);
    let mut sum = 0;
    for machine in machines {
        if let Some(res) = gaussian_elimination_2x2([[machine.button_a_x_movement, machine.button_b_x_movement], [machine.button_a_y_movement, machine.button_b_y_movement]], [machine.prize_x, machine.prize_y]) {
            sum += res[0]*3 + res[1]*1;
        }
    }
    sum
}

fn part_one(input: &str) -> i128 {
    solver(input, 0)
}

fn part_two(input: &str) -> i128 {
    solver(input, 10000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn gauss_elimination() {
        assert_eq!(gaussian_elimination_2x2([[94, 22], [34, 67]], [8400, 5400]), Some([80, 40]));
        assert_eq!(gaussian_elimination_2x2([[26, 67], [66, 21]], [12748, 12176]), None);
        assert_eq!(gaussian_elimination_2x2([[17, 84], [86, 37]], [7870, 6450]), Some([38, 86]));
        assert_eq!(gaussian_elimination_2x2([[69, 27], [23, 71]], [18641, 10279]), None);
    }

    #[test]
    fn gauss_elimination_2() {
        assert_eq!(gaussian_elimination_2x2([[94, 22], [34, 67]], [8400 + 10000000000000, 5400 + 10000000000000]), None);
        assert_eq!(gaussian_elimination_2x2([[26, 67], [66, 21]], [12748 + 10000000000000, 12176 + 10000000000000]), Some([118679050709, 103199174542]));
        assert_eq!(gaussian_elimination_2x2([[17, 84], [86, 37]], [7870 + 10000000000000, 6450 + 10000000000000]), None);
        assert_eq!(gaussian_elimination_2x2([[69, 27], [23, 71]], [18641 + 10000000000000, 10279 + 10000000000000]), Some([102851800151, 107526881786]));
    }

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 480);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 875318608908);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
