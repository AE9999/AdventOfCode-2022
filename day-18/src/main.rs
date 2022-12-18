use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let points = read_input(&args[1])?;

    println!("{:?} is the surface area of your scanned lava droplet",
             solve1(&points));

    let problem = Problem::new(points);
    println!("{:?} is the surface area of your scanned lava droplet",
             problem.solve2());

    Ok(())
}

fn solve1(points: &Vec<Point>) -> usize {
    points.iter()
          .map(|point| {
              let seen_neighbours =
                  points.iter()
                      .map(|point2| if point.neighbours().contains(point2) { 1 } else { 0 })
                      .sum::<usize>();
                  6 - seen_neighbours
           })
          .sum::<usize>()
}

fn read_input(filename: &String) -> io::Result<Vec<Point>> {
    let file_in = File::open(filename)?;

    let points =
        BufReader::new(file_in)
            .lines()
            .map(|line|line.unwrap())
            .map(|line| line.split(',')
                                   .map(|number|number.parse::<i32>()
                                                            .unwrap())
                                   .collect::<Vec<i32>>())
            .map(|numbers| Point { x: numbers[0], y: numbers[1], z: numbers[2]})
            .collect::<Vec<Point>>();

    Ok(points)
}

struct Problem {
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
    minz: i32,
    maxz: i32,

    points: HashSet<Point>,
}

impl Problem {
    fn new(points: Vec<Point>) -> Self {

        let mut minx: i32 = i32::MAX;
        let mut maxx: i32 = i32::MIN;
        let mut miny: i32 = i32::MAX;
        let mut maxy: i32 = i32::MIN;
        let mut minz: i32 = i32::MAX;
        let mut maxz: i32 = i32::MIN;

        let mut points_set: HashSet<Point> = HashSet::new();

        for point in points {
            minx = min(minx, point.x);
            maxx = max(maxx, point.x);
            miny = min(miny, point.y);
            maxy = max(maxy, point.y);
            minz = min(minz, point.z);
            maxz = max(maxz, point.z);

            points_set.insert(point);
        }

        minx = minx - 1;
        maxx = maxx + 1;
        miny = miny - 1;
        maxy = maxy + 1;
        minz = minz - 1;
        maxz = maxz + 1;

        Problem {
            minx,
            maxx,
            miny,
            maxy,
            minz,
            maxz,
            points: points_set,
        }
    }

    fn create_flood_map(&self, point: Point, flood_map: &mut HashSet<Point> ) {
        flood_map.insert(point.clone());
        for neighbour in point.neighbours()  {
            if !flood_map.contains(&neighbour)
                && !self.points.contains(&neighbour)
                && neighbour.x >= self.minx
                && neighbour.x <= self.maxx
                && neighbour.y >= self.miny
                && neighbour.y <= self.maxy
                && neighbour.z >= self.minz
                && neighbour.z <= self.maxz {
                self.create_flood_map(neighbour, flood_map);
            }
        }
    }

    fn solve2(&self) -> usize {

        let mut flood_map: HashSet<Point> = HashSet::new();

        let starting_point = Point { x: self.maxx, y: self.maxy, z: self.maxz };

        self.create_flood_map(starting_point, &mut flood_map);

        self.points.iter()
                   .map(|point| {
                       point.neighbours()
                            .iter()
                            .map(|point| {
                                if !self.points.contains(point)
                                    && flood_map.contains(point) {
                                    1
                                } else {
                                    0
                                }
                             })
                            .sum::<usize>()
                   }).sum::<usize>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {

    fn dxdydz(&self, x: i32, y: i32, z: i32) -> Point {
        Point {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        vec!(self.dxdydz(1,0,0),
             self.dxdydz(-1,0,0),
             self.dxdydz(0,1,0),
             self.dxdydz(0,-1,0),
             self.dxdydz(0,0,1),
             self.dxdydz(0,0,-1),
        )
    }
}
