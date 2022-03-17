use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut aim: i64 = 0;
    let mut hoz: i64 = 0;
    let mut depth: i64 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let vec: Vec<&str> = ip.split(" ").collect();
                if vec[0].eq_ignore_ascii_case("forward") {
                    let hpos: i64 = vec[1].parse::<i64>().unwrap();
                    hoz += hpos;
                    depth += aim * hpos;
                }
                if vec[0].eq_ignore_ascii_case("down") {
                    let depth: i64 = vec[1].parse::<i64>().unwrap();
                    aim += depth;
                }
                if vec[0].eq_ignore_ascii_case("up") {
                    let depth: i64 = vec[1].parse::<i64>().unwrap();
                    aim -= depth;
                }
            }
        }
    }
    println!("{}", hoz * depth);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
