use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let mut input = read_input(input)?;

    println!("{:?}  Is the total of calories that that elf is carrying  ..",
             input.iter().map(|x| x.total_calories()).max().unwrap());

    input.sort_by(|l,r| r.total_calories().cmp(&l.total_calories()));


    let weight: u64 = input.iter().take(3).map(|x|x.total_calories()).sum();
    println!("{:?}  Is the total of calories that those elves are carrying  ..",
             weight);

    Ok(())
}


fn read_input(filename: &String) ->  io::Result<Vec<Elf>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();

    let mut callories : Vec<u64> = Vec::new();
    let mut elves: Vec<Elf> = Vec::new();
    for line in file_reader {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(Elf::new(callories.clone()));
            callories.clear();
        } else {
            callories.push(line.parse().unwrap());
        }
    }
    elves.push(Elf::new(callories.clone()));

    Ok(elves)
}

struct Elf {
    calories: Vec<u64>
}

impl Elf {
    pub fn new(calories: Vec<u64>) -> Self {
        Elf {
            calories
        }
    }

    pub fn total_calories(&self) -> u64 {
        self.calories.iter().sum()
    }
}