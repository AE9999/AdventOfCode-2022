use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()>  {
    let args: Vec<String> = env::args().collect();
    let valves = read_input(&args[1])?;

    println!("Valves: {:?} ..", valves);

    let problem_state = ProblemState::new(valves.clone());
    let mut problem_state_2_known_solutions: HashMap<ProblemState, usize> = HashMap::new();
    println!("{:?} is the most pressure you can release",
            solve1(problem_state, &mut problem_state_2_known_solutions));

    Ok(())
}

fn read_input(filename: &String) -> io::Result<Vec<Valve>> {
    let file_in = File::open(filename)?;

    let valves =
        BufReader::new(file_in)
            .lines()
            .map(|line| line.unwrap())
            .map(|line| Valve::new(line))
            .collect::<Vec<Valve>>();

    Ok(valves)
}

fn solve1(problem_state: ProblemState,
          problem_state_2_known_solutions: &mut HashMap<ProblemState, usize>) -> usize {

    println!("Problem State: {:?} ..", problem_state.time_left);

    if problem_state.time_left == 0 {
        let mut rvalue: usize = 0;
        for valve in problem_state.valves {
            rvalue += valve.minutes_left_after_opened * valve.flow_rate ;
        }
        return rvalue
    }

    if problem_state_2_known_solutions.get(&problem_state).is_some() {
        return *problem_state_2_known_solutions.get(&problem_state).unwrap()
    }

    let current_index = problem_state.valve_index_by_name(problem_state.current_position.as_str());

    let what_would_happen_if_we_switch_this_room =
        if  problem_state.valves[current_index]
                         .minutes_left_after_opened == 0 {

            let mut next_problem_state = problem_state.clone();
            next_problem_state.time_left = problem_state.time_left -1;
            next_problem_state.valves[current_index].minutes_left_after_opened = problem_state.time_left;
            Some(solve1(next_problem_state, problem_state_2_known_solutions))
        } else {
            None
        }
    ;

    let best_option_if_we_move =
        problem_state.valves[current_index].outgoing_tunnels.iter().map(|outgoing_tunnel_id| {
            let mut next_problem_state = problem_state.clone();
            next_problem_state.time_left = next_problem_state.time_left -1;
            next_problem_state.current_position = outgoing_tunnel_id.clone();
            solve1(next_problem_state, problem_state_2_known_solutions)
        }).max().unwrap();

    let best_option =
        if what_would_happen_if_we_switch_this_room.is_some() {
            let what_would_happen_if_we_switch_this_room = what_would_happen_if_we_switch_this_room.unwrap();
            return max(what_would_happen_if_we_switch_this_room, best_option_if_we_move)
        } else {
            best_option_if_we_move
        };

    problem_state_2_known_solutions.insert(problem_state, best_option);

    best_option
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProblemState {
    current_position: String,
    valves: Vec<Valve>,
    time_left: usize
}

impl ProblemState {
    fn new(valves: Vec<Valve>) -> Self {

        let current_position = String::from("AA");
        let time_left: usize = 30;

        ProblemState {
            valves,
            current_position,
            time_left,
        }
    }

    fn valve_index_by_name(&self, name: &str) -> usize {
        for i in 0..self.valves.len() {
            if self.valves[i].name == name {
                return i
            }
        }

        panic!("Expected something to match")
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Valve {
    name: String,
    flow_rate: usize,
    outgoing_tunnels: Vec<String>,
    minutes_left_after_opened: usize,
}

impl Valve {
    fn new(line: String) -> Self {
        // Valve QJ has flow rate=11; tunnels lead to valves HB, GL
        let name = line.chars().skip("Valve ".len()).take(2).collect::<String>();
        let mut split = line.split(';');
        let first_part = split.next().unwrap().to_string();
        let flow_rate = first_part.chars()
                                  .skip("Valve QJ has flow rate=".len())
                                  .collect::<String>()
                                  .parse::<usize>().unwrap();

        let second_part = split.next().unwrap().to_string();
        let outgoing_tunnels =
            if second_part.contains(" tunnels lead to valves ") {
                second_part.chars()
                           .skip(" tunnels lead to valves ".len())
                           .collect::<String>()
                           .split(',')
                           .map(|str|String::from(str.trim()))
                           .collect::<Vec<String>>()
            } else {
                second_part.chars()
                    .skip(" tunnels lead to valve ".len())
                    .collect::<String>()
                    .split(',')
                    .map(|str|String::from(str.trim()))
                    .collect::<Vec<String>>()
            }
        ;

        Valve {
            name,
            flow_rate,
            outgoing_tunnels,
            minutes_left_after_opened: 0
        }
    }
}



