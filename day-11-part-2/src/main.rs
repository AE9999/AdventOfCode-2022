use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use crate::Operand::Constant;

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut problem = read_input(&args[1])?;

    println!("{:?} is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans",
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

            problem.register_monkey(monkey)
        }
    }

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


        for monkey in &self.monkeys {
            let key = (id, monkey.test);
            let value = starting_worry_value % monkey.test;
            self.remainder_for_monkey_for_item_id.insert(key, value);
        }

        return id
    }

    fn register_monkey(&mut self, monkey: Monkey) {
        for item_id in 0..self.items.len() {
            let key = (item_id, monkey.test);
            let value = self.items.get(&item_id).unwrap() % monkey.test;
            self.remainder_for_monkey_for_item_id.insert(key, value);
        }

        self.monkeys.push(monkey)
    }

    fn do_round(&mut self, i: usize) -> HashMap<usize, Vec<usize>> {
        let mut monkey = self.monkeys.get_mut(i).unwrap();

        let mut rvalue: HashMap<usize, Vec<usize>> = HashMap::new();

        rvalue.insert(monkey.test_true,Vec::new());

        rvalue.insert(monkey.test_false,Vec::new());

        for item_id in &monkey.item_ids {
            let key = (monkey.test, *item_id);

            let worry_level_remainder = self.remainder_for_monkey_for_item_id.get(&key).unwrap();

            // TODO(Recalculate this for all buckets)
            let new_worry_level_remainder = 0 as usize;
            self.remainder_for_monkey_for_item_id.insert(key, new_worry_level_remainder);

            if new_worry_level_remainder == 0 {

            } else {

            }
            // rvalue.get_mut(&self.find_target(new_worry_level)).unwrap().push(new_worry_level);

            monkey.nr_of_inspections += 1;
        }

        monkey.item_ids = Vec::new();
        rvalue
    }

    fn solve(&mut self) -> usize {
        for _ in 0..10000 {

            for i in 0..self.monkeys.len() {

                let appensions = self.do_round(i);


                for x in appensions.iter() {
                    let mut items = (x.1).clone();
                    self.monkeys.get_mut(*(x.0)).unwrap().extend_items(&mut items);
                }
            }
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

impl Operand {
    fn evaluate(&self, old: usize)  -> usize {
        *(match self {
            Operand::Constant(value) => value,
            Operand::Old => &old
        })
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Times,
}

impl Operator {
    fn evaluate(&self, left: &Operand, right: &Operand, old: usize) -> usize {
        match self {
            Operator::Plus => left.evaluate(old) + right.evaluate(old),
            Operator::Times => left.evaluate(old) * right.evaluate(old)
        }
    }
}
