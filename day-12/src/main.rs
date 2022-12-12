use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

use lazy_static::lazy_static;

lazy_static! {
    static ref CHAR_VALUE_HASHMAP: HashMap<char, u64> = {
        let mut m : HashMap<char, u64> = HashMap::new();
        ('a'..='z').into_iter().enumerate().for_each(|(i,c)|{
            m.insert(c, (i + 1) as u64);
        });
        m
    };
}

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let problem = read_input(&args[1])?;

    println!("{:?} is the fewest steps required to move from your current position to the location that should get the best signal",
             find_path_lenght(&problem, &problem.initial_start_point));

    //
    // Yes the optimal solution would be to do some reverse stuff, going back for 'E' to see if we can reach a and than find the minimal one.
    // However I am very lazy and rust is very fast so this still runs in reasonable time :-)
    //
    println!("{:?} is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal",
             problem.possible_starting_points
                 .iter()
                 .enumerate()
                 .map(|point| {
                     println!("Solving {:?} / {:?} ..", point.0, problem.possible_starting_points.len());
                     find_path_lenght(&problem, point.1)
                 })
                 .min()
                 .unwrap()
    );

    Ok(())
}

fn find_path_lenght(problem: &Problem, starting_point: &Point) -> usize {

    println!("Testing {:?}", starting_point);

    let mut points_2_weight: HashMap<Point, usize> = HashMap::new();
    let mut stack: Vec<Point> = Vec::new();

    points_2_weight.insert(starting_point.clone(), 0);
    stack.push(starting_point.clone());

    loop {

        if stack.is_empty() {
            break
        }

        let point = stack.pop().unwrap();

        let path_length = *points_2_weight.get(&point).unwrap() + 1;

        for neighbour in problem.neighbours(&point) {
            let neighbour_weight = points_2_weight.get(&neighbour);

            if problem.is_accessible(&point, &neighbour)
               && (neighbour_weight.is_none()
                   || (*neighbour_weight.unwrap() > path_length)) {

                points_2_weight.insert(neighbour.clone(), path_length);
                stack.push(neighbour.clone());

            }
        }

    }

    *points_2_weight.get(&problem.end_point).unwrap_or(& usize::MAX) // not guaranteed to there be a path
}

fn read_input(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;


    let squares =
        BufReader::new(file_in).lines()
            .map(|line| line.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

    Ok(Problem::new(squares))
}

#[derive(Debug, Clone)]
struct Problem {
    squares: Vec<Vec<char>>,
    initial_start_point: Point,
    possible_starting_points: Vec<Point>,
    end_point: Point,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x, y
        }
    }
}

impl Problem {
    fn new(mut squares: Vec<Vec<char>>) -> Self {

        let mut end_point :Option<Point> = None;
        let mut initial_start_point   :Option<Point> = None;
        let mut possible_starting_points: Vec<Point> = Vec::new();

        for y in 0..squares.len() {
            for x in 0..squares[y].len() {
                if squares[y][x] == 'S' {
                    squares[y][x] = 'a';
                    initial_start_point = Some(Point::new(x,y));
                } else if squares[y][x] == 'E' {
                    end_point = Some(Point::new(x,y));
                } else if squares[y][x] == 'a' {
                    possible_starting_points.push(Point::new(x,y))
                }
            }
        }

        assert!(end_point.is_some());
        assert!(initial_start_point.is_some());

        Problem {
            initial_start_point: initial_start_point.unwrap(),
            end_point: end_point.unwrap(),
            possible_starting_points,
            squares
        }
    }

    fn height(&self) -> usize {
        self.squares.len()
    }

    fn width(&self) -> usize {
        self.squares[0].len()
    }

    fn char_at(&self, point: &Point) -> char {
        self.squares[point.y][point.x]
    }

    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for x in  ( (point.x as i32) -1)..((point.x as i32) + 2) {
            for y in  ( (point.y as i32) -1)..((point.y as i32) + 2) {
                if x >= 0
                   && y >= 0
                   && (x as usize) < self.width()
                   && (y as usize) < self.height()
                   && (x != (point.x as i32) || y !=  (point.y as i32))
                    && (x == (point.x as i32) || y ==  (point.y as i32)) {
                    points.push(Point::new(x as usize, y as usize));
                }
            }
        }
        points
    }

    fn is_accessible(&self, from: &Point, to: &Point) -> bool {

        let char_at_from = self.char_at(from);

        if char_at_from == 'S' { return true } // We can always move from s

        if char_at_from == 'E' { return false } // We can never move from e

        let char_at_to = self.char_at(to);

        if char_at_to == 'S' { return false } // We can never go back to s

        let from_value = *CHAR_VALUE_HASHMAP.get(&char_at_from).unwrap();

        let to_value = if char_at_to == 'E'  {
            *CHAR_VALUE_HASHMAP.get(&'z').unwrap()
        } else {
            *CHAR_VALUE_HASHMAP.get(&char_at_to).unwrap()
        };

        return to_value - 1 <= from_value
    }
}