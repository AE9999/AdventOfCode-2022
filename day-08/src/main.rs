use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut tree_map = read_input(&args[1])?;

    println!("{:?} is how many trees are visible from outside the grid", tree_map.solve1());

    println!("{:?} is the highest scenic score possible for any tree", tree_map.solve2());
    Ok(())
}

fn read_input(filename: &String) -> io::Result<TreeMap> {
    let file_in = File::open(filename)?;
    let trees =
        BufReader::new(file_in).lines()
                                     .map(|line|line.unwrap()
                                                                 .chars()
                                                                 .map(|c|c.to_digit(10)
                                                                                .unwrap())
                                                                                .collect::<Vec<u32>>())
                                     .collect::<Vec<Vec<u32>>>();
    Ok(TreeMap::new(trees))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TreeMap {
    trees: Vec<Vec<u32>>,
    visible_trees: HashSet<Point>,
    p_heighest: Option<u32>,
}

impl TreeMap {

    fn new(trees: Vec<Vec<u32>>) -> Self {
        TreeMap {
            trees,
            visible_trees: HashSet::new(),
            p_heighest: None,
        }
    }

    fn do_compare(&mut self,
                  point: Point)  {

        if self.p_heighest.is_none()
            || self.p_heighest
                   .unwrap() < self.trees[point.y][point.x] {
            self.p_heighest = Some(self.trees[point.y][point.x]);
            self.visible_trees.insert(point);
        }
    }

    fn solve1(&mut self) -> usize {


        for y in 0..self.height() {

            self.p_heighest = None;
            for x in 0..self.len() {
                self.do_compare(Point::new(x, y))
            }

            self.p_heighest = None;
            for x in  (0..self.len()).rev() {
                self.do_compare(Point::new(x,y))
            }
        };

        for x in (0..self.len()).rev() {

            self.p_heighest = None;
            for y in 0..self.height() {
                self.do_compare(Point::new(x,y))
            }

            self.p_heighest = None;
            for y in  (0..self.height()).rev() {
                self.do_compare(Point::new(x,y))
            }
        };

        self.visible_trees
            .len()
    }

    fn solve2(&self) -> usize {
        // Trees at border have automatic viewing scores of 0 due to ..
        (1..self.height() - 1).map(|y|{
            (1..self.len() - 1).map(|x| self.visible_trees_from_point(Point::new(x,y)))
                           .max()
                           .unwrap()

        }).max()
          .unwrap()
    }

    fn visible_trees_from_point(&self, point: Point) -> usize {
        let mut visible_trees_to_the_left: usize = 0;
        for x in (0..point.x).rev() {

            visible_trees_to_the_left += 1;
            if self.trees[point.y][point.x] <= self.trees[point.y][x] {
                break;
            }
        }

        let mut visible_trees_to_the_right: usize = 0;
        for x in (point.x + 1)..self.len() {
            visible_trees_to_the_right += 1;
            if self.trees[point.y][point.x] <= self.trees[point.y][x] {
                break;
            }
        }

        let mut visible_trees_above: usize = 0;
        for y in (0..point.y).rev() {
            visible_trees_above += 1;
            if self.trees[point.y][point.x] <= self.trees[y][point.x] {
                break;
            }
        }

        let mut visible_trees_below: usize = 0;
        for y in (point.y + 1)..self.height() {
            visible_trees_below += 1;
            if self.trees[point.y][point.x] <= self.trees[y][point.x] {
                break;
            }
        }

        visible_trees_to_the_left * visible_trees_to_the_right * visible_trees_above * visible_trees_below
    }

    fn len(&self) -> usize {
        self.trees.get(0).unwrap().len()
    }

    fn height(&self) -> usize {
        self.trees.len()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y:usize) -> Self {
        Point {
            x,
            y
        }
    }
}
