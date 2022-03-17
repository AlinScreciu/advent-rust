use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut depths: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let depth = ip.parse::<i32>().unwrap();
                depths.push(depth);
            }
        }
    }
    let mut increments: i32 = 0;
    let mut prev_sum: i32 = 0;
    for i in 1..depths.len() - 3 {
        let sum = depths[i] + depths[i + 1] + depths[i + 2];
        if sum > prev_sum {
            increments += 1;
        }
        prev_sum = sum;
    }
    println!("{}", increments);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
