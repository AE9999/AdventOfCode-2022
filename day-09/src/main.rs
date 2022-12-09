use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let rope_movements = read_input(&args[1])?;

    println!("{:?} positions does the tail of the rope visit at least once.",
             solve1(&rope_movements));

    println!("{:?} positions does the tail of the rope visit at least once.",
             solve2(&rope_movements));

    Ok(())
}

fn read_input(filename: &String) -> io::Result<Vec<RopeMovement>> {
    let file_in = File::open(filename)?;
    let rope_movements =
        BufReader::new(file_in)
                  .lines()
                  .map(|line| line.unwrap())
                  .map(|line| RopeMovement::new(line))
                  .collect::<Vec<RopeMovement>>();
    Ok(rope_movements)
}

fn solve1(rope_movements: &Vec<RopeMovement>) -> usize {
    let mut visited_tail_positions: HashSet<Point> = HashSet::new();

    let mut position_head = Point::new(0, 0);
    let mut position_tail = Point::new(0,0);

    visited_tail_positions.insert(position_tail.clone());

    for rope_movement in rope_movements {
        for _step in 0..(rope_movement.amount) {
            position_head.move_step(&rope_movement.direction);
            propagate_move(&mut position_head, &mut position_tail);
            visited_tail_positions.insert(position_tail.clone());
        }
    }


    visited_tail_positions.len()
}

fn solve2(rope_movements: &Vec<RopeMovement>) -> usize {
    let mut visited_tail_positions: HashSet<Point> = HashSet::new();
    let mut positions: Vec<Point> = vec![Point::new(0,0) ; 10];

    for rope_movement in rope_movements {
        for _step in 0..(rope_movement.amount) {
            positions.get_mut(0).unwrap().move_step(&rope_movement.direction);
            for i in 1..positions.len() {
                propagate_move(&(positions[i - 1].clone()),
                               positions.get_mut(i).unwrap());

            }
            visited_tail_positions.insert(positions.last().unwrap().clone());
        }
    }

    visited_tail_positions.len()
}

fn propagate_move(position_head: &Point, position_tail: &mut Point) {

    let distance_x = (position_head.x - position_tail.x).abs();

    let distance_y = (position_head.y - position_tail.y).abs();

    let directions_tail: Option<Vec<char>> =
        // If the head is ever two steps directly up, down, left, or right from the tail,
        // the tail must also move one step in that direction so it remains close enough:
        if distance_x == 0 && distance_y == 2 {
            if position_head.y >  position_tail.y {
                Some(vec!('U'))
            } else {
                Some(vec!('D'))
            }
        }
        else if distance_x == 2 && distance_y == 0 {
            if position_head.x >  position_tail.x {
                Some(vec!('R'))
            } else {
                Some(vec!('L'))
            }
        }
        // Otherwise, if the head and tail aren't touching and aren't in the same row or column,
        // the tail always moves one step diagonally to keep up:
        else if (distance_x >= 1 && distance_y >= 1) && !(distance_x == 1 && distance_y == 1) {
            let mut directions_tail: Vec<char> = Vec::new();
            if position_head.x > position_tail.x {
                directions_tail.push('R')
            } else {
                directions_tail.push('L')
            }
            if position_head.y > position_tail.y {
                directions_tail.push('U')
            } else {
                directions_tail.push('D')
            }
            Some(directions_tail)
        } else {
            None
        }
    ;

    if directions_tail.is_some() {
        for direction in directions_tail.unwrap() {
            position_tail.move_step(&direction)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct RopeMovement {
    direction: char,
    amount: u32,
}

impl RopeMovement {
    fn new(line: String) -> Self {
        let mut split = line.split(" ");
        RopeMovement {
            direction: split.next().unwrap().chars().next().unwrap(),
            amount: split.next().unwrap().parse::<u32>().unwrap()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y
        }
    }

    fn move_step(&mut self, direction: &char) {
        match direction {
            'U' => {
                self.y = self.y + 1
            }
            'D' => {
                self.y = self.y - 1
            }
            'L' => {
                self.x = self.x - 1
            }
            'R' => {
                self.x = self.x + 1
            }
            _ => panic!("Unknown direction")
        }
    }
}

