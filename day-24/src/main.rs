use std::collections::BTreeSet;  // If we want to hash states we need order stuff
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let problem  = read_input(&args[1])?;

    println!("{:?} is the fewest number of minutes required to avoid the blizzards and reach the goal",
             0);

    Ok(())
}

fn read_input(filename: &String) -> io::Result<ProblemState> {
    let file_in = File::open(filename)?;

    let lines =
        BufReader::new(file_in)
            .lines()
            .map(|line| line.unwrap())
            .map(|line|line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

    let mut walls: BTreeSet<Point> = BTreeSet::new();

    let mut blizzards: BTreeSet<Blizzard> = BTreeSet::new();

    let mut start: Option<Point> = None;

    let mut end: Option<Point> = None;

    let width = lines[0].len() as i32;

    let height = lines.len() as i32;

    for y in 0..lines.len() {
        let line = &(lines[y]);
        for x in 0..line.len() {
            let char = line[x];
            let point = Point { x: x as i32, y: y as i32};
            match char {
                '#' => {
                    walls.insert(point );
                },
                '.' => {
                    if y == 0 {
                        start = Some(point);
                    } else if y == lines.len() -1 {
                        end = Some(point);
                    };
                },
                '>' => {
                    blizzards.insert( Blizzard{ point, direction: Direction::Right });
                },
                '<' => {
                    blizzards.insert(Blizzard{ point, direction: Direction::Left } );
                },
                '^' => {
                    blizzards.insert(Blizzard{ point, direction: Direction::Up });
                },
                'v' => {
                    blizzards.insert(Blizzard{ point, direction: Direction::Down });
                },
                _ => {
                    panic!("Unexpected input");
                }
            }
        }
    }

    let problem_state = ProblemState {
        width,
        height,
        walls,
        blizzards,
        my_current_location: start.as_ref().unwrap().clone(),
        end: end.unwrap(),
        start: start.unwrap(),
    };

    Ok(problem_state)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProblemState {
    width: i32,
    height: i32,
    walls: BTreeSet<Point>,
    blizzards: BTreeSet<Blizzard>,
    my_current_location: Point,
    end: Point,
    start: Point,
}

impl ProblemState {

    fn step(&mut self) -> bool {
        let next_blizzards : BTreeSet<Blizzard> = BTreeSet::new();

        for blizzard in self.blizzards.iter() {
            let next_point = blizzard.point.point_in_direction(&blizzard.direction);
            assert_ne!(next_point, self.start);
            let next_point =
                if self.walls.contains(&next_point) {
                    match &blizzard.direction {
                        Direction::Up =>  Point {
                            x: next_point.x,
                            y: self.height - 2,
                        },
                        Direction::Down  => Point {
                            x: next_point.x,
                            y: 1,
                        },
                        Direction::Left  => Point {
                            x: self.width - 2,
                            y: next_point.y,
                        },
                        Direction::Right  => Point {
                            x: 1,
                            y: next_point.y,
                        },
                    }
                } else {
                    next_point
                }
            ;
            if next_point == self.my_current_location {
                // Player got killed
                return false;
            }
        };

        self.blizzards = next_blizzards;

        true
    }

    fn player_can_move(&self, point: &Point) -> bool {
        !self.walls.contains(point)
        && !self.blizzards.iter().any (|blizzard| &blizzard.point == point)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Blizzard {
    point: Point,
    direction: Direction,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dxdy(&self, dxdy: (i32, i32)) -> Point {
        Point {
            x: self.x + dxdy.0,
            y: self.y + dxdy.1,
        }
    }

    fn point_in_direction(&self, direction: &Direction) -> Point {
        self.dxdy(direction.to_dxdy())
    }

    fn neighbours(&self) -> Vec<Point> {
        vec!(
            self.point_in_direction(&Direction::Up),
            self.point_in_direction(&Direction::Down),
            self.point_in_direction(&Direction::Left),
            self.point_in_direction(&Direction::Right),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_dxdy(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down  => (0, 1),
            Direction::Left  => (-1, 0),
            Direction::Right  => (1, 0),
        }
    }
}


