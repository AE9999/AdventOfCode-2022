use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let  expression_map  = read_input(&args[1])?;

    println!("{:?} is the number will the monkey named root yell",
            solve1("root", &expression_map));

    Ok(())
}

fn solve1(key: &str, expression_map: &HashMap<String, Expression>) -> i64 {
    match expression_map.get(key).unwrap() {
        Expression::Plus(l, r) => {
            solve1(l, expression_map) + solve1(r, expression_map)
        },
        Expression::Minus(l, r) => {
            solve1(l, expression_map) - solve1(r, expression_map)
        },
        Expression::Times(l, r) => {
            solve1(l, expression_map) * solve1(r,expression_map)
        },
        Expression::Divide(l, r) => {
            solve1(l, expression_map) / solve1(r,expression_map)
        },
        Expression::Constant(v) => {
            *v
        }
    }
}

fn read_input(filename: &String) -> io::Result<HashMap<String, Expression>> {
    let file_in = File::open(filename)?;

    let mut rvalue: HashMap<String, Expression> = HashMap::new();

    BufReader::new(file_in)
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line|{
            let mut it = line.split(": ");
            let left = it.next().unwrap().to_string();
            let to_parse = it.next().unwrap();
            let expression: Expression =

                if to_parse.contains(" + ") {
                    let mut it = to_parse.split(" + ");
                    Expression::Plus(it.next().unwrap().to_string(),
                                     it.next().unwrap().to_string())
                } else if to_parse.contains(" - ") {
                    let mut it = to_parse.split(" - ");
                    Expression::Minus(it.next().unwrap().to_string(),
                                     it.next().unwrap().to_string())
                } else if  to_parse.contains(" * ") {
                    let mut it = to_parse.split(" * ");
                    Expression::Times(it.next().unwrap().to_string(),
                                      it.next().unwrap().to_string())
                } else if to_parse.contains(" / ") {
                    let mut it = to_parse.split(" / ");
                    Expression::Divide(it.next().unwrap().to_string(),
                                      it.next().unwrap().to_string())
                }  else {
                    let constant: i64 = to_parse.parse::<i64>().unwrap();
                    Expression::Constant(constant)
                }
            ;

            rvalue.insert(left, expression);
        });

    Ok(rvalue)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Expression {
    Plus(String, String) ,
    Minus(String, String),
    Times(String, String),
    Divide(String, String),
    Constant(i64)
}