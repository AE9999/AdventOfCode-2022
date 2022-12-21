extern crate core;

use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let problem  = read_input(&args[1])?;
    println!("{:?} is the final password", solve1(problem.clone()));

    Ok(())
}

fn solve1(mut problem: Problem) -> i32 {
    loop {
        // println!("Standing at {:?} ", &(problem.position));
        problem.step();
        // println!("Moved to {:?} ", &(problem.position));
        if problem.done() {
            return problem.password()
        }
    }
}

fn read_input(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;

    let mut reading_tiles = true;

    let mut tiles : Vec<Vec<char>> = vec!();

    let mut it =
        BufReader::new(file_in).lines().map(|line| line.unwrap()).into_iter();
    loop {
        let s = it.next();
        if s.is_none() { panic!("Unexpected input") }
        let s = s.unwrap();
        if s.is_empty() {
            reading_tiles = false;
            continue;
        }
        if reading_tiles {
            tiles.push(s.chars().collect::<Vec<char>>())
        } else {
            return Ok(Problem::new(tiles, s.chars().collect::<Vec<char>>()))
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    tiles: Vec<Vec<char>>,
    input: Vec<char>,
    position: Point,
    direction: Direction,
    instruction_index: usize,
}

impl Problem {
    fn new(tiles: Vec<Vec<char>>, input: Vec<char>) -> Self {
        let y: i32  = 0;
        let x =
            (&tiles)[0].iter()
                       .enumerate()
                       .filter(|c| c.1 == &'.')
                       .take(1)
                       .map(|c|c.0 as i32)
                       .next()
                       .unwrap();

        Problem {
            tiles,
            input,
            position: Point { x, y },
            direction: Direction::Right,
            instruction_index: 0,
        }
    }

    fn step(&mut self) {
        assert!(self.instruction_index < self.input.len());

        let instruction = &self.input[self.instruction_index];

        if instruction.is_digit(10) {
            let mut instruction_buffer = String::new();
            loop {
                instruction_buffer.push(self.input[self.instruction_index].clone());
                self.instruction_index = self.instruction_index + 1;
                if self.instruction_index >= self.input.len()
                   || !self.input[self.instruction_index].is_digit(10) {
                    break;
                }
            }

            let movement = instruction_buffer.parse::<usize>().unwrap();
            // println!("Moving {:?} positions ..", movement);
            for _ in 0..movement {
                let point = self.position.next(&self.direction);
                let point =
                    if self.is_off_map(&point) {
                        self.calculate_entry_point(&point, &self.direction)
                    } else {
                        point
                    };
                // println!("Working with {:?}, {:?} ..", point, self.get_char(&point));
                assert!(self.is_wall(&point) ^ self.is_empty(&point));
                if self.is_empty(&point) {
                    self.position = point;
                }
            }
        } else {
            self.direction = self.direction.next(&self.input[self.instruction_index]);

            // println!("Moving to the {:?} direction is now {:?}",
            //         &self.input[self.instruction_index],
            //         self.direction);

            self.instruction_index = self.instruction_index + 1;
        }

        assert!(self.position.x >= 0 && self.position.y >= 0);
    }

    fn done(&self) -> bool {
        self.instruction_index >= self.input.len()
    }

    fn password(&self) -> i32 {
        (1000 * (self.position.y + 1) ) +  (4 * (self.position.x + 1)) + self.direction.to_password()
    }

    fn get_char(&self , point: &Point) -> Option<char> {
        if point.y < 0 || point.x < 0 {
            return None;
        }
        let row =  (&self.tiles).get(point.y as usize);
        if row.is_none() {
            None
        } else {
            let row = row.unwrap();
            let char = row.get(point.x as usize);
            if char.is_none() {
                None
            } else {
                Some(char.unwrap().clone())
            }
        }
    }

    fn is_wall(&self, point: &Point) -> bool {
        let char = self.get_char(point);
        if char.is_none() {
            false
        } else {
            char.unwrap() == '#'
        }
    }

    fn is_empty(&self, point: &Point) -> bool {
        let char = self.get_char(point);
        if char.is_none() {
            false
        } else {
            char.unwrap() == '.'
        }
    }

    fn is_off_map(&self, point: &Point) -> bool {
        let char = self.get_char(point);
        char.is_none() || char.unwrap() == ' '
    }

    fn len(&self) -> usize {
        self.tiles.iter().map(|tile| tile.len()).max().unwrap()
    }

    fn calculate_entry_point(&self, point: &Point, direction: &Direction) -> Point {
        assert!(self.is_off_map(point));
        match direction {
            Direction::Up => {
                for y in (0..self.tiles.len()).into_iter().rev() {
                    let candidate = Point { x: point.x, y: y as i32  };
                    if !self.is_off_map(&candidate) {
                        return candidate
                    }
                }
            },
            Direction::Down => {
                for y in (0..self.tiles.len()).into_iter() {
                    let candidate = Point { x: point.x, y: y as i32 };
                    if !self.is_off_map(&candidate) {
                        return candidate
                    }
                }
            },
            Direction::Left => {
                for x in (0..self.len()).into_iter().rev() {
                    let candidate = Point { x: x as i32, y: point.y };
                    if !self.is_off_map(&candidate) {
                        return candidate
                    }
                }
            },
            Direction::Right  => {
                for x in (0..self.len()).into_iter() {
                    let candidate = Point { x: x as i32, y: point.y };
                    if !self.is_off_map(&candidate) {
                        return candidate
                    }
                }
            },
        };
        panic!("Should not reach")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn next(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => { Point { x: self.x, y: self.y - 1 } },
            Direction::Down => { Point { x: self.x, y: self.y + 1 } },
            Direction::Left => { Point { x: self.x -1, y: self.y } },
            Direction::Right  => { Point { x: self.x + 1, y: self.y } },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_password(&self) -> i32 {
        match self {
            Direction::Up => 3,
            Direction::Down  => 1,
            Direction::Left  => 2,
            Direction::Right => 0,
        }

    }

    fn next(&self, direction: &char) -> Self {
        match self {
            Direction::Up => {
                match direction {
                    'R' => { Direction::Right },
                    'L' => { Direction::Left },
                    _ => panic!("unexpected input")
                }
            },
            Direction::Down=> {
                match direction {
                    'R' => { Direction::Left },
                    'L' => { Direction::Right },
                    _ => panic!("unexpected input")
                }
            },
            Direction::Left=> {
                match direction {
                    'R' => { Direction::Up },
                    'L' => { Direction::Down },
                    _ => panic!("unexpected input")
                }
            },
            Direction::Right=> {
                match direction {
                    'R' => { Direction::Down },
                    'L' => { Direction::Up },
                    _ => panic!("unexpected input")
                }
            },
        }
    }
}
