use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
        static ref INSTRUCTION_RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let puzzle = read_input(input)?;
    println!("After the rearrangement procedure completes the crates {:?} end up on top of each stack",
             puzzle.clone().solve1());

    println!("After the rearrangement procedure completes the crates {:?} end up on top of each stack",
             puzzle.clone().solve2());

    Ok(())
}

fn read_input(filename: &String) ->  io::Result<Puzzle> {
    let file_in = File::open(filename)?;
    let lines = BufReader::new(file_in).lines();
    let mut reading_state: ReadingState = ReadingState::ReadingStacks;

    let mut raw_stacks: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut label_line: String = String::from("");

    for line in lines {
        let line = line.unwrap();
        match reading_state {
            ReadingState::ReadingStacks => {
                if contains_crate(line.as_str()) {
                    raw_stacks.push(line.chars().collect::<Vec<char>>())
                } else {
                    label_line = line;
                    reading_state = ReadingState::ExpectingEmptyLine;
                }
            },
            ReadingState::ExpectingEmptyLine => {
                reading_state = ReadingState::ReadingInstructions;
            },
            ReadingState::ReadingInstructions=> {
                let mut cap = INSTRUCTION_RE.captures_iter(line.trim());
                let cap  = cap.next().unwrap();
                let amount = cap[1].parse::<usize>().unwrap();
                let from = cap[2].parse::<usize>().unwrap();
                let to = cap[3].parse::<usize>().unwrap() ;
                instructions.push(Instruction::new(amount, from, to))
            }
        }
    }
    let amount_of_crates: u32 = label_line.chars().last().unwrap().to_digit(10).unwrap();

    let first_crate_index: usize = 1;

    let mut my_stacks: Vec<Stack> = Vec::new();
    (0..amount_of_crates).for_each(|_| my_stacks.push( Stack::new() ));
    (0..raw_stacks.len()).rev().for_each(|y| {
        (0..amount_of_crates).map(|x| x as usize).for_each(|x| {
            let stack = raw_stacks.get(y).unwrap();
            let effective_index = first_crate_index + ("] [ ".len() * x);
            let c = stack.get(effective_index).unwrap_or(&' ');
            if c != &' ' {
                my_stacks.get_mut(x).unwrap().push_crate(*c);
            }

        })
    });

    Ok(Puzzle::new(my_stacks,
                   instructions))
}

fn contains_crate(line: &str) -> bool {
    line.contains('[')
}

#[derive(Debug, Clone)]
enum ReadingState {
    ReadingStacks,
    ExpectingEmptyLine,
    ReadingInstructions
}

#[derive(Debug, Clone)]
struct Puzzle {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn new(stacks: Vec<Stack>,
           instructions: Vec<Instruction>) -> Self {
        Puzzle {
            stacks,
            instructions
        }
    }

    fn solve1(&mut self) -> String {
        self.instructions.iter().for_each(|instruction| {

            let mut buffer: Vec<char> = Vec::new();

            {
                let from = self.stacks.get_mut(instruction.from - 1).unwrap();
                (0..instruction.amount).for_each(|_| {
                    buffer.push(from.pop_crate());
                });
            }

            let to = self.stacks.get_mut(instruction.to - 1).unwrap();
            for c in buffer {
                to.push_crate(c);
            }

        });
        self.stacks.iter().map(|stack|stack.peek()).collect::<String>()
    }

    fn solve2(&mut self) -> String {
        self.instructions.iter().for_each(|instruction| {

            let mut buffer: Vec<char> = Vec::new();

            {
                let from = self.stacks.get_mut(instruction.from - 1).unwrap();
                (0..instruction.amount).for_each(|_| {
                    buffer.push(from.pop_crate());
                });
            }

            let to = self.stacks.get_mut(instruction.to - 1).unwrap();
            for c in buffer.iter().rev() {
                to.push_crate(*c);
            }

        });
        self.stacks.iter().map(|stack|stack.peek()).collect::<String>()
    }
}

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<char>
}

impl Stack {
    fn new() -> Self {
        Stack {
            crates: Vec::new()
        }
    }

    fn push_crate(&mut self, c: char) {
        self.crates.push(c)
    }

    fn pop_crate(&mut self) -> char {
        self.crates.pop().unwrap()
    }

    fn peek(&self) -> char {
        self.crates.last().unwrap().clone()
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl  Instruction {
    fn new(amount: usize, from: usize, to: usize)  -> Self {
        Instruction {
            amount,
            from,
            to
        }
    }
}
