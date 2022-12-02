use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use crate::Outcome::{Draw, Loss, Win};
use crate::PlayerInput::{Paper, Rock, Scissors};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    println!("{:?} is your total score be if everything goes exactly according to your strategy guide",
             read_input(input)?.iter().map(|x| x.score()).sum::<u64>());


    println!("{:?} is your total score be if everything goes exactly according to your strategy guide",
             read_input_part2(input)?.iter().map(|x| x.score()).sum::<u64>());

    Ok(())
}

fn read_input(filename: &String) ->  io::Result<Vec<RoundInput>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();
    let round_inputs: Vec<RoundInput> = file_reader.map(|x| x.unwrap() ).map(|x| RoundInput::new(x)).collect();
    Ok(round_inputs)
}

fn read_input_part2(filename: &String) ->  io::Result<Vec<Part2RoundInput>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();
    let part2_round_inputs: Vec<Part2RoundInput> = file_reader.map(|x| x.unwrap() ).map(|x| Part2RoundInput::new(x)).collect();
    Ok(part2_round_inputs)
}

struct RoundInput {
    opponent_input: PlayerInput,
    my_input: PlayerInput,
}

impl RoundInput {

    fn new(line: String) -> Self {
        let mut line = line.split(" ");
        RoundInput {
            opponent_input: PlayerInput::from_left_input(line.next().unwrap()),
            my_input: PlayerInput::from_right_input(line.next().unwrap()),
        }
    }

    fn score(&self) -> u64 {
        self.my_input.to_score() + self.my_input.beats(&self.opponent_input).to_score()
    }
}

struct Part2RoundInput {
    opponent_input: PlayerInput,
    desired_outcome: Outcome,
}

impl Part2RoundInput {
    fn new(line: String) -> Self {
        let mut line = line.split(" ");
        Part2RoundInput {
            opponent_input: PlayerInput::from_left_input(line.next().unwrap()),
            desired_outcome: Outcome::from_input(line.next().unwrap()),
        }
    }

    fn score(&self) -> u64 {
        let my_input = self.opponent_input.find_my_piece_depending_on_opponent_piece_and_desired_outcome(&self.desired_outcome);
        my_input.to_score() + my_input.beats(&self.opponent_input).to_score()
    }
}

#[derive(Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn to_score(&self) -> u64 {
        match &self {
            Loss => 0,
            Draw => 3,
            Win => 6
        }
    }

    fn from_input(input: &str) -> Self {
        match input {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Unexpected Input")
        }
    }
}

#[derive(Debug)]
enum PlayerInput {
    Rock,
    Paper,
    Scissors
}

impl PlayerInput {
    fn from_left_input(input: &str) -> Self {
        match input {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!("Unexpected Input")
        }
    }

    fn from_right_input(input: &str) -> Self {
        match input {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!("Unexpected Input")
        }
    }

    fn beats(&self, other: &PlayerInput) -> Outcome {
        match *self {
            Rock => match *other {
                Rock => Draw,
                Paper =>Loss,
                Scissors => Win
            },
            Paper => match *other {
                Rock => Win,
                Paper => Draw,
                Scissors => Loss
            },
            Scissors => match *other {
                Rock => Loss,
                Paper => Win,
                Scissors => Draw
            }
        }
    }

    fn find_my_piece_depending_on_opponent_piece_and_desired_outcome(&self, other: &Outcome) -> PlayerInput {
        match *self {
            Rock => match other {
                Win => Paper,
                Draw => Rock,
                Loss => Scissors,
            },
            Paper => match other {
                Win => Scissors,
                Draw => Paper,
                Loss => Rock,
            },
            Scissors => match other {
                Win => Rock,
                Draw => Scissors,
                Loss => Paper,
            }
        }
    }

    fn to_score(&self) -> u64 {
        match *self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}