use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let problem  = read_input(&args[1])?;
    println!("Hello, world!");
    Ok(())
}

fn read_input(filename: &String) -> io::Result<()> {
    let file_in = File::open(filename)?;

    let mut reading_tiles = true;

    let mut tiles : Vec<Vec<char>> = vec!();

    let mut it =
        BufReader::new(file_in).lines().map(|line| line.unwrap()).into_iter();
    Ok(())
}
