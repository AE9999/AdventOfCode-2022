extern crate core;

use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let inputs = read_input(&args[1])?;

    println!("{:?} units tall will the tower of rocks be after 2022 rocks have stopped falling ..",
             solve1(inputs.clone()));

    println!("{:?} units tall will the tower of rocks be after 1000000000000 rocks have stopped falling ..",
             solve2(inputs.clone()));
    Ok(())
}

fn solve1(input: Vec<char>) -> i64 {
    let mut simulation = Simulation::new(7, input);
    simulation.solve1()
}

fn solve2(input: Vec<char>) -> i64 {
    let mut simulation = Simulation::new(7, input.clone());
    simulation.solve2()
}

fn read_input(filename: &String) -> io::Result<Vec<char>> {
    let file_in = File::open(filename)?;

    let mut input =
        BufReader::new(file_in)
            .lines()
            .map(|line|line.unwrap())
            .take(1)
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

    Ok(input.remove(0))
}

#[derive(Debug, Clone)]
struct Simulation {
    settled_rocks: HashSet<Point>,
    highest_rock_y: i64,
    width: i64,
    jet_pattern: Vec<char>,
    nr_of_fallen_rocks: i64,
    nr_of_taken_steps: usize,
    current_dropping_rock: Option<CurrentDroppingRock>,
}

impl Simulation {
    fn new(width: i64,
           jet_pattern: Vec<char>) -> Self {

        Simulation {
            settled_rocks: HashSet::new(),
            highest_rock_y: -1,
            width,
            jet_pattern,
            nr_of_fallen_rocks: 0,
            nr_of_taken_steps: 0,
            current_dropping_rock: None,
        }
    }

    fn solve1(&mut self) -> i64 {
        loop {
            if self.nr_of_fallen_rocks == 2022 {
                return self.highest_rock_y + 1 // We work with indices not height
            }
            self.step()
        }
    }

    fn equilibrium_reached(differences_fallen_rocks_rock_between_phases: &Vec<i64>,
                           differences_hightest_rock_between_phases: &Vec<i64>) -> Option<usize> {
        for i in 1..(differences_fallen_rocks_rock_between_phases.len() / 2) {
            let it = differences_fallen_rocks_rock_between_phases.iter().rev();
            let left = it.take(i).collect::<Vec<&i64>>();

            let  it = differences_fallen_rocks_rock_between_phases.iter().rev();
            let right = it.skip(i).take(i).collect::<Vec<&i64>>();
            if left != right {
                continue;
            }

            let it = differences_hightest_rock_between_phases.iter().rev();
            let left = it.take(i).collect::<Vec<&i64>>();

            let  it = differences_hightest_rock_between_phases.iter().rev();
            let right = it.skip(i).take(i).collect::<Vec<&i64>>();
            if left != right {
                continue;
            }

            return Some(i)
        };
        None
    }

    fn solve2(&mut self) -> i64 {
        let mut p_highest_rock = self.highest_rock_y;
        let mut p_nr_of_fallen_rocks = self.nr_of_fallen_rocks;

        let mut differences_hightest_rock_between_phases: Vec<i64> = Vec::new();
        let mut differences_fallen_rocks_rock_between_phases: Vec<i64> = Vec::new();

        loop {
            while self.nr_of_taken_steps < self.jet_pattern.len() {
                self.step();
            }
            self.nr_of_taken_steps = 0;

            let diff_highest_rock = self.highest_rock_y - p_highest_rock;
            let diff_nr_of_fallen_rocks = self.nr_of_fallen_rocks - p_nr_of_fallen_rocks;

            differences_fallen_rocks_rock_between_phases.push(diff_nr_of_fallen_rocks);
            differences_hightest_rock_between_phases.push(diff_highest_rock);

            let eq =
                Simulation::equilibrium_reached(&differences_fallen_rocks_rock_between_phases,
                                                &differences_hightest_rock_between_phases);
            if eq.is_some() {

                let eq = eq.unwrap();

                let phase_height_growth =
                    differences_hightest_rock_between_phases.iter()
                        .rev()
                        .take(eq)
                        .map(|x|*x)
                        .sum::<i64>();
                let phase_amount_of_rocks =
                    differences_fallen_rocks_rock_between_phases.iter()
                        .rev()
                        .take(eq)
                        .map(|x|*x)
                        .sum::<i64>();

                let remaining_rocks_needed : i64 =
                    1000000000000 - self.nr_of_fallen_rocks;

                let remaining_phases_needed: i64 = remaining_rocks_needed / phase_amount_of_rocks;
                let remaining_rocks_needed: i64 = remaining_rocks_needed % phase_amount_of_rocks;

                p_highest_rock = self.highest_rock_y;

                p_nr_of_fallen_rocks = self.nr_of_fallen_rocks;

                while (self.nr_of_fallen_rocks - p_nr_of_fallen_rocks) < remaining_rocks_needed {
                    self.step();
                }
                let heigh_increase_in_final_stretch = self.highest_rock_y - p_highest_rock;

                let height_increase_in_phases = remaining_phases_needed * phase_height_growth;

                return p_highest_rock + height_increase_in_phases + heigh_increase_in_final_stretch + 1; // Yes because we count form 0
            }

            p_highest_rock = self.highest_rock_y;
            p_nr_of_fallen_rocks = self.nr_of_fallen_rocks;
        }
    }

