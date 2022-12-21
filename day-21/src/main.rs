use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let expression_map  = read_input(&args[1])?;

    println!("{:?} is the number will the monkey named root yell",
            solve1("root", &expression_map));

    println!("{:?} is the number you yell to pass root's equality test",
             solve2(&expression_map));

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

fn solve2(expression_map: &HashMap<String, Expression>) -> i64 {
    let root_expression: &Expression = expression_map.get("root").unwrap();

    let (left, right) = root_expression.operands();

    let left_contains_human = contains_human(left.as_str(), expression_map);

    let right_contains_human = contains_human(right.as_str(), expression_map);

    assert!(left_contains_human ^ right_contains_human);

    let needed_amount =
        if left_contains_human {
            solve1(right.as_str(), expression_map)
        } else {
            solve1(left.as_str(), expression_map)
        };

    let human_expression=
        if left_contains_human {
            left.as_str()
        } else {
            right.as_str()
        };

    do_solve2(human_expression,
              needed_amount,
              expression_map)
}

fn do_solve2(expression_name: &str,
             needed_amount: i64,
             expression_map: &HashMap<String, Expression>) -> i64 {

    let expression = expression_map.get(expression_name).unwrap();
    if expression_name == "humn" {
        return needed_amount
    }

    assert!(!expression.is_constant());

    let (left, right) = expression.operands();

    let left_contains_human = contains_human(left.as_str(), expression_map);

    let right_contains_human = contains_human(right.as_str(), expression_map);

    assert!(left_contains_human ^ right_contains_human);

    let evaluation =
        if left_contains_human {
            solve1(right.as_str(), expression_map)
        } else {
            solve1(left.as_str(), expression_map)
        };

    let human_expression=
        if left_contains_human {
            left.as_str()
        } else {
            right.as_str()
        };

    match expression {
        Expression::Plus(_l, _r) => {
            if left_contains_human {
                // needed_amount = h + evaluation =>
                // h = needed_amount - evaluation
                do_solve2(human_expression,
                          needed_amount - evaluation,
                          expression_map)
            } else {
                // needed_amount = evaluation + h =>
                // h = needed_amount - evaluation
                do_solve2(human_expression,
                          needed_amount - evaluation,
                          expression_map)
            }
        },
        Expression::Minus(_l, _r) => {
            if left_contains_human {
                // needed_amount = h - evaluation =>
                // needed_amount + evaluation = h
                do_solve2(human_expression,
                          needed_amount + evaluation,
                          expression_map)
            } else {
                // needed_amount = evaluation - h =>
                // h = (needed_amount - evaluation) * -1
                do_solve2(human_expression,
                          (needed_amount - evaluation) * -1,
                          expression_map)
            }
        },
        Expression::Times(_l, _r) => {
            if left_contains_human {
                // needed_amount = h * evaluation =>
                // h = needed_amount / evaluation
                do_solve2(human_expression,
                          needed_amount / evaluation,
                          expression_map)
            } else {
                // needed_amount = evaluation * h =>
                // h = needed_amount / evaluation
                do_solve2(human_expression,
                          needed_amount / evaluation,
                          expression_map)
            }
        },
        Expression::Divide(_l, _r) => {
            if left_contains_human {
                // needed_amount = h / evaluation =>
                // h = needed_amount * evaluation
                do_solve2(human_expression,
                          needed_amount * evaluation,
                          expression_map)
            } else {
                // needed_amount = evaluation / h =>
                //  => h * needed_amount = evaluation
                //  => h = evaluation / needed_amount
                do_solve2(human_expression,
                           evaluation / needed_amount,
                          expression_map)
            }
        },
        Expression::Constant(_) => {
            panic!("Constants don't have inverses")
        }
    }
}

fn contains_human(expression: &str, expression_map: &HashMap<String, Expression>) -> bool {
    return expression == "humn"
           ||  match expression_map.get(expression).unwrap() {
                    Expression::Plus(l, r) => {
                        contains_human(l, expression_map)
                        || contains_human(r, expression_map)
                    },
                    Expression::Minus(l, r) => {
                        contains_human(l, expression_map)
                        || contains_human(r, expression_map)
                    },
                    Expression::Times(l, r) => {
                        contains_human(l, expression_map)
                        || contains_human(r, expression_map)
                    },
                    Expression::Divide(l, r) => {
                        contains_human(l, expression_map)
                        || contains_human(r, expression_map)
                    },
                    Expression::Constant(_) => {
                        false
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

impl Expression {
    fn is_constant(&self) -> bool {
        match self {
            Expression::Constant(_v) => {
                true
            },
            _  => false
        }
    }

    fn operands(&self) -> (String, String) {
        match self {
            Expression::Plus(l, r) => {
                (l.clone(), r.clone())
            },
            Expression::Minus(l, r) => {
                (l.clone(), r.clone())
            },
            Expression::Times(l, r) => {
                (l.clone(), r.clone())
            },
            Expression::Divide(l, r) => {
                (l.clone(), r.clone())
            },
            Expression::Constant(_) => {
                panic!("Constants don't have operators")
            }
        }
    }
}