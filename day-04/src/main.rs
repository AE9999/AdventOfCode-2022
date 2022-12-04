use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::ops::Range;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
}

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;

    println!("{:?} is how many assignment pairs does one range fully contain the other",
             input.iter()
                 .filter(|x|x.one_contains_the_other())
                 .fold(0, |sum, _val| sum + 1));

    println!("{:?} is in how many assignment pairs do the ranges overlap",
        input.iter()
                 .filter(|x|x.one_has_some_intersection_with_other())
                 .fold(0, |sum, _val| sum + 1));
    Ok(())
}

fn read_input(filename: &String) ->  io::Result<Vec<RangePair>> {
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in).lines();
    Ok(file_reader.map(|x|x.unwrap())
                  .map(|x| string_2_range_pair(x))
                  .collect::<Vec<RangePair>>())
}

fn string_2_range_pair(s: String) -> RangePair {
    let mut cap = RE.captures_iter(s.trim());
    let cap  = cap.next().unwrap();
    let lb_l = cap[1].parse::<i32>().unwrap();
    let ub_l = cap[2].parse::<i32>().unwrap() + 1;
    let lb_r = cap[3].parse::<i32>().unwrap();
    let ub_r = cap[4].parse::<i32>().unwrap() + 1;

    RangePair::new(lb_l..ub_l, lb_r..ub_r)
}

struct RangePair {
    left: Range<i32>,
    right: Range<i32>,
}

impl RangePair {
    fn new(left: Range<i32>, right: Range<i32>)  -> RangePair {
        RangePair {
            left,
            right
        }
    }

    fn one_contains_the_other(&self) -> bool {
        (self.left.contains(&self.right.start)
            && self.left.contains(&(&self.right.end - 1)))
        || (self.right.contains(&self.left.start)
            && self.right.contains(&(&self.left.end - 1)))
    }

    fn one_has_some_intersection_with_other(&self) -> bool {
        self.left.contains(&self.right.start)
        || self.left.contains(&(&self.right.end - 1))
        || self.right.contains(&self.left.start)
        || self.right.contains(&(&self.left.end - 1))
    }

}
