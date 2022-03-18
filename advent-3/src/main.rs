use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut bits: [i8; 12] = [0; 12];
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                for i in 0..ip.len() {
                    match ip.to_string().chars().nth(i).unwrap() {
                        '1' => bits[i] += 1,
                        '0' => bits[i] -= 1,
                        _ => panic!("Invalid"),
                    };
                }
            }
        }
    }
    let mut gamma: i64 = 0;
    let mut eps: i64 = 0;
    let base: i64 = 2;
    for i in 0..(bits.len()) {
        if bits[i] > 1 {
            gamma += base.pow((bits.len() - i - 1) as u32);
        }
        if bits[i] <= 0 {
            eps += base.pow((bits.len() - i - 1) as u32);
        }
    }
    println!("Gamma: {gamma}\nEps: {eps}");
    println!("Power consumption: {}", gamma * eps);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
