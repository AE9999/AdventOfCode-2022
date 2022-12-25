extern crate core;

use std::cmp::min;
use std::collections::{BTreeSet, HashMap};  // If we want to hash states we need order stuff
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let initial_state = read_input(&args[1])?;

    let snowstorm_phase = calculate_snowstorm_phase(initial_state.clone());

    println!("The snowstorm phase for our problems is {:?} ", snowstorm_phase);

    println!("{:?} is the fewest number of minutes required to avoid the blizzards and reach the goal",
             solve1(initial_state.clone(),
                &mut GlobalStats::new(snowstorm_phase)));

    println!("{:?} is the fewest number of minutes required to reach the goal, go back to the start, then reach the goal again",
            solve2(initial_state.clone(),
                   &mut GlobalStats::new(snowstorm_phase)));

    Ok(())
}

fn solve1(state: State,
          global_stats: &mut GlobalStats) -> usize {

    if state.moves_made + state.distance_from_end_state() >= global_stats.global_lb {
        return usize::MAX;
    }

    if global_stats.state_2_known_distance.contains_key(&global_stats.state_to_hash(&state)) {
        let offset = *global_stats.state_2_known_distance.get(&global_stats.state_to_hash(&state)).unwrap();
        return if offset == usize::MAX {
                    offset
                }
                else {
                    offset + state.moves_made
                }
    }

    let lb_for_point =
        global_stats.lb_for_points.get(&state.my_current_location).unwrap_or(&usize::MAX);

    if state.moves_made + state.distance_from_end_state() >= *lb_for_point {
        return usize::MAX
    }

    let known_lb = *global_stats.state_2_lb.get(&global_stats.state_to_hash(&state)).unwrap_or(&usize::MAX);
    if known_lb < state.moves_made  {
        // println!("\tA better solution is already being calculated {:?} ..", state.my_current_location);
        return usize::MAX
    }

    global_stats.state_2_lb.insert(global_stats.state_to_hash(&state), state.moves_made);

    let mut children: Vec<State> = vec!();

    let next_blizzard = state.calculate_next_blizzards();

    for possible_move in state.possible_moves() {
        if next_blizzard.iter().any(|blizzard| blizzard.point == possible_move) {
            continue
        }

        let next_state = state.next_step(possible_move);

        let known_lb = *global_stats.state_2_lb.get(&global_stats.state_to_hash(&next_state)).unwrap_or(&usize::MAX);
        if known_lb < state.moves_made   {
            continue
        } else {
            global_stats.state_2_lb.insert(global_stats.state_to_hash(&next_state), next_state.moves_made);
        }

        if next_state.in_end_state() {
            let answer = next_state.moves_made;

            // println!("\t\t Solution {:?} found! :-) ..", answer);

            assert!(global_stats.global_lb > answer);
            global_stats.global_lb = answer;

            global_stats.lb_for_points.insert(state.my_current_location.clone(), answer);

            return answer;
        }

        children.push(next_state)
    }

    if children.is_empty() {
        // global_stats.state_2_lb.insert(global_stats.state_to_hash(&state), 0);
        // println!("\t No children to evaluate :-) ..");
        global_stats.state_2_known_distance.insert(global_stats.state_to_hash(&state), usize::MAX);
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
                                   global_stats);

                        answer

                    })
                    .min()
                    .unwrap();

        let lb_for_point =
            *global_stats.lb_for_points.get(&state.my_current_location).unwrap_or(&usize::MAX);

        let lb_for_point = min(min_distance, lb_for_point);

        global_stats.lb_for_points.insert(state.my_current_location.clone(), lb_for_point);

        global_stats.state_2_known_distance.insert(global_stats.state_to_hash(&state), min_distance - state.moves_made);

        min_distance
    }
}

// 1440 is too high
// 723 is too high
fn solve2(heen_state: State,
          global_stats: &mut GlobalStats) -> usize {

    let amount_heen =
        solve1(heen_state.clone(),
          &mut global_stats.clone());

    println!("Amount heen was {:?} ..", amount_heen);

    let mut terug_state = heen_state.reverse_start_and_end();
    for _ in 0..amount_heen {
        terug_state = terug_state.let_blizzard_blow();
    }

    let amount_terug =
        solve1(terug_state.clone(),
               &mut global_stats.clone());
    println!("Amount terug was {:?} ..", amount_terug);


    let mut en_weer_heen_state = heen_state.clone();
    for _ in 0..(amount_heen + amount_terug) {
        en_weer_heen_state = en_weer_heen_state.let_blizzard_blow();
    }
    let amount_en_weer_heen =
        solve1(en_weer_heen_state.clone(),
               &mut global_stats.clone());

    println!("Amount en weer terug was {:?} ..",  amount_en_weer_heen);

    amount_heen + amount_terug + amount_en_weer_heen
}

fn calculate_snowstorm_phase(mut state: State) -> usize {
    let initial_state = state.blizzards.clone();
    let mut iterations: usize = 0;
    loop {
        iterations = iterations + 1;
        state = state.next_step(state.my_current_location.clone());
        if state.blizzards == initial_state {
            return iterations
        }
        assert!(iterations < 100000)
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
        moves_made: 0,
        blown_blizzards: 0,
        walls,
        blizzards,
        my_current_location: start.as_ref().unwrap().clone(),
        end: end.unwrap(),
        start: start.unwrap(),
    };

    Ok(problem_state)
}

#[derive(Debug, Clone)]
struct GlobalStats {
    state_2_lb: HashMap<(Point, usize), usize>,
    state_2_known_distance: HashMap<(Point, usize), usize>,
    lb_for_points: HashMap<Point, usize>,
    global_lb: usize,
    snowstorm_phase: usize,
}

impl GlobalStats {
    fn new(snowstorm_phase: usize) -> Self {
        GlobalStats {
            state_2_lb: HashMap::new(),
            state_2_known_distance: HashMap::new(),
            lb_for_points: HashMap::new(),
            global_lb: usize::MAX,
            snowstorm_phase,
        }
    }

    fn state_to_hash(&self,
                     state: &State) -> (Point, usize) {
        let effective_phase = state.blown_blizzards % self.snowstorm_phase;
        return (state.my_current_location.clone(), effective_phase)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    width: i32,
    height: i32,
    moves_made: usize,
    walls: BTreeSet<Point>,
    blizzards: BTreeSet<Blizzard>,
    blown_blizzards: usize,
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

    fn next_step(&self, next_location: Point) -> Self  {
        let mut next_state = self.clone();
        next_state.my_current_location = next_location;
        next_state.blizzards = self.calculate_next_blizzards();
        next_state.blown_blizzards = self.blown_blizzards + 1;
        next_state.moves_made = self.moves_made + 1;
        next_state
    }

    fn let_blizzard_blow(&self) -> Self  {
        let mut next_state = self.clone();
        next_state.blizzards = self.calculate_next_blizzards();
        next_state.blown_blizzards = self.blown_blizzards + 1;
        next_state
    }

    fn reverse_start_and_end(&self) -> Self {
        assert_eq!(self.my_current_location, self.start);
        let old_start = self.start.clone();
        let old_end = self.end.clone();

        let mut next_state = self.clone();
        next_state.my_current_location = old_end.clone();
        next_state.start = old_end.clone();
        next_state.end = old_start;
        next_state
    }

    // fn get_eq(&mut self) {
    //
    // }
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


