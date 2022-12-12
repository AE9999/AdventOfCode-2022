extern crate core;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use crate::Operand::Constant;

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut problem = read_input(&args[1])?;

    println!("{:?} is the level of monkey business after 10000 rounds",
             problem.solve());

    Ok(())
}

fn read_input(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;


    let raw_lines =
        BufReader::new(file_in).lines()
                  .map(|line|line.unwrap())
                  .collect::<Vec<String>>();

    let mut lines = raw_lines.iter();

    let mut problem = Problem::new();

    loop {
        let line = lines.next();

        if line.is_none() {
            break;
        }

        if line.unwrap().starts_with("Monkey") {
            let items
                = lines.next()
                       .unwrap()
                       .chars()
                       .skip("  Starting items: ".len())
                       .collect::<String>()
                       .split(", ")
                       .map(|x| x.parse::<usize>().unwrap())
                       .collect::<Vec<usize>>();

            let mut item_ids: Vec<usize> = Vec::new();
            for starting_worry_value in items {
                item_ids.push(problem.register_item(starting_worry_value))
            }


            let operation_line =
                lines.next()
                     .unwrap()
                     .chars()
                     .skip("  Operation: new = ".len())
                     .collect::<String>();

            let plus_operation = operation_line.contains(" + ");
            let mut split =
                if plus_operation {
                    operation_line.split(" + ")
                } else {
                    operation_line.split(" * ")
                };
            let left = split.next().map(|x| parse_operand(x)).unwrap();
            let right = split.next().map(|x| parse_operand(x)).unwrap();
            let operator =
                if plus_operation {
                    Operator::Plus
                }  else {
                    Operator::Times
                };

            let test =
                lines.next()
                     .unwrap()
                     .chars()
                     .skip("  Test: divisible by ".len())
                     .collect::<String>()
                     .parse::<usize>().unwrap();
            let test_true =
                lines.next()
                    .unwrap()
                    .chars()
                    .skip("    If true: throw to monkey ".len())
                    .collect::<String>()
                    .parse::<usize>().unwrap();
            let test_false =
                lines.next()
                    .unwrap()
                    .chars()
                    .skip("    If false: throw to monkey ".len())
                    .collect::<String>()
                    .parse::<usize>().unwrap();

            let monkey =
                Monkey {
                item_ids,
                left,
                right,
                operator,
                test,
                test_true,
                test_false,
                nr_of_inspections: 0
            };

            problem.monkeys.push(monkey)
        }
    }

    problem.initialize_remainders();

    Ok(problem)
}

fn parse_operand(operand: &str) -> Operand {
    if operand == "old" {
        Operand::Old
    } else {
        Constant(operand.parse::<usize>().unwrap())
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    item_ids: Vec<usize>,
    left: Operand,
    right: Operand,
    operator: Operator,
    test: usize,
    test_true: usize,
    test_false: usize,
    nr_of_inspections: usize
}

impl Monkey {

    fn extend_items(&mut self, item_ids: &mut Vec<usize>) {
        self.item_ids.append(item_ids);
    }

}

struct Problem {
    items: HashMap<usize, usize>,
    remainder_for_monkey_for_item_id: HashMap<(usize, usize), usize>,
    monkeys: Vec<Monkey>
}

impl Problem {

    fn new() -> Self {
        Problem {
            items: HashMap::new(),
            remainder_for_monkey_for_item_id: HashMap::new(),
            monkeys: Vec::new(),
        }
    }

    fn register_item(&mut self, starting_worry_value: usize) -> usize {
        let id = self.items.len();
        self.items.insert(id, starting_worry_value);
        return id
    }

    fn do_round_with_side_effects(&mut self, i: usize) -> HashMap<usize, Vec<usize>> {
        let throwing_monkey = self.monkeys.get(i).unwrap();

        let mut rvalue: HashMap<usize, Vec<usize>> = HashMap::new();

        rvalue.insert(throwing_monkey.test_true, Vec::new());

        rvalue.insert(throwing_monkey.test_false, Vec::new());

        for item_id in &throwing_monkey.item_ids {

            for monkey in self.monkeys.iter() {
                let key = (monkey.test, *item_id);
                let remainder = *(self.remainder_for_monkey_for_item_id.get(&key).unwrap());

                let value = Self::recalculate_remainder(&throwing_monkey.operator,
                                                              &throwing_monkey.right,
                                                              monkey.test,
                                                              remainder);

                self.remainder_for_monkey_for_item_id.insert(key, value);
            }


            let key = (throwing_monkey.test, *item_id);
            let value= *(self.remainder_for_monkey_for_item_id.get(&key).unwrap());

            if value == 0 {
                rvalue.get_mut(&(throwing_monkey.test_true)).unwrap().push(*item_id)
            } else {
                rvalue.get_mut(&(throwing_monkey.test_false)).unwrap().push(*item_id)
            }

        }

        rvalue
    }

    fn initialize_remainders(&mut self) {
        for item in self.items.iter() {
            for monkey in self.monkeys.iter() {
                let key = (monkey.test, *item.0);
                let value = *item.1 % monkey.test;
                self.remainder_for_monkey_for_item_id.insert(key, value);
            }
        }
    }

    fn recalculate_remainder(operator: &Operator,
                             right: &Operand,
                             test: usize,
                             remainder: usize)-> usize {
        match operator {
            // ((x * test) + remainder) + constant =>
            Operator::Plus => {
                let value =
                    match right {
                        Operand::Constant(value)  => value,
                        _ => panic!("Right has to be constant")
                    };
                (*value + remainder) % test
            },
            Operator::Times => {
                match right {
                    Operand::Old => {
                        // ((x * test) + remainder)^2  => remainder^2 % remainder
                        (remainder * remainder) % test
                    },
                    Operand::Constant(value) => {
                        // ((x * test) + remainder) * value => (remainder * constant) % remainder
                        (remainder * value) % test
                    }
                }
            }
        }
    }

    fn solve(&mut self) -> usize {
        for _ in 0..10000 {


            for i in 0..self.monkeys.len() {

                let appensions = self.do_round_with_side_effects(i);

                for x in appensions.iter() {
                    let mut items = (x.1).clone();
                    self.monkeys.get_mut(*(x.0)).unwrap().extend_items(&mut items);
                }

                let monkey = self.monkeys.get_mut(i).unwrap();
                monkey.nr_of_inspections += monkey.item_ids.len();
                monkey.item_ids.clear();
            }

            // if x < 10 {
            //     println!("After round {:?} ..", x + 1);
            //     for i in 0..self.monkeys.len() {
            //         println!("Monkey {:?} inspected items {:?} times", i, self.monkeys.get(i).unwrap().nr_of_inspections);
            //         println!("Monkey {:?} => {:?} ", i, self.monkeys.get(i).unwrap().item_ids);
            //     }
            // }
        }

        let mut inspections =
            self.monkeys.iter()
                .map(|monkey|monkey.nr_of_inspections)
                .collect::<Vec<usize>>();

        inspections.sort();

        inspections.iter().rev().take(2).fold(1, |prod, val| prod * val)

    }
}


#[derive(Debug, Clone)]
enum Operand {
    Constant(usize),
    Old,
}

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Times,
}