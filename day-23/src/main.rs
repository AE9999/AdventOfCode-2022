use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let problem  = read_input(&args[1])?;
    println!("{:?} is how many empty ground tiles that rectangle contain",
             problem.clone().solve1());

    println!("{:?} is the number of the first round where no Elf moves",
             problem.clone().solve2());
    Ok(())
}

fn read_input(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;

    let lines =
        BufReader::new(file_in)
              .lines()
              .map(|line| line.unwrap())
              .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

    Ok(Problem::new(lines))
}

#[derive(Clone)]
struct Problem {
    elves: HashSet<Point>,
    suggestions: Vec<(Direction, Direction, Direction)>,
}

impl Problem {

    fn new(lines: Vec<Vec<char>>) -> Self {
        let mut elves: HashSet<Point> = HashSet::new();
        for y in 0..(lines.len()) {
            let line = &lines[y];
            for x in 0..(line.len()) {
                match line[x] {
                    '.' => {
                        // do noting
                    },
                    '#' => {
                        elves.insert(Point {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    _ => {
                        panic!("Unexpected input")
                    }
                };
            }
        }

        let suggestions: Vec<(Direction, Direction, Direction)> =
            vec!(
                (Direction::North, Direction::NorthEast, Direction::NorthWest),
                (Direction::South, Direction::SouthEast, Direction::SouthWest),
                (Direction::West, Direction::NorthWest, Direction::SouthWest),
                (Direction::East, Direction::NorthEast, Direction::SouthEast),
            );

        Problem {
            elves,
            suggestions,
        }
    }

    fn step(&mut self) -> bool {

        let mut elf_2_next_move: HashMap<Point, Point> = HashMap::new();
        let mut point_2_proposed_suggestions: HashMap<Point, usize> = HashMap::new();

        /*
            During the first half of each round, each Elf considers the eight positions adjacent to themself. If no other Elves are in one of those eight positions, the Elf does not do anything during this round. Otherwise, the Elf looks in each of four directions in the following order and proposes moving one step in the first valid direction:

            If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
            If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
            If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
            If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
         */

        for elf in &self.elves {

            let mut proposal: Option<Point> = None;

            // If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
            let has_neighbours =
                elf.neighbours()
                   .iter()
                   .any(|neighbour|self.elves.contains(neighbour));

            if has_neighbours {
                for i in 0..self.suggestions.len() {
                    if !self.elves.contains(&elf.dxdy(&self.suggestions[i].0))
                        && !self.elves.contains(&elf.dxdy(&self.suggestions[i].1))
                        && !self.elves.contains(&elf.dxdy(&self.suggestions[i].2)) {
                        proposal = Some(elf.dxdy(&self.suggestions[i].0));
                        break;
                    }
                }
            }

            if proposal.is_some() {
                let proposal = proposal.unwrap();
                elf_2_next_move.insert(elf.clone(), proposal.clone());

                if point_2_proposed_suggestions.get(&proposal).is_none() {
                    point_2_proposed_suggestions.insert(proposal.clone(), 0);
                }

                let current_value = *point_2_proposed_suggestions.get(&proposal).unwrap();
                point_2_proposed_suggestions.insert(proposal, current_value + 1);
            }
        }

        let mut next_elves : HashSet<Point> = HashSet::new();

        /*
        After each Elf has had a chance to propose a move, the second half of the round can begin.
        Simultaneously, each Elf moves to their proposed destination tile if they were the only Elf to propose moving to that position.
        If two or more Elves propose moving to the same position, none of those Elves move.
         */
        let mut elf_moved = false;
        for elf_move in &self.elves {
            if !elf_2_next_move.contains_key(elf_move) {
                next_elves.insert(elf_move.clone());
                continue;
            }

            let next_point = elf_2_next_move.get(elf_move).unwrap();
            if point_2_proposed_suggestions.get(next_point).unwrap() == &1  {
                next_elves.insert(next_point.clone());
                elf_moved = true
            } else {
                next_elves.insert(elf_move.clone());
            }
        }
        assert_eq!(self.elves.len(), next_elves.len(), "We can't lose any elves");
        self.elves = next_elves;

        // Finally, at the end of the round, the first direction the Elves considered is moved to the end of the list of directions.
        let last_suggestion = self.suggestions.remove(0);
        self.suggestions.push(last_suggestion);

        //println!("Suggestions: {:?} ..", self.suggestions);

        elf_moved
    }

    fn covered_ground(&self) -> usize {
        let ll_x =
            self.elves.iter().map(|point| point.x).min().unwrap();
        let ll_y =
            self.elves.iter().map(|point| point.y).min().unwrap();
        let ur_x =
            self.elves.iter().map(|point| point.x).max().unwrap();
        let ur_y =
            self.elves.iter().map(|point| point.y).max().unwrap();

        let lenght = (ur_x - ll_x).abs() + 1;
        let height = (ur_y - ll_y).abs() + 1;

        ((height * lenght) as usize) - self.elves.len()
    }

    fn solve1(&mut self) -> usize {
        for _ in 0..10 {
            self.step();
        }

        self.covered_ground()
    }

    fn solve2(&mut self) -> usize {
        let mut answer = 1;
        loop {
            if !self.step() {
                return answer;
            }
            answer = answer + 1;
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn dxdy(&self, direction: &Direction) -> Point {
        let dxdy= direction.to_dxdy();
        Point {
            x: self.x + dxdy.0,
            y: self.y + dxdy.1,
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        vec!(
            self.dxdy(&Direction::North),
            self.dxdy(&Direction::South),
            self.dxdy(&Direction::West),
            self.dxdy(&Direction::NorthEast),
            self.dxdy(&Direction::NorthWest),
            self.dxdy(&Direction::East),
            self.dxdy(&Direction::South),
            self.dxdy(&Direction::SouthWest),
            self.dxdy(&Direction::SouthEast),
        )
    }
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

impl Direction {
    fn to_dxdy(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
            Direction::NorthEast => (1, -1),
            Direction::NorthWest => (-1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
        }
    }
}