    fn step(&mut self) {
        if self.current_dropping_rock.is_none()  {
            self.current_dropping_rock = Some(self.next_dropping_rock());
        }

        self.handle_jet_stream();

        self.handle_drop();
    }

    fn handle_jet_stream(&mut self) {
        let jet_stream: &char = &(self.jet_pattern[ self.nr_of_taken_steps % self.jet_pattern.len()]);

        self.nr_of_taken_steps = self.nr_of_taken_steps + 1;

        let mut current_dropping_rock = self.current_dropping_rock.as_ref().unwrap().clone();
        assert!(jet_stream == &'>' || jet_stream == &'<');
        let dx =
            if jet_stream == &'>' {
                1
            } else {
                -1
            };

        let forbidden_points =
            current_dropping_rock.occupied_squares()
                .iter()
                .map(|point| point.dxdy(dx, 0))
                .any(|point|self.forbidden(&point));
        if !forbidden_points {
            current_dropping_rock.ll = current_dropping_rock.ll.dxdy(dx, 0);
        }
        self.current_dropping_rock = Some(current_dropping_rock);
    }

    fn handle_drop(&mut self) {
        let mut current_dropping_rock = self.current_dropping_rock.as_ref().unwrap().clone();
        let dy = -1;

        let forbidden_points =
            current_dropping_rock.occupied_squares()
                .iter()
                .map(|point| point.dxdy(0, dy))
                .any(|point|self.forbidden(&point));

        if !forbidden_points {
            current_dropping_rock.ll = current_dropping_rock.ll.dxdy(0, dy);
            self.current_dropping_rock = Some(current_dropping_rock);
        } else {
            for settled_rock in  current_dropping_rock.occupied_squares() {
                self.highest_rock_y = max(self.highest_rock_y, settled_rock.y);
                self.settled_rocks.insert(settled_rock);
            }
            self.current_dropping_rock = None;
            self.nr_of_fallen_rocks = self.nr_of_fallen_rocks + 1;
        }
    }

    fn forbidden(&self, point: &Point) -> bool {
        point.y < 0
            || point.x >= self.width
            || point.x < 0
            || self.settled_rocks.contains(point)

    }

    fn next_dropping_rock(&mut self) -> CurrentDroppingRock {
        CurrentDroppingRock {
            rock_type: self.next_rock_type(),
            ll: self.next_rock_starting_place(),
        }
    }

    fn next_rock_type(&self)-> RockType {
        match self.nr_of_fallen_rocks % 5  {
            0 => RockType::RockTypeMinus,
            1 => RockType::RockTypePlus,
            2 => RockType::RockTypeAngle,
            3 => RockType::RockTypePipe,
            4 => RockType::RockTypeSquare,
            _ => panic!("Unreachable code")
        }
    }

    fn next_rock_starting_place(&self) -> Point {
        Point {
            x: 2,
            y: self.highest_rock_y + 4
        }
    }
}

#[derive(Debug, Clone)]
struct CurrentDroppingRock {
    rock_type: RockType,
    ll: Point,
}

impl CurrentDroppingRock {
    fn occupied_squares(&self) -> Vec<Point> {
        self.rock_type.occupied_squares(&self.ll)
    }
}

#[derive(Debug, Clone)]
enum RockType {
    RockTypeMinus,
    RockTypePlus,
    RockTypeAngle,
    RockTypePipe,
    RockTypeSquare,
}

impl RockType {
    fn occupied_squares(&self,  ll: &Point) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        match self {
            RockType::RockTypeMinus => {
                points.push(ll.clone());
                points.push(ll.dxdy(1, 0));
                points.push(ll.dxdy(2, 0));
                points.push(ll.dxdy(3, 0));
            },
            RockType::RockTypePlus=> {
                points.push(ll.dxdy(1, 0));
                points.push(ll.dxdy(0, 1));
                points.push(ll.dxdy(1, 1));
                points.push(ll.dxdy(2, 1));
                points.push(ll.dxdy(1, 2));
            },
            RockType::RockTypeAngle=> {
                points.push(ll.clone());
                points.push(ll.dxdy(1, 0));
                points.push(ll.dxdy(2, 0));
                points.push(ll.dxdy(2, 1));
                points.push(ll.dxdy(2, 2));
            },
            RockType::RockTypePipe=> {
                points.push(ll.clone());
                points.push(ll.dxdy(0, 1));
                points.push(ll.dxdy(0, 2));
                points.push(ll.dxdy(0, 3));
            },
            RockType::RockTypeSquare=> {
                points.push(ll.clone());
                points.push(ll.dxdy(0, 1));
                points.push(ll.dxdy(1, 0));
                points.push(ll.dxdy(1, 1));
            }
        }
        points
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn dxdy(&self, dx: i64, dy: i64) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}
