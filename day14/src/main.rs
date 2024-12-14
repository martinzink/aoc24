use regex::Regex;
use utils::coord::Coord;

struct Robot {
    position: Coord,
    velocity: Coord,
}

fn wrap_range(input: i32, max: i32, min: i32) -> i32 {
    let modulus = max - min + 1;
    let in_group = input % modulus;
    let min_group_0 = min - min % modulus;
    let mut out = min_group_0 + in_group;
    if out < min {
        out += modulus;
    }
    out
}

impl Robot {
    fn new(line: &str) -> Robot {
        let robot_regex = "^p=(?<p_x>.*),(?<p_y>.*) v=(?<v_x>.*),(?<v_y>.*)$";
        let re = Regex::new(robot_regex).expect("invalid regex");
        let captures = re.captures(line).unwrap();
        Self{position:Coord::new(*&captures["p_x"].parse::<i32>().unwrap(), *&captures["p_y"].parse::<i32>().unwrap()),
        velocity:Coord::new(*&captures["v_x"].parse::<i32>().unwrap(), *&captures["v_y"].parse::<i32>().unwrap())}
    }

    fn parse_robots(input: &str) -> Vec<Robot> {
        input.split("\n").map(|l| Robot::new(l)).collect()
    }

    fn get_pos_after(&self, steps: i32, max_x: i32, max_y: i32) -> Coord {
        let x = wrap_range(self.position.x + self.velocity.x * steps, max_x, 0);
        let y = wrap_range(self.position.y + self.velocity.y * steps, max_y, 0);
        Coord::new(x, y)
    }

    fn move_once(&mut self, max_x: i32, max_y: i32) {
        self.position = self.get_pos_after(1, max_x, max_y);
    }
}

fn get_neighbour_count(x: &Robot, coords: &Vec<Robot>) -> usize{
    coords.iter().filter(|c| { let d = &c.position-&x.position; d.x.abs() + d.y.abs() <= 2 }).count()
}

fn get_neighbour_score(robot_coords: &Vec<Robot>) -> usize {
    robot_coords.iter().fold(0, |acc, c| { acc + get_neighbour_count(c, &robot_coords)})
}

fn visualize_robot_coords(robot_coords: &Vec<Coord>, max_x: usize, max_y: usize) {
    let mut matrix = Vec::new();
    for i in 0..max_y+1 {
        matrix.push(Vec::new());
        for j in 0..max_x+1 {
            let c = Coord::new(j as i32, i as i32);
            let res = robot_coords.iter().filter(|rb| **rb==c).count();
            let robot_vis = char::from_digit(res as u32, 10).unwrap();
            matrix[i].push(robot_vis);
        }
    }
    for i in 0..max_y+1 {
        let row = matrix[i].iter().map(|c| if *c == '0' { '.' } else { *c } ).collect::<String>();
        println!("{:?}", row);
    }
}

fn part_one(input:&str, max_x: usize, max_y: usize) -> i32 {
    let mut robots = Robot::parse_robots(input);
    for _i in 0..100 {
        for robot in &mut robots {
            robot.move_once(max_x as i32, max_y as i32);
        }
    }

    let q1_bots = robots.iter().filter(|rc| { rc.position.x < (max_x / 2) as i32 && rc.position.y < (max_y / 2) as i32 }).count();
    let q2_bots = robots.iter().filter(|rc| { rc.position.x > (max_x / 2) as i32 && rc.position.y < (max_y / 2) as i32 }).count();
    let q3_bots = robots.iter().filter(|rc| { rc.position.x < (max_x / 2) as i32 && rc.position.y > (max_y / 2) as i32 }).count();
    let q4_bots = robots.iter().filter(|rc| { rc.position.x > (max_x / 2) as i32 && rc.position.y > (max_y / 2) as i32 }).count();

   ( q1_bots * q2_bots * q3_bots * q4_bots) as i32
}

fn part_two(input:&str, max_x: usize, max_y: usize) -> i32 {
    let mut robots = Robot::parse_robots(input);
    let mut max_score = 0;
    let mut max_score_i = 0;
    for i in 1..10000 {
        for robot in &mut robots {
            robot.move_once(max_x as i32, max_y as i32);
        }
        let score = get_neighbour_score(&robots);
        max_score = max_score.max(score);
        if score == max_score {
            max_score_i = i;
        }
        if max_score > 3000 {
            break;
        }
        println!("{} max score i={}, max_score={}", i, max_score_i, max_score);
    }
    let mut max_arrangement = Vec::new();
    for robot in Robot::parse_robots(input) {
        max_arrangement.push(robot.get_pos_after(max_score_i as i32, max_x as i32, max_y as i32));
    }
    visualize_robot_coords(&max_arrangement, max_x, max_y);
    max_score_i as i32
}


fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT, 100, 102));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT, 100, 102));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");


    #[test]
    fn test_robot() {
        let robot = Robot::new("p=2,4 v=2,-3");
        assert_eq!(robot.position, Coord::new(2, 4));
        assert_eq!(robot.get_pos_after(1, 10, 6), Coord::new(4, 1));
        assert_eq!(robot.get_pos_after(2, 10, 6), Coord::new(6, 5));
        assert_eq!(robot.get_pos_after(3, 10, 6), Coord::new(8, 2));

    }

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE, 10, 6), 12);
    }

}
