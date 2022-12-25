extern crate core;

use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let snafu_numbers = read_input(&args[1])?;

    let dec_output =
        snafu_numbers.iter()
                     .map(|snafu_number| snafu_2_dec(snafu_number))
                     .sum::<i64>();

    println!("{} is the decimal number we need", dec_output);

    println!("{} is the SNAFU number you supply to Bob's console",
             enumerate_snafu(dec_output));

    Ok(())
}

fn snafu_char_2_value(snafu_char: &char) -> i64 {
    match snafu_char {
        '=' => -2,
        '-' => -1,
        '0' =>  0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Unexpected input")
     }
}

fn snafu_2_dec(snafu: &Vec<char>) -> i64 {
    snafu.iter()
         .rev()
         .enumerate()
        .map(|(i,c)| (5 as i64).pow(i as u32) * snafu_char_2_value(c))
        .sum()
}

// Yes extremly hacky, but the elves are getting cold and I am done for the year.
fn enumerate_snafu(i : i64) -> String {
    let mut rvalue: Vec<char> = vec!('0');
    for x in 0..i {
        println!("{x} / {i}");
        let mut index = 0;
        loop {
            if index >= rvalue.len() {
                rvalue.push('1');
                break;
            } else {
                let (nchar, overflow) = snafu_adder(&rvalue[index]);
                rvalue[index] = nchar;
                index = index + 1;
                if !overflow {
                    break;
                }
            }
        }
    }

    rvalue.into_iter().rev().collect::<String>()
}

// fn prefix_2_dec(prexix: String, i: u35) {
    // let c = &prexix.chars()
    // snafu_char_2_value()
// }

// Ok we have to put in the work
fn dec_2_snafu(dec: i64) -> String {
    match dec {
        0 => "0".to_string(),
        1 => "1".to_string(),
        2 => "2".to_string(),
        3 => "1=".to_string(),
        4 => "1-".to_string(),
        5 => "10".to_string(),
        6 => "11".to_string(),
        7 => "12".to_string(),
        8 => "2=".to_string(),
        9 => "2-".to_string(),
        _ => panic!("Unexepected input")
    }
}

// Output / Overflow
fn snafu_adder(left: &char, right: &char) -> (char, Option<char>) {
    match left {
        '0' =>
            match right {
                '2' => ('2', None),
                '1' => ('1', None),
                '0' => ('0', None),
                '-' => ('-', None),
                '=' => ('=', None),
                _ => panic!("Unexpected input")
            },
        '1' =>
            match right {
                '2' => ('=', Some('1')),
                '1' => ('2', None),
                '0' => ('1', None),
                '-' => ('0', None),
                '=' => ('-', None),
                _ => panic!("Unexpected input")
            },
        '2' =>
            match right {
                '2' => ('-', Some('1')),
                '1' => ('=', Some('1')),
                '0' => ('2', None),
                '-' => ('1', None),
                '=' => ('0', None),
                _ => panic!("Unexpected input")
            },
        '=' => ('-', false),
        '-' => ('0', false),
        _ => panic!("Unexpected input")
    }
}

// (a * 5^y) * 10 =>
// 2a * 5^y+1
fn shift_to_the_lef() {

}

fn read_input(filename: &String) -> io::Result<Vec<Vec<char>>> {
    let file_in = File::open(filename)?;

    let snafu_numbers =
        BufReader::new(file_in)
            .lines()
            .map(|l|l.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

   Ok(snafu_numbers)
}