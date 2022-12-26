use std::collections::HashSet;

fn main() {
    get_num_of_unique_positions();
    get_num_of_unique_positions_10_knots();
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

impl From<String> for Direction {
    fn from(line: String) -> Self {
        let distance = line[2..].parse::<i32>().unwrap();
        match &line[0..1] {
            "L" => Direction::Left(distance),
            "R" => Direction::Right(distance),
            "U" => Direction::Up(distance),
            "D" => Direction::Down(distance),
            _ => unreachable!("Invalid direction"),
        }
    }
}

impl Direction {
    pub fn apply(&self, leader_x: &mut i32, leader_y: &mut i32) {
        match self {
            Direction::Left(d) => *leader_x -= d,
            Direction::Right(d) => *leader_x += d,
            Direction::Up(d) => *leader_y += d,
            Direction::Down(d) => *leader_y -= d,
        };
    }

    pub fn apply_unit(&self, leader_x: &mut i32, leader_y: &mut i32) {
        match self {
            Direction::Left(_) => *leader_x -= 1,
            Direction::Right(_) => *leader_x += 1,
            Direction::Up(_) => *leader_y += 1,
            Direction::Down(_) => *leader_y -= 1,
        };
    }

    pub fn get_magnitude(&self) -> i32 {
        *match self {
            Direction::Left(d) => d,
            Direction::Right(d) => d,
            Direction::Up(d) => d,
            Direction::Down(d) => d,
        }
    }
}

fn is_touching(leader_pos: (i32, i32), follower_pos: (i32, i32)) -> bool {
    let x_dif = (leader_pos.0 - follower_pos.0).abs();
    let y_dif = (leader_pos.1 - follower_pos.1).abs();

    x_dif <= 1 && y_dif <= 1
}

fn move_follower(leader_axis: i32, follower_axis: &mut i32) {
    if leader_axis > *follower_axis {
        *follower_axis += 1;
    } else {
        *follower_axis -= 1;
    }
}

pub fn get_num_of_unique_positions() {
    let lines = common::get_lines!();
    let mut positions = HashSet::<(i32, i32)>::new();
    positions.insert((0, 0));

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    for line in lines {
        let direction: Direction = line.unwrap().into();
        direction.apply(&mut head_pos.0, &mut head_pos.1);

        while !is_touching(head_pos, tail_pos) {
            let x_dif = (head_pos.0 - tail_pos.0).abs();
            let y_dif = (head_pos.1 - tail_pos.1).abs();

            if (y_dif >= 2 && x_dif >= 1) || (x_dif >= 2 && y_dif >= 1) {
                // move diagonally
                move_follower(head_pos.0, &mut tail_pos.0);
                move_follower(head_pos.1, &mut tail_pos.1);
            } else if x_dif >= 2 {
                move_follower(head_pos.0, &mut tail_pos.0);
            } else if y_dif >= 2 {
                move_follower(head_pos.1, &mut tail_pos.1);
            } else {
                unreachable!("Invalid state?");
            }

            positions.insert(tail_pos);
        }
    }

    println!("Unique tail positions: {}", positions.len());
}

pub fn get_num_of_unique_positions_10_knots() {
    let lines = common::get_lines!();
    let mut positions = HashSet::<(i32, i32)>::new();
    positions.insert((0, 0));

    let mut knots = [(0, 0); 10];

    for line in lines {
        let direction: Direction = line.unwrap().into();

        // The head doesn't move in one move, it moves in one step at a time, and everything else updates
        let total_direction_magnitude = direction.get_magnitude();
        let mut current_magnitude = 0;

        while current_magnitude < total_direction_magnitude {
            current_magnitude += 1;
            let head = &mut knots[0];
            direction.apply_unit(&mut head.0, &mut head.1);

            // cant use .windows as its always immutable
            let mut leader_idx = 0;
            let mut follower_idx = 1;

            while follower_idx < 10 {
                let head_pos = knots[leader_idx];
                let tail_pos = &mut knots[follower_idx];

                while !is_touching(head_pos, *tail_pos) {
                    let x_dif = (head_pos.0 - tail_pos.0).abs();
                    let y_dif = (head_pos.1 - tail_pos.1).abs();

                    if (y_dif >= 2 && x_dif >= 1) || (x_dif >= 2 && y_dif >= 1) {
                        // move diagonally
                        move_follower(head_pos.0, &mut tail_pos.0);
                        move_follower(head_pos.1, &mut tail_pos.1);
                    } else if x_dif >= 2 {
                        move_follower(head_pos.0, &mut tail_pos.0);
                    } else if y_dif >= 2 {
                        move_follower(head_pos.1, &mut tail_pos.1);
                    } else {
                        unreachable!("Invalid state?");
                    }

                    if follower_idx == 9 {
                        positions.insert(*tail_pos);
                    }
                }

                leader_idx += 1;
                follower_idx += 1;
            }
        }
    }

    println!("Unique tail positions: {}", positions.len());
}
