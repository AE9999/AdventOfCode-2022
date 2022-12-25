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
             dec_to_big_endian_snafu(dec_output).iter().rev().collect::<String>());

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

fn dec_to_big_endian_snafu(dec: i64) -> Vec<char> {
    if dec > 10 {
        let quotient = dec / 2;
        let remainder = dec % 2;
        let snafu_quotient = dec_to_big_endian_snafu(quotient);
        let snafu_remainder = dec_to_big_endian_snafu(remainder);

        let added_quotient =
            add_two_big_endian_snafu_numbers(snafu_quotient.clone(),
                                            snafu_quotient.clone());

        add_two_big_endian_snafu_numbers(added_quotient,
                                         snafu_remainder)

    } else {
        simple_dec_to_big_endian_snafu(dec)
    }
}

// Ok we have to put in the work
fn simple_dec_to_big_endian_snafu(dec: i64) -> Vec<char> {
    match dec {
        0 => "0".to_string().chars().rev().collect::<Vec<char>>(),
        1 => "1".to_string().chars().rev().collect::<Vec<char>>(),
        2 => "2".to_string().chars().rev().collect::<Vec<char>>(),
        3 => "1=".to_string().chars().rev().collect::<Vec<char>>(),
        4 => "1-".to_string().chars().rev().collect::<Vec<char>>(),
        5 => "10".to_string().chars().rev().collect::<Vec<char>>(),
        6 => "11".to_string().chars().rev().collect::<Vec<char>>(),
        7 => "12".to_string().chars().rev().collect::<Vec<char>>(),
        8 => "2=".to_string().chars().rev().collect::<Vec<char>>(),
        9 => "2-".to_string().chars().rev().collect::<Vec<char>>(),
        10 => "20".to_string().chars().rev().collect::<Vec<char>>(),
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
        '=' =>
            match right {
                '2' => ('0', None),
                '1' => ('-', None),
                '0' => ('=', None),
                '-' => ('2', Some('-')),
                '=' => ('1', Some('-')),
                _ => panic!("Unexpected input")
            },
        '-' =>
            match right {
                '2' => ('1', None),
                '1' => ('0', None),
                '0' => ('-', None),
                '-' => ('=', None),
                '=' => ('2', Some('-')),
                _ => panic!("Unexpected input")
            },
        _ => panic!("Unexpected input")
    }
}

fn add_two_big_endian_snafu_numbers(left: Vec<char>,
                                    right: Vec<char>) -> Vec<char> {

    let mut rvalue: Vec<char> = vec!();

    let (longest, shortest) =
        if left.len() > right.len() {
            (&left, &right)
        } else {
            (&right, &left)
        }
    ;

    let mut carry: Option<char> = None;

    for index in 0..(shortest.len()) {
        let res = longest[index];

        let (res, carry_) =
            if carry.is_some() {
                snafu_adder(&res, &carry.unwrap())
            }  else {
                (res, None)
            }
        ;
        carry = carry_;

        let right = right[index];

        let (res, additional_carry) =
            snafu_adder(&res, &right);

        assert!(carry.is_none() || additional_carry.is_none());

        rvalue.push(res);

        carry =
            if carry.is_some() {
                carry
            } else if additional_carry.is_some() {
                additional_carry
            } else {
                None
            }
        ;
    }

    for index in shortest.len()..longest.len() {
        let (res, carry_) =
            if carry.is_some() {
                snafu_adder(&longest[index], &carry.unwrap())
            }  else {
                (longest[index], None)
            };
        carry = carry_;
        rvalue.push(res);
    }

    if carry.is_some() {
        let carry = carry.unwrap();
        assert!(carry != '=' && carry != '0' && carry != '-');
        rvalue.push(carry);
    }

    rvalue
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