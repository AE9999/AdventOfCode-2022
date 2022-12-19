use std::cmp::max;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_BLUEPRINT: Regex = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let blueprints = read_input(&args[1])?;

    let problem = Problem { blueprints };

    println!("{:?} is what do you get if you add up the quality level of all of the blueprints in your list",
             problem.solve1());


    println!("{:?} is  you get if you multiply these numbers together.",
             problem.solve2());
    Ok(())
}

fn read_input(filename: &String) -> io::Result<Vec<Blueprint>> {
    let file_in = File::open(filename)?;

    let blue_prints =
        BufReader::new(file_in)
            .lines()
            .map(|line|line.unwrap())
            .map(|line| Blueprint::new(line))
            .collect::<Vec<Blueprint>>();
    Ok(blue_prints)
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,
    ore_robot_cost_in_ore: usize,
    clay_robot_cost_in_ore: usize,
    obsidian_robot_cost_in_clay_and_ore: (usize, usize),
    geode_robot_in_ore_and_obsidian: (usize, usize),
}

#[derive(Debug, Clone)]
struct Problem {
    blueprints: Vec<Blueprint>,
}

impl Problem {

    fn solve1(&self) -> usize {
        self.blueprints.iter()
                       .map(|blueprint|  {
                           let mut global_state = GlobalState::new();
                           let initial_state = self.initial_problem_state(24, blueprint);
                           let geos_cracked = self.do_solve(initial_state,
                                                            &mut global_state);
                           geos_cracked * blueprint.id
                       })
                       .sum()
    }

    fn solve2(&self) -> usize {
        self.blueprints.iter()
                       .take(3)
                       .map(|blueprint| {
                            let mut global_state = GlobalState::new();
                            let initial_state = self.initial_problem_state(32, blueprint);
                            self.do_solve(initial_state, &mut global_state)
                        })
                       .fold(1, |prod, val| prod * val)
    }

    fn do_solve(&self,
                problem_state: ProblemState,
                global_state: &mut GlobalState) -> usize {

        if problem_state.ub() <= global_state.lower_bound {
            return global_state.lower_bound;
        }

        let mut possible_outcomes: Vec<usize> = vec!();

        // We decide to do nothing at all
        {
            let solution = problem_state.lb();
            global_state.lower_bound = max(global_state.lower_bound, solution);
            possible_outcomes.push(solution);
        }

        // We decide to construct an ore_bot
        {
            let mut ore_problem_state = problem_state.clone();
            loop {
                if ore_problem_state.can_construct_ore_bot() {
                    ore_problem_state = ore_problem_state.construct_ore_bot();
                    let solution = self.do_solve(ore_problem_state, global_state);
                    global_state.lower_bound = max(global_state.lower_bound, solution);
                    possible_outcomes.push(solution);
                    break;
                }
                ore_problem_state = ore_problem_state.simulate_step();
                if ore_problem_state.time_left <= 0 {
                    // Could not construct stuff same as doing nothing
                    break;
                }
            }
        }

        // We decide to construct a clay bot
        {
            let mut clay_problem_state = problem_state.clone();
            loop {
                if clay_problem_state.can_construct_clay_bot() {
                    clay_problem_state = clay_problem_state.construct_clay_bot();
                    let solution = self.do_solve(clay_problem_state, global_state);
                    global_state.lower_bound = max(global_state.lower_bound, solution);
                    possible_outcomes.push(solution);
                    break;
                }
                clay_problem_state = clay_problem_state.simulate_step();
                if clay_problem_state.time_left <= 0 {
                    // Could not construct stuff same as doing nothing
                    break;
                }
            }
        }

        // We decide to construct an obsidian bot, don't attempt if we don't even have clay bots
        if problem_state.has_clay_bots() {
            let mut obsidian_problem_state = problem_state.clone();
            loop {
                if obsidian_problem_state.can_construct_obsidian_bot() {
                    obsidian_problem_state = obsidian_problem_state.construct_obsidian_bot();
                    let solution = self.do_solve(obsidian_problem_state, global_state);
                    global_state.lower_bound = max(global_state.lower_bound, solution);
                    possible_outcomes.push(solution);
                    break;
                }
                obsidian_problem_state = obsidian_problem_state.simulate_step();
                if obsidian_problem_state.time_left <= 0 {
                    // Could not construct stuff same as doing nothing
                    break;
                }
            }
        }

        // We decide to construct a geode bot don't attempt if we don't even have obsidian bots
        if problem_state.has_obsidian_bots() {
            let mut geode_problem_state = problem_state.clone();
            loop {
                if geode_problem_state.can_construct_geode_bot() {
                    geode_problem_state = geode_problem_state.construct_geode_bot();
                    let solution = self.do_solve(geode_problem_state, global_state);
                    global_state.lower_bound = max(global_state.lower_bound, solution);
                    possible_outcomes.push(solution);
                    break;
                }
                geode_problem_state = geode_problem_state.simulate_step();
                if geode_problem_state.time_left <= 0 {
                    // Could not construct stuff same as doing nothing
                    break;
                }
            }
        }

        *possible_outcomes.iter().max().unwrap()
    }

    fn initial_problem_state(&self, time_left: i32, blueprint: &Blueprint) -> ProblemState {
        ProblemState {
            time_left,
            amount_of_ore_bots: 1,
            amount_of_clay_bots: 0,
            amount_of_obsidian_bots: 0,
            amount_of_geode_bots: 0,
            amount_of_ore: 0,
            amount_of_clay: 0,
            amount_of_obsidian: 0,
            amount_of_geode: 0,
            blueprint: blueprint.clone(),
        }
        
    }
}

#[derive(Debug, Clone)]
struct GlobalState {
    lower_bound : usize,
}

