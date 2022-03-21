use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const INPUT: &str = "input.txt";

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn make_point(input: &str) -> Point {
    let mut split = input.trim().split(",");
    Point {
        x: split.next().unwrap().parse::<i64>().unwrap(),
        y: split.next().unwrap().parse::<i64>().unwrap(),
    }
}
fn main() {
    let lines: Vec<String> = read_lines();
    let mut points: Vec<Point> = read_points(lines);
    points.sort_by(|p1, p2| {
        if p1.y == p2.y {
            p1.x.cmp(&p2.x)
        } else {
            p1.y.cmp(&p2.y)
        }
    });
    let mut pointset: HashMap<Point, Vec<Point>> = HashMap::new();
    for point in points.iter() {
        pointset.entry(*point).or_insert(vec![]).push(*point);
    }
    let mut dups: i64 = 0;
    for (_, v) in &pointset {
        if v.len() > 1 {
            dups += 1;
        }
    }
    println!("There are {} overlaps", dups);
}

fn read_points(lines: Vec<String>) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    for line in lines.iter() {
        let mut split = line.trim().split("->");
        let p1 = make_point(split.next().unwrap());
        let p2 = make_point(split.next().unwrap());
        let res: Option<Vec<Point>> = make_segment(p1, p2);
        if res.is_some() {
            points.append(&mut res.unwrap());
        }
    }
    return points;
}

fn make_segment(p1: Point, p2: Point) -> Option<Vec<Point>> {
    let mut points: Vec<Point> = Vec::new();
    let max_x: i64 = cmp::max::<i64>(p1.x, p2.x);
    let min_x: i64 = cmp::min::<i64>(p1.x, p2.x);
    let max_y: i64 = cmp::max::<i64>(p1.y, p2.y);
    let min_y: i64 = cmp::min::<i64>(p1.y, p2.y);
    if p1.y == p2.y {
        for i in min_x..(max_x + 1) {
            points.push(Point { x: i, y: p1.y });
        }
    } else if p1.x == p2.x {
        for i in min_y..(max_y + 1) {
            points.push(Point { x: p1.x, y: i });
        }
    } else {
        let gradient: f32 = ((p2.y - p1.y) as f32) / ((p2.x - p1.x) as f32);
        
        for i in min_x..(max_x+1) {
            points.push(point_gen(p1, gradient, i).clone());
        }
    }

    return Some(points);
}
fn point_gen(p: Point, m: f32, x: i64) -> Point {
    let y: i64 = ((p.y as f32) + m * ((x - p.x) as f32)) as i64;
    Point { x: (x), y: (y) }
}
fn load_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn read_lines() -> Vec<String> {
    let mut linev: Vec<String> = Vec::new();
    if let Ok(lines) = load_lines(INPUT) {
        for line in lines {
            if let Ok(ip) = line {
                linev.push(ip);
            }
        }
    }
    return linev;
}
