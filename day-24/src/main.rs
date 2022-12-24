use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet};  // If we want to hash states we need order stuff
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let initial_state = read_input(&args[1])?;
    let mut state_2_lb: HashMap<State, usize> = HashMap::new();
    let mut state_2_solved_distance: HashMap<State, usize> = HashMap::new();
    let mut lb_for_point: HashMap<Point, usize> = HashMap::new();

    println!("{:?} is the fewest number of minutes required to avoid the blizzards and reach the goal",
             solve1(initial_state.clone(),
                               0,
                               &mut state_2_solved_distance,
                               &mut state_2_lb,
                    &mut lb_for_point));

    Ok(())
}

fn solve1(state: State,
          moves_made: usize,
          state_2_solved_distance: &mut HashMap<State, usize>,
          state_2_lb: &mut HashMap<State, usize>,
          lb_for_points: &mut HashMap<Point, usize>) -> usize {

    println!("Considering {:?}, moves_made {:?} ..", state.my_current_location, moves_made);

    if state_2_solved_distance.contains_key(&state) {
        // println!("\tAlready solved {:?} ..", state.my_current_location);
        return  *state_2_solved_distance.get(&state).unwrap()
    }

    let lb_for_point =
        *lb_for_points.get(&state.my_current_location).unwrap_or(&usize::MAX);

    if moves_made + state.distance_from_end_state() >= lb_for_point {
        return usize::MAX
    }

    let known_lb = *state_2_lb.get(&state).unwrap_or(&usize::MAX);
    if known_lb < moves_made  {
        // println!("\tA better solution is already being calculated {:?} ..", state.my_current_location);
        return usize::MAX
    }
    state_2_lb.insert(state.clone(), moves_made);

    let mut children: Vec<State> = vec!();

    let next_blizzard = state.calculate_next_blizzards();

    for possible_move in state.possible_moves() {
        // println!("\t Considering move {:?} ..", possible_move);
        if next_blizzard.iter().any(|blizzard| blizzard.point == possible_move) {
            // println!("\t\t In blizzard :-( ..");
            continue
        }

        let next_state = state.next(possible_move);

        let known_lb = *state_2_lb.get(&next_state).unwrap_or(&usize::MAX);
        if known_lb < moves_made   {
            // println!("\t\t We have a quicker way to get there :-) ..");
            continue
        } else {
            state_2_lb.insert(next_state.clone(), moves_made + 1);
        }

        if next_state.in_end_state() {
            // println!("\t\t Solution found! :-) ..");

            let answer = moves_made + 1;

            state_2_solved_distance.insert( state.clone(), answer);

            let lb_for_point =
                *lb_for_points.get(&state.my_current_location).unwrap_or(&usize::MAX);

            let lb_for_point = min(answer, lb_for_point);

            lb_for_points.insert(state.my_current_location.clone(), lb_for_point);

            return answer;
        }

        children.push(next_state)
    }

    if children.is_empty() {
        // state_2_lb.insert(state.clone(), 0);
        // println!("\t No children to evaluate :-) ..");
        state_2_solved_distance.insert(state.clone(), usize::MAX);
        return usize::MAX
    } else {

        // Add a heuristic. We want to go towards the endpoint
        children.sort_by( |a, b| {
            a.distance_from_end_state().cmp(&b.distance_from_end_state())
        }) ;


        let min_distance =
            children.into_iter()
                    .map(|child| {
                        let answer =
                            solve1(child.clone(),
                                   moves_made + 1,
                                   state_2_solved_distance,
                                   state_2_lb,
                                   lb_for_points);

                        let lb_for_point =
                            *lb_for_points.get(&child.my_current_location).unwrap_or(&usize::MAX);

                        let lb_for_point = min(answer, lb_for_point);

                        lb_for_points.insert(child.my_current_location.clone(), lb_for_point);

                        state_2_solved_distance.insert(child, answer);

                        answer

                    })
                    .min()
                    .unwrap();

        let lb_for_point =
            *lb_for_points.get(&state.my_current_location).unwrap_or(&usize::MAX);

        let lb_for_point = min(min_distance, lb_for_point);

        lb_for_points.insert(state.my_current_location.clone(), lb_for_point);

        state_2_solved_distance.insert(state.clone(), min_distance);

        min_distance
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

    fn distance_from_end_state(&self) -> usize {
        self.my_current_location.distance(&self.end) as usize
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

    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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


