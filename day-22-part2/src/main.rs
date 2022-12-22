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
    println!("Standing at {:?} ", &(problem.position));
    loop {
        problem.step();
        println!("Moved to {:?} ", &(problem.position));
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
                let off_map = self.is_off_map(&point);

                let (point, direction) =
                    if off_map {
                        self.calculate_arrival_point(&self.position, &self.direction)
                    } else {
                        (point, self.direction.clone())
                    };

                if off_map {
                    println!("Trying to see if we reverse we would be back ..");
                    let step_back =
                        self.calculate_arrival_point(&point, &direction.inverse());
                    assert_eq!(step_back.0,
                               self.position,
                              "Reverse does not equal starting position. please debug ..Calculated Point");
                }

                assert!(self.is_wall(&point) ^ self.is_empty(&point));

                // println!("Working with {:?}, {:?} ..", point, self.get_char(&point));

                if self.is_empty(&point) {
                    self.position = point;
                    self.direction = direction
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

    fn calculate_face_of_cube(&self, point: &Point) -> u32 {
        assert!(!self.is_off_map(point));
        let l_unit = (self.len() / 4) as i32;
        let h_unit = (self.height() / 3) as i32;
        assert_eq!( (self.len() / 4), (self.height() / 3));


        if point.y < h_unit {
            assert!(point.x >= l_unit * 2 && point.x < l_unit * 3);
            1
        } else if point.y >= h_unit * 2 {
            assert!(point.x >= l_unit * 2);
            if point.x >= l_unit * 3 {
                6
            } else {
                5
            }
        } else {
            assert!(point.y >= l_unit && point.x < l_unit * 3, "{}", format!("Point {:?} does not lie in 2,3 or 4, {:?}", point, l_unit));
            if point.x < l_unit {
                2
            } else if point.x < l_unit * 2 {
                3
            } else {
                4
            }
        }
    }

    fn len(&self) -> usize {
        self.tiles.iter().map(|tile| tile.len()).max().unwrap()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn calculate_arrival_point(&self, point: &Point, direction: &Direction) -> (Point, Direction) {
        let w_unit = (self.len() / 4) as i32;
        let h_unit = (self.height() / 3) as i32;
        let face = self.calculate_face_of_cube(point);

        println!("Calculating the next entry point for point:{:?} on face: {:?} with direction:{:?} ..",
                 point,
                 face,
                 direction);
        assert!(self.is_off_map(&point.next(direction)));
        assert_eq!(w_unit, h_unit);
        match face {
            1 => {
                match direction {

                    Direction::Up => {
                        // 1 -> 2
                        let offset  = point.x - (3 * w_unit);
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: offset,
                            y: h_unit
                        };
                        let direction = Direction::Down;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 2);
                        (entry_point, direction)
                    },
                    Direction::Left => {
                        // 1 -> 3
                        let offset = point.y;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: h_unit + offset,
                            y: h_unit,
                        };
                        let direction = Direction::Down;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 3);
                        (entry_point, direction)
                    },
                    Direction::Right => {
                        // 1 -> 6
                        let offset = h_unit - point.y - 1;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: 2 * w_unit + offset,
                            y: 2 * h_unit,
                        };
                        let direction = Direction::Left;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 6);
                        (entry_point, direction)
                    },
                    _ => panic!("Should be unreachable for direction {:?}", direction)
                }
            },
            2 => {
                match direction {
                    Direction::Up => {
                        // 2 -> 1
                        let offset = point.x;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: 3 * w_unit - 1 - offset,
                            y: 0,
                        };
                        let direction = Direction::Down;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 6);
                        (entry_point, direction)
                    },
                    Direction::Down => {
                        // 2 -> 5
                        let offset = w_unit - point.x - 1;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: 2 * w_unit + offset,
                            y: ((self.height() -1) as i32)
                        };
                        let direction = Direction::Up;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 5);
                        (entry_point, direction)
                    },
                    Direction::Left => {
                        // 2 -> 6
                        let offset = point.y - w_unit - 1;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: (self.len() - 1) as i32,
                            y: 2 * w_unit -1 + offset,

                        };
                        let direction = Direction::Up;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 6);
                        (entry_point, direction)
                    },
                    _ => panic!("Should be unreachable for direction {:?}", direction)
                }
            },
            3 => {
                match direction {
                    Direction::Up => {
                        // 3 -> 1
                        let offset = point.x - w_unit;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: 2 * w_unit,
                            y: offset,
                        };
                        let direction = Direction::Right;
                        println!("\t generated {:?} ..",entry_point);

                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 1);
                        (entry_point, direction)
                    },
                    Direction::Down => {
                        // 3 -> 5
                        let offset = (2 * w_unit -1) -  point.x;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: 2 * w_unit,
                            y: (2 * h_unit) + offset,
                        };
                        let direction = Direction::Right;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 5);
                        (entry_point, direction)
                    },
                    _ => panic!("Should be unreachable for direction {:?}", direction)
                }
            },
            4 => {
                match direction {
                    Direction::Right => {
                        // 4 -> 6
                        let offset = point.y - h_unit;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: (self.len() as i32) - 1 - offset,
                            y: (2 * h_unit),
                        };
                        let direction = Direction::Down;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 6);
                        (entry_point, direction)
                    },
                    _ => panic!("Should be unreachable for direction {:?}", direction)
                }
            },
            5 => {
                match direction {
                    Direction::Down => {
                        // 5 -> 2
                        let offset = point.x - 2 * w_unit - 1;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: offset,
                            y: w_unit * 2 -1,
                        };
                        let direction = Direction::Up;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 2);
                        (entry_point, direction)
                    },
                    Direction::Left => {
                        // 5 -> 3
                        let offset = point.y - (3 * h_unit - 1);
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x:  w_unit + offset,
                            y: w_unit * 2 - 1,
                        };
                        let direction = Direction::Up;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 3);
                        (entry_point, direction)
                    },
                    _ => panic!("Should be unreachable for direction {:?}", direction)
                }
            },
            6 => {
                match direction {
                    Direction::Up => {
                        // 6 -> 4
                        let offset = point.x - 3 * w_unit;
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: 3 * w_unit - 1,
                            y: 2 * h_unit - 1 - offset,
                        };
                        let direction = Direction::Left;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 4);
                        (entry_point, direction)
                    },
                    Direction::Down => {
                        // 6 -> 2
                        let offset = point.x - (3 * w_unit);
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: 0,
                            y: 2 * w_unit - offset -1,
                        };
                        let direction = Direction::Right;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 2);
                        (entry_point, direction)
                    },
                    Direction::Right => {
                        // 6 -> 1
                        let offset = point.y - (2 * h_unit);
                        assert!(offset < w_unit && offset >= 0);
                        let entry_point = Point {
                            x: 3 * w_unit - 1,
                            y: w_unit - offset - 1,
                        };
                        let direction = Direction::Left;
                        println!("\t --> {:?} on surface ({:?}) with direction {:?} ..",
                                 entry_point,
                                 self.calculate_face_of_cube(&entry_point),
                                 direction);
                        assert_eq!(self.calculate_face_of_cube(&entry_point), 1);
                        (entry_point, direction)
                    },
                    _ => panic!("Should be unreachable for direction {:?}", direction)
                }
            },
            _ => panic!("Should be unreachable face: {:?}", face)
        }
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

    fn inverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down  => Direction::Up,
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left,
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
