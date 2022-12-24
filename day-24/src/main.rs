use std::collections::{BTreeSet, HashMap, HashSet};  // If we want to hash states we need order stuff
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let initial_state = read_input(&args[1])?;
    let mut state_2_lb: HashMap<State, usize> = HashMap::new();
    let mut terminating_states: HashSet<State> =  HashSet::new();

    println!("{:?} is the fewest number of minutes required to avoid the blizzards and reach the goal",
             solve1(initial_state.clone(),
                               0,
                               &mut terminating_states,
                               &mut state_2_lb));

    Ok(())
}

fn solve1(state: State,
          moves_made: usize,
          terminating_states: &mut HashSet<State>,
          state_2_lb: &mut HashMap<State, usize>) -> usize {

    // println!("Considering state: {:?}, moves_made:{:?} ..", state.my_current_location, moves_made);

    if terminating_states.contains(&state) {
        // println!("\tIs terminated before ..");
        return usize::MAX
    }

    let known_lb = *state_2_lb.get(&state).unwrap_or(&usize::MAX);

    if known_lb < moves_made {
        // println!("\tBetter Lb found ..");
        return usize::MAX
    }

    state_2_lb.insert(state.clone(), moves_made);

    let mut children: Vec<State> = vec!();

    let next_blizzard = state.calculate_next_blizzards();

    for possible_move in state.possible_moves() {

        // println!("\tConsidering move {:?} ..", possible_move);
        if next_blizzard.iter().any(|blizzard| blizzard.point == possible_move) {
            // println!("\t\tIs in blizzard ..");
            continue
        }

        let next_state = state.next(possible_move);

        if terminating_states.contains(&next_state) {
            // println!("\t\tState is known to be terminating");
            continue
        }

        let known_lb = *state_2_lb.get(&next_state).unwrap_or(&usize::MAX);
        if known_lb <= moves_made + 1 {
            // println!("\t\tBetter LB known!");
            continue
        } else {
            state_2_lb.insert(next_state.clone(), moves_made + 1);
        }

        if next_state.in_end_state() {
            // println!("\t\tFound a sollution!");
            return moves_made + 1;
        }

        children.push(next_state)
    }

    if children.is_empty() {
        // println!("\tNo path forward!");
        terminating_states.insert(state.clone());
        return usize::MAX
    } else {
        // println!("\tEvaluating Children!");
        children.into_iter()
                .map(|child| {
                    solve1(child, moves_made + 1, terminating_states, state_2_lb)
                })
                .min()
                .unwrap()
    }
}

fn read_input(filename: &String) -> io::Result<State> {
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

    let problem_state = State {
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
struct State {
    width: i32,
    height: i32,
    walls: BTreeSet<Point>,
    blizzards: BTreeSet<Blizzard>,
    my_current_location: Point,
    end: Point,
    start: Point,
}

impl State {

    fn possible_moves(&self) -> Vec<Point> {
        self.my_current_location
            .neighbours_including_self()
            .into_iter()
            .filter(|point|{
                point.x >= 0
                && point.x < self.width
                && point.y >= 0
                && point.y < self.height
                && !self.walls.contains(point)
            })
            .collect::<Vec<Point>>()
    }

    fn in_end_state(&self) -> bool {
        return self.my_current_location == self.end;
    }

    fn calculate_next_blizzards(&self) -> BTreeSet<Blizzard>  {
        let mut next_blizzards : BTreeSet<Blizzard> = BTreeSet::new();

        assert!(!self.walls.contains(&self.my_current_location));
        assert!(self.my_current_location.x >= 0 && self.my_current_location.x < self.width);
        assert!(self.my_current_location.y >= 0 && self.my_current_location.y < self.height);

        for blizzard in self.blizzards.iter() {
            let next_point = blizzard.point.point_in_direction(&blizzard.direction);
            assert_ne!(next_point, self.start);

            let next_blizzard: Blizzard =
                if self.walls.contains(&next_point) {
                    let point =
                        match &blizzard.direction {
                            Direction::Up => Point {
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
                        };
                    Blizzard {
                        direction: blizzard.direction.clone(),
                        point,
                    }
                } else {
                    Blizzard {
                        direction: blizzard.direction.clone(),
                        point: next_point
                    }
                }
            ;
            // println!("{:?} -> {:?} ..", blizzard, next_blizzard);
            next_blizzards.insert(next_blizzard);
        };

        assert_eq!(self.blizzards.len(), next_blizzards.len());

        next_blizzards
    }

    fn next(&self, next_location: Point) -> Self  {
        let mut next_state = self.clone();
        next_state.my_current_location = next_location;
        next_state.blizzards = self.calculate_next_blizzards();
        next_state
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

    fn neighbours_including_self(&self) -> Vec<Point> {
        vec!(
            self.clone(),
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


