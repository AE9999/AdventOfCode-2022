use std::collections::{HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cave = read_input(&args[1])?;
    println!("{:?} many units of sand come to rest before sand starts flowing into the abyss below!",
             cave.solve1());

    println!("{:?} many units of sand come to rest",
             cave.solve2());
    Ok(())
}

fn read_input(filename: &String) -> io::Result<Cave> {
    let file_in = File::open(filename)?;
    let mut rocks : Vec<Rock> = Vec::new();
    let lines: Vec<String> =
        BufReader::new(file_in)
                  .lines()
                  .map(|line|line.unwrap())
                  .collect::<Vec<String>>();

    for line in lines {
        let split_line = line.split(" -> ").collect::<Vec<&str>>();
        let points =
            split_line.iter().map(|&entry| {
                let mut it = entry.split(",").into_iter();
                Point {
                    x: it.next().unwrap().parse:: < usize>().unwrap(),
                    y: it.next().unwrap().parse:: < usize>().unwrap()
                }
            }).collect::<Vec<Point>>();

        let mut rocks_ =
            (1..points.len()).into_iter()
                             .map(|i| Rock::new(points[i-1].clone(), points[i].clone()))
                             .collect::<Vec<Rock>>();
        rocks.append(&mut rocks_);
    }

    Ok(Cave {
        rocks
    })
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize
 }

impl  Point {
    fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1
        }
    }

    fn down_and_left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y + 1
        }
    }

    fn down_and_right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y + 1
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Rock {
    ll: Point,
    ur: Point,
}

impl Rock {
    fn new(start: Point, end: Point) -> Self {
        if start.x == end.x {
            Rock {
                ll: Point { x: start.x, y: std::cmp::max(start.y, end.y) },
                ur: Point { x: start.x, y: std::cmp::min(start.y, end.y) },
            }
        } else {
            assert_eq!(start.y, end.y, "We should be working with straight lines here");
            Rock {
                ll: Point { x: std::cmp::min(start.x, end.x), y: start.y },
                ur: Point { x: std::cmp::max(start.x, end.x), y: start.y },
            }
        }
    }
}

struct Cave {
    rocks: Vec<Rock>,
}

impl Cave {

    fn solve1(&self) -> usize {

        let mut occupied_squares : HashSet<Point> = HashSet::new();

        let end_y = self.rocks.iter().map(|rock|rock.ll.y).max().unwrap();

        for rock in (&self.rocks).iter() {
            if rock.ll.x == rock.ur.x {
                for y in (rock.ur.y)..(rock.ll.y + 1) {
                    occupied_squares.insert(Point { x: rock.ll.x, y});
                }
            } else {
                assert_eq!(rock.ll.y, rock.ur.y, "We are supposed to be dealing with lines here");
                for x in (rock.ll.x)..(rock.ur.x + 1) {
                    occupied_squares.insert(Point { x, y: rock.ur.y});
                }
            }
        }

        let original_size = occupied_squares.len();

        loop {
            let mut current_point = Point { x: 500, y: 0 };
            loop {
                if current_point.y == end_y {
                    return occupied_squares.len() - original_size;
                }
                if !occupied_squares.contains(&current_point.down()) {
                    current_point = current_point.down();
                } else if !occupied_squares.contains(&current_point.down_and_left()) {
                    current_point = current_point.down_and_left();
                } else if !occupied_squares.contains(&current_point.down_and_right()) {
                    current_point = current_point.down_and_right();
                } else {
                    occupied_squares.insert(current_point.clone());
                    break;
                }
            }
        }
    }

    fn solve2(&self) -> usize {
        let mut occupied_squares : HashSet<Point> = HashSet::new();

        let end_y = self.rocks.iter().map(|rock|rock.ll.y).max().unwrap() + 2;

        for rock in (&self.rocks).iter() {
            if rock.ll.x == rock.ur.x {
                for y in (rock.ur.y)..(rock.ll.y + 1) {
                    occupied_squares.insert(Point { x: rock.ll.x, y});
                }
            } else {
                assert_eq!(rock.ll.y, rock.ur.y, "We are supposed to be dealing with lines here");
                for x in (rock.ll.x)..(rock.ur.x + 1) {
                    occupied_squares.insert(Point { x, y: rock.ur.y});
                }
            }
        }

        let original_size = occupied_squares.len();
        let beginning: Point = Point { x: 500, y: 0 };
        loop {
            let mut current_point = beginning.clone();
            loop {
                if !(occupied_squares.contains(&current_point.down()))
                    && !(current_point.down().y >= end_y) {
                    current_point = current_point.down();
                } else if !(occupied_squares.contains(&current_point.down_and_left()))
                            && !(current_point.down_and_left().y >= end_y) {
                    current_point = current_point.down_and_left();
                } else if !occupied_squares.contains(&current_point.down_and_right())
                            && !(current_point.down().y >= end_y) {
                    current_point = current_point.down_and_right();
                } else {
                    occupied_squares.insert(current_point.clone());
                    if current_point == beginning {
                        return occupied_squares.len() - original_size;
                    }
                    break;
                }
            }
        }
    }
}

