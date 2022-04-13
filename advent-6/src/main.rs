use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const INPUT: &str = "input.txt";

fn main() {
    let lines: Vec<String> = read_lines();
    let mut fishes: Vec<i32> = Vec::new();
    for line in lines.iter() {
        for fish in line.split(',') {
            fishes.push(fish.parse::<i32>().unwrap());
        }
    }
    for _i in 0..180 {
        for i in 0..fishes.len() {
            if fishes[i] == 0 {
                fishes.push(8);
                fishes[i] = 6;
            } else {
                fishes[i] -= 1;
            }
        }
    }
    println!("{}", fishes.len());
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
/*
180 -> 20
190 -> 40
200 -> 80
210 -> 160
220 -> 320
230 -> 640
240 -> 1280
250 -> 2560
256 -> ~3000
*/