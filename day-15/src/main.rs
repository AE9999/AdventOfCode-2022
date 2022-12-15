extern crate core;

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::ops::Range;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
        static ref RE_SENSOR: Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let sensor_reports = read_input(&args[1])?;

    println!("In the row where y=2000000 {:?} positions cannot contain a beacon",
             solve1(&sensor_reports));

    println!("{:?} is its tuning frequency",
             solve2(&sensor_reports));

    Ok(())
}

fn solve1(sensor_reports: &Vec<SensorReport>) -> i64 {
    let mut current_intersections : HashSet<Range<i64>> = HashSet::new();
    // let y :i64 = 10;
    let y :i64 = 2000000;
    for sensor_report in sensor_reports.iter() {
        let intersection = sensor_report.intersection(y);
        if intersection.is_some() {
            let mut subsumed_intersections : HashSet<Range<i64>> = HashSet::new();
            let mut new_intersection = intersection.unwrap();
            let mut subsumed = false;
            for current_intersection in current_intersections.iter() {
                if is_subsumed_by(&new_intersection, current_intersection) {
                    assert!(subsumed_intersections.is_empty(), "Inconsistent state");
                    subsumed = true;
                    break;

                } else if overlaps(&new_intersection, current_intersection) {
                    new_intersection = merge(current_intersection, &new_intersection);
                    subsumed_intersections.insert(current_intersection.clone());
                }
            }
            if !subsumed {
                current_intersections.insert(new_intersection);
            }
            for subsumed_intersection in subsumed_intersections.iter() {
                current_intersections.remove(subsumed_intersection);
            }
        }
    }

    let intersection_space: i64 =
        current_intersections.iter()
                         .map(|intersection| intersection.end - intersection.start)
                         .sum();

    let mut beacons_in_space: HashSet<Point> = HashSet::new();
    for sensor_report in (&sensor_reports).iter() {
        for current_intersection in &current_intersections {
            if sensor_report.closest_beacon_location.y == y
               && current_intersection.contains(&(sensor_report.closest_beacon_location.x)) {
                beacons_in_space.insert(sensor_report.closest_beacon_location.clone());
            }
        }
    }

    intersection_space - (beacons_in_space.len() as i64)
}

fn solve2(sensor_reports: &Vec<SensorReport>) -> i64 {
    // let search_space_lenght: i64 = 20;
    let search_space_lenght: i64 = 4000000;

    for y in 0..(search_space_lenght + 1) {
        println!("{:?} / {:?} ..", y, search_space_lenght);

        let forbidden_ranges_in_line = find_forbidden_ranges_in_line(y, sensor_reports);
        assert!(forbidden_ranges_in_line.len() > 0, "At least something should be forbidden");

        if forbidden_ranges_in_line[0].start > 0 {
            return ((forbidden_ranges_in_line[0].start - 1) * 4000000) + y;
        } else if forbidden_ranges_in_line[forbidden_ranges_in_line.len() - 1].end <  search_space_lenght + 1 {
            return ((forbidden_ranges_in_line[0].end) * 4000000) + y;
        }

        for i in 1..forbidden_ranges_in_line.len() {
            let l = &forbidden_ranges_in_line[i -1];
            let r = &forbidden_ranges_in_line[i];
            if l.end != r.start {
                assert_eq!(l.end + 1, r.start, "We are looking for a single point");
                return ((l.end) * 4000000) + y;
            }
        }


    }
    panic!("There should be one point which violates no constraints ..")
}

fn find_forbidden_ranges_in_line(y: i64, sensor_reports: &Vec<SensorReport>) -> Vec<Range<i64>> {
    let mut current_intersections : HashSet<Range<i64>> = HashSet::new();

    for sensor_report in sensor_reports.iter() {
        let intersection = sensor_report.intersection(y);
        if intersection.is_some() {
            let mut subsumed_intersections : HashSet<Range<i64>> = HashSet::new();
            let mut new_intersection = intersection.unwrap();
            let mut subsumed = false;
            for current_intersection in current_intersections.iter() {
                if is_subsumed_by(&new_intersection, current_intersection) {
                    assert!(subsumed_intersections.is_empty(), "Inconsistent state");
                    subsumed = true;
                    break;

                } else if overlaps(&new_intersection, current_intersection) {
                    new_intersection = merge(current_intersection, &new_intersection);
                    subsumed_intersections.insert(current_intersection.clone());
                }
            }
            if !subsumed {
                current_intersections.insert(new_intersection);
            }
            for subsumed_intersection in subsumed_intersections.iter() {
                current_intersections.remove(subsumed_intersection);
            }
        }
    }

    let mut ranges =
        current_intersections.iter()
                             .map(|range|range.clone())
                             .collect::<Vec<Range<i64>>>();
    ranges.sort_by(|a,b| a.start.cmp(&b.start));

    return ranges
}

fn is_subsumed_by(new_intersection : &Range<i64>, current_intersection : &Range<i64>) -> bool {
    return overlaps(new_intersection, current_intersection)
           && new_intersection.start >= current_intersection.start
           && new_intersection.end <= current_intersection.end
}

fn overlaps(l: &Range<i64>, r: &Range<i64>) -> bool {
    return r.contains(&(l.end -1))
           || r.contains(&(l.start))
           || l.contains(&(r.end -1))
           || l.contains(&(r.start))
}

fn merge(l: &Range<i64>, r: &Range<i64>) -> Range<i64> {
    assert!(overlaps(l,r));
    std::cmp::min(l.start, r.start)..std::cmp::max(l.end, r.end)
}

fn read_input(filename: &String) -> io::Result<Vec<SensorReport>> {
    let file_in = File::open(filename)?;

    let sensor_reports =
        BufReader::new(file_in)
                  .lines()
                  .map(|line|line.unwrap())
                  .map(|line|SensorReport::new(line))
                  .collect::<Vec<SensorReport>>();
    Ok(sensor_reports)
}

#[derive(Debug, Clone)]
struct SensorReport {
    sensor_location: Point,
    closest_beacon_location: Point,
}

impl SensorReport {
    fn new(line: String) -> Self {
        let mut cap = RE_SENSOR.captures_iter(line.as_str().trim());
        let cap  = cap.next().unwrap();
        SensorReport {
            sensor_location: Point {
                x: cap[1].parse::<i64>().unwrap(),
                y: cap[2].parse::<i64>().unwrap(),
            },
            closest_beacon_location: Point {
                x: cap[3].parse::<i64>().unwrap(),
                y: cap[4].parse::<i64>().unwrap(),
            }
        }
    }

    fn radius(&self) -> i64 {
        (self.closest_beacon_location.x - self.sensor_location.x).abs()
        + (self.closest_beacon_location.y - self.sensor_location.y).abs()
    }

    // Check for odd & even
    fn intersection(&self, y: i64) -> Option<Range<i64>> {
        let distance_from_line = (y - self.sensor_location.y).abs();
        if (self.radius() * 2 + 1) <= distance_from_line * 2 {
            None
        } else {
            let start = self.sensor_location.x - (self.radius() - distance_from_line);
            let  end = self.sensor_location.x + (self.radius() - distance_from_line) + 1; // Don't forget to count self
                                                                                             // end is non inclusive
            Some(start..end)
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}
