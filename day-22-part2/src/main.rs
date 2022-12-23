extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let problem  = read_input(&args[1])?;
    println!("{:?} is the final password", solve2(problem.clone()));

    Ok(())
}

fn solve2(mut problem: Problem) -> i32 {

    loop {
        problem.step();
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
    jump_map: HashMap<(Point, Direction), (Point, Direction)>,
}

impl Problem {

    fn populate_jump_map(left_edge: Vec<Point>,
                         incoming_direction: Direction,
                         right_edge:Vec<Point>,
                         outgoing_direction: Direction,
                         reverse: bool,
                         jump_map: &mut HashMap<(Point, Direction), (Point, Direction)>) {
        for i in 0..left_edge.len() {
            let left = left_edge[i].clone();
            let right =
                if reverse {
                    right_edge[right_edge.len() -1 - i].clone()
                } else {
                    right_edge[i].clone()
                };
            jump_map.insert((left, incoming_direction.clone()),
                             (right, outgoing_direction.clone()));
        }
    }

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

        // Unique for my input I guess

        let sq_size = (tiles.len() / 4) as i32;

        let base_ll =
            Point {
                x: 0,
                y: (tiles.len() -1) as i32,
            };

        let lls: Vec<Point> =
         vec!(
             // 1
            base_ll.dxdy(0, 0),

             // 2
            base_ll.dxdy(0, -sq_size),

             // 3
             base_ll.dxdy(sq_size, -sq_size),

             // 4
            base_ll.dxdy(sq_size, -2 * sq_size),

             // 5
             base_ll.dxdy(sq_size, -3 * sq_size),

             // 6
             base_ll.dxdy(2 * sq_size, -3 * sq_size),
         );

        let squares: Vec<Square> =
            lls.iter().map(|ll| Square {
                ll: ll.clone(),
                size: sq_size as i32,
            }).collect::<Vec<Square>>();

        let mut jump_map: HashMap<(Point, Direction), (Point, Direction)> = HashMap::new();

        // 1 -> 3
        Problem::populate_jump_map(
            squares[0].get_edge(Edge::Right),
            Direction::Right,
            squares[2].get_edge(Edge::Down),
            Direction::Up,
            true,
            &mut jump_map);

        // 3 -> 1
        Problem::populate_jump_map(
            squares[2].get_edge(Edge::Down),
            Direction::Down,
            squares[0].get_edge(Edge::Right),
            Direction::Left,
            true,
            &mut jump_map);

        // 1 -> 6
        Problem::populate_jump_map(
            squares[0].get_edge(Edge::Down),
            Direction::Down,
            squares[5].get_edge(Edge::Up),
            Direction::Down,
            false,
            &mut jump_map);

        // 6 -> 1
        Problem::populate_jump_map(
            squares[5].get_edge(Edge::Up),
            Direction::Up,
            squares[0].get_edge(Edge::Down),
            Direction::Up,
            false,
            &mut jump_map);

        // 2 -> 4
        Problem::populate_jump_map(
            squares[1].get_edge(Edge::Up),
            Direction::Up,
            squares[3].get_edge(Edge::Left),
            Direction::Right,
            true,
            &mut jump_map);


        // 4 -> 2
        Problem::populate_jump_map(
            squares[3].get_edge(Edge::Left),
            Direction::Left,
            squares[1].get_edge(Edge::Up),
            Direction::Down,
            true,
            &mut jump_map);

        // 2 -> 5
        Problem::populate_jump_map(
            squares[1].get_edge(Edge::Left),
            Direction::Left,
            squares[4].get_edge(Edge::Left),
            Direction::Right,
            true,
            &mut jump_map);


        // 5 -> 2
        Problem::populate_jump_map(
            squares[4].get_edge(Edge::Left),
            Direction::Left,
            squares[1].get_edge(Edge::Left),
            Direction::Right,
            true,
            &mut jump_map);

        // 3 -> 6
        Problem::populate_jump_map(
        squares[2].get_edge(Edge::Right),
        Direction::Right,
        squares[5].get_edge(Edge::Right),
        Direction::Left,
        true,
        &mut jump_map);


        // 6 -> 3
        Problem::populate_jump_map(
        squares[5].get_edge(Edge::Right),
        Direction::Right,
        squares[2].get_edge(Edge::Right),
        Direction::Left,
        true,
        &mut jump_map);

        // 4 -> 6
        Problem::populate_jump_map(
        squares[3].get_edge(Edge::Right),
        Direction::Right,
        squares[5].get_edge(Edge::Down),
        Direction::Up,
        true,
        &mut jump_map);

        // 6 -> 4
        Problem::populate_jump_map(
        squares[5].get_edge(Edge::Down),
        Direction::Down,
        squares[3].get_edge(Edge::Right),
        Direction::Left,
        true,
        &mut jump_map);

        // 1 -> 5
        Problem::populate_jump_map(
        squares[0].get_edge(Edge::Left),
        Direction::Left,
        squares[4].get_edge(Edge::Up),
        Direction::Down,
        true,
        &mut jump_map);

        // 5 -> 1
        Problem::populate_jump_map(
        squares[4].get_edge(Edge::Up),
        Direction::Up,
        squares[0].get_edge(Edge::Left),
        Direction::Right,
        true,
        &mut jump_map);

        Problem {
            tiles,
            input,
            position: Point { x, y },
            direction: Direction::Right,
            instruction_index: 0,
            jump_map,
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
            for _ in 0..movement {
                let point = self.position.next(&self.direction);
                let off_map = self.is_off_map(&point);

                let (point, direction) =
                    if off_map {
                        self.calculate_arrival_point_and_direction(&self.position, &self.direction)
                    } else {
                        (point, self.direction.clone())
                    };

                assert!(self.is_wall(&point) ^ self.is_empty(&point));

                // println!("Working with {:?}, {:?} ..", point, self.get_char(&point));

                if self.is_empty(&point) {
                    self.position = point;
                    self.direction = direction
                }
            }
        } else {
            self.direction = self.direction.next(&self.input[self.instruction_index]);

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

    fn calculate_arrival_point_and_direction(&self, point: &Point, direction: &Direction) -> (Point, Direction) {
        self.jump_map.get(&(point.clone(), direction.clone())).unwrap().clone()
    }
}

#[derive(Debug, Clone)]
struct Square {
    ll: Point,
    size: i32,
}

impl Square {

    fn get_edge(&self,
                edge: Edge) -> Vec<Point> {
        match edge {
            Edge::Left => {
                (0..self.size).into_iter()
                                .map(|i| {
                                    Point {
                                        x: self.ll.x,
                                        y: self.ll.y - i,
                                    }
                                }).collect::<Vec<Point>>()
            },
            Edge::Right => {
                (0..self.size).into_iter()
                    .map(|i| {
                        Point {
                            x: self.ll.x + (self.size - 1),
                            y: self.ll.y - i,
                        }
                    }).collect::<Vec<Point>>()
            },
            Edge::Up => {
                (0..self.size).into_iter()
                    .map(|i| {
                        Point {
                            x: self.ll.x + i,
                            y: self.ll.y - (self.size - 1),
                        }
                    }).collect::<Vec<Point>>()
            },
            Edge::Down => {
                (0..self.size).into_iter()
                    .map(|i| {
                        Point {
                            x: self.ll.x + i,
                            y: self.ll.y,
                        }
                    }).collect::<Vec<Point>>()
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Edge {
    Left,
    Right,
    Up,
    Down,
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

    fn dxdy(&self, dx: i32, dy: i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
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
