use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use crate::Instruction::{Addx, Noop};

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut video_system =  read_input(&args[1])?;

    println!("{:?} is the sum of these six signal strengths.",
             video_system.clone().solve1());

    video_system.solve2();

    Ok(())
}

fn read_input(filename: &String) -> io::Result<VideoSystem> {
    let file_in = File::open(filename)?;
    let instructions =
        BufReader::new(file_in)
            .lines()
            .map(|line| line.unwrap())
            .map(|line| {
                if line.contains("noop") {
                    Noop { cycle_length: 0 }
                } else {
                    let mut it = line.split(" ");
                    it.next();
                    Addx {
                        cycle_length: 1,
                        argument: it.next().unwrap().parse::<i64>().unwrap()
                    }
                }
            })
            .collect::<Vec<Instruction>>();
    Ok(VideoSystem::new(instructions))
}

#[derive(Debug, Clone)]
enum Instruction {
    Noop { cycle_length: u64 },
    Addx { cycle_length: u64, argument: i64 }
}

#[derive(Debug, Clone)]
struct VideoSystem {
    instructions: Vec<Instruction>,
    current_clock: u64,
    current_instruction_start: u64,
    pc: usize,
    register: i64,
}

impl VideoSystem {
    fn new(instructions: Vec<Instruction>) -> Self {
        VideoSystem {
            instructions,
            current_clock: 1,
            current_instruction_start: 1,
            pc: 0,
            register: 1,
        }
    }

    fn solve1(&mut self) -> i64 {
        let mut register_values: Vec<i64> = Vec::new();

        loop {
            self.step();

            if self.current_clock == 20
               || (self.current_clock >= 60
                    && (((self.current_clock - 60) % 40) == 0)) {
                register_values.push(self.register * (self.current_clock as i64));
            }

            if register_values.len() >= 6 {
                break;
            }
        }

        register_values.iter()
                       .sum()
    }

    fn solve2(&mut self) {
        for _ in 0..6 {
            for i in 0..40 {
                print!("{}", if ((self.register-1)..(self.register+2)).contains(&(i as i64)) {
                    '#'
                } else {
                    '.'
                });
                self.step();

            }
            print!("\n")
        }
    }

    fn next_instruction(&mut self) {
        self.pc = self.pc + 1;
        self.current_instruction_start = self.current_clock + 1;
    }

    fn instruction_done(&self, cycle_length: &u64) -> bool {
        self.current_instruction_start + cycle_length <= self.current_clock
    }

    fn step(&mut self) {
        let current_instruction = self.instructions.get(self.pc).unwrap();

        match current_instruction {
            Noop { cycle_length} => {
                if self.instruction_done(cycle_length) {
                    self.next_instruction();
                }
            },
            Addx { cycle_length, argument} => {
                if self.instruction_done(cycle_length) {
                    self.register = self.register + argument;
                    self.next_instruction();
                }
            }
        }

        self.current_clock += 1;
    }
}