impl GlobalState {
    fn new() -> Self {
        GlobalState {
            lower_bound: 0
        }
    }
}

#[derive(Debug, Clone)]
struct ProblemState {
    time_left: i32,
    amount_of_ore_bots: usize,
    amount_of_clay_bots: usize,
    amount_of_obsidian_bots: usize,
    amount_of_geode_bots: usize,
    amount_of_ore: usize,
    amount_of_clay: usize,
    amount_of_obsidian: usize,
    amount_of_geode: usize,
    blueprint: Blueprint
}

impl ProblemState {

    fn ub(&self) -> usize { // Double check
        let mut rvalue = self.amount_of_geode;
        rvalue = rvalue + self.amount_of_geode_bots * (self.time_left as usize);  // the amount of geode we would still mine with the bots we currently have
        let remaining = (((self.time_left -1) * (self.time_left)) / 2) as usize;
        rvalue = rvalue + remaining;
        rvalue
    }

    fn lb(&self) -> usize {
        self.amount_of_geode + (self.amount_of_geode_bots * (self.time_left as usize))
    }

    fn can_construct_ore_bot(&self) -> bool {
        self.time_left >= 1 && self.amount_of_ore >= self.blueprint.ore_robot_cost_in_ore
    }

    fn construct_ore_bot(&mut self) -> ProblemState {
        assert!(self.can_construct_ore_bot());
        let mut me = self.clone();
        me.amount_of_ore = me.amount_of_ore - me.blueprint.ore_robot_cost_in_ore;
        let mut me = me.simulate_step();
        me.amount_of_ore_bots = me.amount_of_ore_bots + 1;
        me
    }

    fn has_clay_bots(&self) -> bool {
        self.amount_of_clay_bots > 0
    }

    fn can_construct_clay_bot(&self) -> bool {
        return self.time_left >= 1
               && self.amount_of_ore >= self.blueprint.clay_robot_cost_in_ore;
    }

    fn construct_clay_bot(&self) -> ProblemState {
        assert!(self.can_construct_clay_bot());
        let mut me = self.clone();
        me.amount_of_ore = me.amount_of_ore - me.blueprint.clay_robot_cost_in_ore;
        let mut me = me.simulate_step();
        me.amount_of_clay_bots = me.amount_of_clay_bots + 1;
        me
    }

    fn has_obsidian_bots(&self) -> bool {
        self.amount_of_obsidian_bots > 0
    }

    fn can_construct_obsidian_bot(&self) -> bool {
        self.time_left > 0
        && self.amount_of_clay >= self.blueprint.obsidian_robot_cost_in_clay_and_ore.0
        && self.amount_of_ore >= self.blueprint.obsidian_robot_cost_in_clay_and_ore.1
    }

    fn construct_obsidian_bot(&self) -> ProblemState {
        assert!(self.can_construct_obsidian_bot());
        let mut me = self.clone();
        me.amount_of_clay = me.amount_of_clay - me.blueprint.obsidian_robot_cost_in_clay_and_ore.0;
        me.amount_of_ore = me.amount_of_ore - me.blueprint.obsidian_robot_cost_in_clay_and_ore.1;
        let mut me = me.simulate_step();
        me.amount_of_obsidian_bots = me.amount_of_obsidian_bots + 1;
        me
    }

    fn can_construct_geode_bot(&self) -> bool {
        self.time_left > 0
            && self.amount_of_ore >= self.blueprint.geode_robot_in_ore_and_obsidian.0
            && self.amount_of_obsidian >= self.blueprint.geode_robot_in_ore_and_obsidian.1
    }

    fn construct_geode_bot(&self) -> ProblemState {
        assert!(self.can_construct_geode_bot());
        let mut me = self.clone();
        me.amount_of_ore = me.amount_of_ore - me.blueprint.geode_robot_in_ore_and_obsidian.0;
        me.amount_of_obsidian = me.amount_of_obsidian - me.blueprint.geode_robot_in_ore_and_obsidian.1;
        let mut me = me.simulate_step();
        me.amount_of_geode_bots = me.amount_of_geode_bots + 1;
        me
    }

    fn simulate_step(&self) -> ProblemState {
        let mut next_problem_state = self.clone();
        next_problem_state.time_left =
            next_problem_state.time_left - 1;
        next_problem_state.amount_of_ore  =
            next_problem_state.amount_of_ore + next_problem_state.amount_of_ore_bots;
        next_problem_state.amount_of_clay =
            next_problem_state.amount_of_clay + next_problem_state.amount_of_clay_bots;
        next_problem_state.amount_of_obsidian =
            next_problem_state.amount_of_obsidian + next_problem_state.amount_of_obsidian_bots;
        next_problem_state.amount_of_geode=
            next_problem_state.amount_of_geode + next_problem_state.amount_of_geode_bots;

        next_problem_state
    }
}

impl Blueprint {
    fn new(line: String) -> Self {
        let mut cap = RE_BLUEPRINT.captures_iter(line.as_str().trim());
        let cap  = cap.next().unwrap();

        Blueprint {
            id: cap[1].parse::<usize>().unwrap(),
            ore_robot_cost_in_ore: cap[2].parse::<usize>().unwrap(),
            clay_robot_cost_in_ore: cap[3].parse::<usize>().unwrap(),
            obsidian_robot_cost_in_clay_and_ore: (cap[5].parse::<usize>().unwrap(),
                                                  cap[4].parse::<usize>().unwrap()),
            geode_robot_in_ore_and_obsidian:  (cap[6].parse::<usize>().unwrap(),
                                               cap[7].parse::<usize>().unwrap())
        }
    }
}

// Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 15 clay. Each geode robot costs 2 ore and 8 obsidian.
