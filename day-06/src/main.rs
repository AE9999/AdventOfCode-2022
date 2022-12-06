use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use itertools::Itertools;


fn main() ->  io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    read_input(input)?.iter()
                              .for_each(|s| {
                                println!("{:?} characters need to be processed before the first start-of-packet marker is detected.",
                                         find_start_of_pattern(s,4 as usize));
                                  println!("{:?} characters need to be processed before the first start-of-message marker is detected.",
                                         find_start_of_pattern(s,14 as usize))
                              });

    Ok(())
}

fn find_start_of_pattern(buffer: &Vec<char>, window_size: usize) -> usize {
    buffer.windows(window_size)
          .enumerate()
          .skip_while(|slice| {
              let x =  (*slice).1.iter().map(|x|x.clone()).collect::<Vec<char>>();
              let set = x.iter().unique().collect::<Vec<&char>>();
              set.len() < window_size
          }).take(1)
            .fold(0, |_sum, val| val.0) + // The window
             (window_size - 1) + // We look 'window_size' chars ahead
             (1 as usize)  // This is an index not a position
}

fn read_input(filename: &String) -> io::Result<Vec<Vec<char>>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in)
                 .lines()
                 .map(|s|s.unwrap().chars().collect::<Vec<char>>())
                 .collect::<Vec<Vec<char>>>())
}
