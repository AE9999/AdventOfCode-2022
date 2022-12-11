use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use crate::Operand::Constant;

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let monkeys = read_input(&args[1])?;

    println!("{:?} is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans",
             solve1(monkeys.clone()));

    Ok(())
}

fn solve1(mut monkeys: Vec<Monkey>) -> usize {
   for _ in 0..20 {

       for i in 0..monkeys.len() {
            let appensions = monkeys.get_mut(i).unwrap().do_round();
            for x in appensions.iter() {
                let mut items = (x.1).clone();
                monkeys.get_mut(*(x.0)).unwrap().extend_items(&mut items);
            }
        }
   }

    let mut inspections =
        monkeys.iter()
               .map(|monkey|monkey.nr_of_inspections)
               .collect::<Vec<usize>>();

    inspections.sort();

    inspections.iter().rev().take(2).fold(1, |prod, val| prod * val)

}

fn read_input(filename: &String) -> io::Result<Vec<Monkey>> {
    let file_in = File::open(filename)?;


    let raw_lines =
        BufReader::new(file_in).lines()
                  .map(|line|line.unwrap())
                  .collect::<Vec<String>>();

    let mut lines = raw_lines.iter();

    let mut monkeys: Vec<Monkey> = Vec::new();

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
                items,
                left,
                right,
                operator,
                test,
                test_true,
                test_false,
                nr_of_inspections: 0
            };

            monkeys.push(monkey);
        }
    }

    Ok(monkeys)
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
    items: Vec<usize>,
    left: Operand,
    right: Operand,
    operator: Operator,
    test: usize,
    test_true: usize,
    test_false: usize,
    nr_of_inspections: usize
}

impl Monkey {
    fn do_round(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut rvalue: HashMap<usize, Vec<usize>> = HashMap::new();
        rvalue.insert(self.test_true,Vec::new());
        rvalue.insert(self.test_false,Vec::new());

        for item in &self.items {
            let new_worry_level = self.new_worry_level(*item);
            rvalue.get_mut(&self.find_target(new_worry_level)).unwrap().push(new_worry_level);
            self.nr_of_inspections += 1;
        }

        self.items = Vec::new();
        rvalue
    }

    fn new_worry_level(&self, current_worry_level: usize) -> usize {
        self.operator.evaluate(&self.left, &self.right,current_worry_level ) / 3
    }

    fn find_target(&self, new_worry_level: usize) -> usize {
        if new_worry_level % self.test == 0 {
            self.test_true
        } else {
            self.test_false
        }
    }

    fn extend_items(&mut self, items: &mut Vec<usize>) {
        self.items.append(items);
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
