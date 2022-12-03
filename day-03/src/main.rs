use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use lazy_static::lazy_static;

lazy_static! {
    static ref HASHMAP: HashMap<char, u64> = {
        let mut m : HashMap<char, u64> = HashMap::new();
        ('a'..='z').into_iter().enumerate().for_each(|(i,c)|{
            m.insert(c, (i + 1) as u64);
        });
        ('A'..='Z').into_iter().enumerate().for_each(|(i,c)|{
            m.insert(c, (i + 27) as u64);
        });
        m
    };
}

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    println!("{:?} is the sum of the priorities of those item types",
             read_input(input)?.iter()
                 .map(|x|x.intersection().iter().map(|c|priority(c)).sum::<u64>())
                 .sum::<u64>());

    println!("{:?} is the sum of the priorities of those item types",
             read_input_part2(input)?.iter()
                 .map(|x|x.common_items().iter().map(|c|priority(c)).sum::<u64>())
                 .sum::<u64>());
    Ok(())
}

fn read_input(filename: &String) ->  io::Result<Vec<Rucksack>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();
    Ok(file_reader.map(|x| x.unwrap())
                  .map(|x|Rucksack::new(x))
                  .collect())
}

fn read_input_part2(filename: &String) ->  io::Result<Vec<Group>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();
    Ok(file_reader.map(|x| x.unwrap())
        .map(|x|Rucksack::new(x))
        .collect::<Vec<Rucksack>>()
        .chunks(3)
        .map(|x| x.into_iter()
                             .map(|x|x.clone())
                             .collect::<Vec<Rucksack>>())
        .map(|x|Group::new(x))
        .collect::<Vec<Group>>()
     )
}

fn priority(c: &char) -> &u64 {
    HASHMAP.get(c).unwrap()
}

#[derive(Clone)]
struct Rucksack {
    left: Vec<char>,
    right: Vec<char>
}

impl Rucksack {
    fn new(input: String) -> Self {
        let left: Vec<char> = input[0..input.len() / 2].chars().collect();
        let right: Vec<char> = input[input.len() / 2..input.len()].chars().collect();
        Rucksack {
            left,
            right,
        }
    }

    fn intersection(&self) -> Vec<char> {
        let left : HashSet<char> = HashSet::from_iter(self.left.clone().into_iter());
        let right: HashSet<char>  = HashSet::from_iter(self.right.clone().into_iter());
        let intersection: Vec<char> = left.intersection(&right).map(|x|x.clone()).collect();
        intersection
    }

    fn union(&self) -> HashSet<char> {
        let mut rvalue: Vec<char> = Vec::new();
        rvalue.extend(&self.left);
        rvalue.extend(&self.right);
        HashSet::from_iter(rvalue.into_iter())
    }
}

struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    fn new(rucksacks: Vec<Rucksack>) -> Self {
        Group {
            rucksacks
        }
    }

    fn common_items(&self) -> HashSet<char> {
        // TODO for next year find elegant way to loop through sets
        let mut start = self.rucksacks.get(0).unwrap().union();
        start = start.intersection(&self.rucksacks.get(1).unwrap().union()).into_iter().map(|x| x.clone()).collect::<HashSet<char>>();
        start = start.intersection(&self.rucksacks.get(2).unwrap().union()).into_iter().map(|x| x.clone()).collect::<HashSet<char>>();
        start
    }
}