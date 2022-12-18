use std::cmp::{max};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()>  {
    let args: Vec<String> = env::args().collect();
    let valves = read_input(&args[1])?;

    let mut problem = Problem::new(valves.clone());

    println!("{:?} is the most pressure you can release.",
             problem.solve());

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


fn valve_index_by_name(valves: &Vec<Valve>, name: &str) -> usize {
    for i in 0..valves.len() {
        if valves[i].name == name {
            return i
        }
    }
    panic!("Expected something to match")
}

#[derive(Debug, Clone)]
struct Problem {
    distance_between_valves: HashMap<(String, String), i32>,
    current_lower_bound: i32,
    valves: Vec<Valve>,
    starting_position: String,
    starting_time_left: i32,
}

impl Problem {

    fn do_solve(&mut self, search_state: SearchState) -> i32 {

        let current_value = (0..search_state.valve_open.len()).into_iter()
            .map(|i|{
                let valve = self.valves.get(i).unwrap();
                return valve.flow_rate * search_state.valve_open[i]
            }).fold(0, |sum, val| sum + val);

        let possible_value_me = (0..search_state.valve_open.len()).into_iter()
            .filter(|&i|{
                search_state.valve_open[i] == 0
            })
            .map(|i|{
                let valve = self.valves.get(i).unwrap();
                let key = (search_state.current_position_me.clone(), valve.name.clone());
                let time_needed = *(self.distance_between_valves.get(&key).unwrap()) + 1;
                return if  search_state.time_left_me >= time_needed   {
                    valve.flow_rate * (search_state.time_left_me - time_needed)
                } else {
                    0
                }
            }).fold(0, |sum, val| sum + val);

        let possible_value_elephant = (0..search_state.valve_open.len()).into_iter()
            .filter(|&i|{
                search_state.valve_open[i] == 0
            })
            .map(|i|{
                let valve = self.valves.get(i).unwrap();
                let key = (search_state.current_position_elephant.clone(), valve.name.clone());
                let time_needed = *(self.distance_between_valves.get(&key).unwrap()) + 1;
                return if  search_state.time_left_elephant >= time_needed   {
                    valve.flow_rate * (search_state.time_left_elephant - time_needed)
                } else {
                    0
                }
            }).fold(0, |sum, val| sum + val);

        if current_value + possible_value_me + possible_value_elephant <= self.current_lower_bound {
            return self.current_lower_bound;
        }

        let mut results: Vec<i32> = Vec::new();
        for i in 0..search_state.valve_open.len() {
            let valve = self.valves.get(i).unwrap().clone();

            if (search_state.valve_open[i] == 0)
                && valve.flow_rate > 0 { // Yeah it really is that simple {

                // I go
                let key = (search_state.current_position_me.clone(), valve.name.clone());
                let time_needed = *(self.distance_between_valves.get(&key).unwrap()) + 1;
                let result_me =
                    if search_state.time_left_me - time_needed > 0 {
                        let mut next_search_state = search_state.clone();
                        next_search_state.current_position_me = valve.name.clone();
                        next_search_state.time_left_me = search_state.time_left_me - time_needed;
                        next_search_state.valve_open[i] = search_state.time_left_me - time_needed;
                        Some(self.do_solve(next_search_state))
                    } else {
                        None
                    }
                ;

                // The elephant goes
                let key = (search_state.current_position_elephant.clone(), valve.name.clone());
                let time_needed = *(self.distance_between_valves.get(&key).unwrap()) + 1;
                let result_elephant =
                    if search_state.time_left_elephant - time_needed > 0 {
                        let mut next_search_state = search_state.clone();
                        next_search_state.current_position_elephant = valve.name.clone();
                        next_search_state.time_left_elephant = search_state.time_left_elephant - time_needed;
                        next_search_state.valve_open[i] = search_state.time_left_elephant - time_needed;
                        Some(self.do_solve(next_search_state))
                    } else {
                        None
                    }
                ;

                if result_me.is_none() && result_elephant.is_none() {
                    continue; // do nothing
                } else if result_me.is_some() && result_elephant.is_none() {
                    let result = result_me.unwrap();
                    self.current_lower_bound = max(self.current_lower_bound, result);
                    results.push(result);
                } else if result_me.is_none() && result_elephant.is_some() {
                    let result = result_elephant.unwrap();
                    self.current_lower_bound = max(self.current_lower_bound, result);
                    results.push(result);
                } else {
                    assert!(result_me.is_some() && result_elephant.is_some());
                    let result = max(result_me.unwrap(), result_elephant.unwrap());
                    self.current_lower_bound = max(self.current_lower_bound, result);
                    results.push(result);
                }
            }
        }

        let result = if !results.is_empty() {
            *(results.iter().max().unwrap())
        } else {
            (0..search_state.valve_open.len()).into_iter()
                                          .map(|i|{
                                              let valve = self.valves.get(i).unwrap();
                                              return valve.flow_rate * search_state.valve_open[i]
                                          }).fold(0, |sum, val| sum + val)
        };
        self.current_lower_bound = max(self.current_lower_bound, result);

        result
    }

    fn solve(&mut self) -> i32 {
        let search_state = SearchState {
            current_position_me: self.starting_position.clone(),
            time_left_me: self.starting_time_left,
            current_position_elephant: self.starting_position.clone(),
            time_left_elephant: self.starting_time_left,
            valve_open: vec![0 ; self.valves.len()],
        };
        self.do_solve(search_state)
    }
}

fn find_shortest_paths_for_valve_to_other_valves(valves: &Vec<Valve>,
                                                 start_valve: usize) -> HashMap<String, i32> {
    let mut queue: Vec<(String, i32)> = Vec::new();
    let mut rvalue: HashMap<String, i32> = HashMap::new();

    queue.push((valves[start_valve].name.clone(), 0));

    while !queue.is_empty() {
        let top = queue.remove(0);
        rvalue.insert(top.0.clone(), top.1);

        let valve =
            valves.get(valve_index_by_name(valves, top.0.as_str())).unwrap();

        for neighbour in valve.outgoing_tunnels.iter() {
            if !rvalue.contains_key(neighbour) {
                queue.push((neighbour.clone(), top.1 + 1));
            }
        }
    }

    rvalue
}

impl Problem {
    fn new(valves: Vec<Valve>) -> Self {

        let mut distance_between_valves: HashMap<(String, String), i32> = HashMap::new();

        for i in 0..valves.len() {
            let valve = valves.get(i).unwrap();

            let shortest_paths =
                find_shortest_paths_for_valve_to_other_valves(&valves, i);
            for shortest_path in shortest_paths.iter() {
                let left_name = valve.name.clone();
                let right_name = shortest_path.0.clone();
                let lenght = *shortest_path.1;
                let key = (left_name, right_name);
                distance_between_valves.insert(key, lenght);
            }
        }

        Problem {
            distance_between_valves,
            current_lower_bound: 0,
            valves,
            starting_position: String::from("AA"),
            starting_time_left: 26,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchState {
    current_position_me: String,
    time_left_me: i32,
    current_position_elephant: String,
    time_left_elephant: i32,
    valve_open: Vec<i32>,
}


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Valve {
    name: String,
    flow_rate: i32,
    outgoing_tunnels: Vec<String>,
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
                                  .parse::<i32>().unwrap();

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
        }
    }
}



