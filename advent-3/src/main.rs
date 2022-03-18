use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const N_LEN: usize = 12;
const INPUT: &str = "input.txt";
fn main() {
    let mut mc_bits_row: [i64; N_LEN] = [0; N_LEN];
    let lines: Vec<String> = read_lines();
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => mc_bits_row[i] += 1,
                '0' => mc_bits_row[i] -= 1,
                _ => continue,
            }
        }
    }
    let mut gamma: i64 = 0;
    let mut eps: i64 = 0;
    let rt: (i32, i32) = get_ogr(lines);
    let base: i64 = 2;
    for i in 0..(mc_bits_row.len()) {
        if mc_bits_row[i] >= 1 {
            gamma += base.pow((mc_bits_row.len() - i - 1) as u32);
        }
        if mc_bits_row[i] < 0 {
            eps += base.pow((mc_bits_row.len() - i - 1) as u32);
        }
    }
    println!("Gamma: {gamma}\nEps: {eps}\nOgr: {}\nCsr: {}", rt.0, rt.1);
    println!("Power consumption: {}", gamma * eps);
    println!("Life support rating: {}", rt.0 * rt.1);
}
fn get_ogr(_lines: Vec<String>) -> (i32, i32) {
    let mut ogr: i32 = 0;
    let mut csr: i32 = 0;
    let mut lines: Vec<String> = _lines.clone();
    for i in 0..N_LEN {
        let mut mc: i64 = 0;
        for line in lines.iter() {
            if line.chars().nth(i).unwrap() == '1' {
                mc += 1
            } else if line.chars().nth(i).unwrap() == '0' {
                mc -= 1
            };
        }
        lines.retain(|line| {
            if mc == 0 {
                line.chars().nth(i).unwrap() == '1'
            } else {
                line.chars().nth(i).unwrap() == char::from_digit((mc > 0) as u32, 10).unwrap()
            }
        });
        if lines.len() == 1{
        for line in lines.iter() {
            ogr =  i32::from_str_radix(line, 2).expect("Invalid");
        }}
    }
    lines = _lines.clone();
    for i in 0..N_LEN {
        let mut mc: i64 = 0;
        for line in lines.iter() {
            if line.chars().nth(i).unwrap() == '1' {
                mc -= 1
            } else if line.chars().nth(i).unwrap() == '0' {
                mc += 1
            };
        }
        lines.retain(|line| {
            if mc == 0 {
                line.chars().nth(i).unwrap() == '0'
            } else {
                line.chars().nth(i).unwrap() == char::from_digit((mc > 0) as u32, 10).unwrap()
            }
        });
        if lines.len() == 1{
        for line in lines.iter() {
            csr =  i32::from_str_radix(line, 2).expect("Invalid");
        }}
    }
    return (ogr, csr);
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
