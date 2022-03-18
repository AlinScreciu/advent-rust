use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const INPUT: &str = "input.txt";
#[derive(Clone, Debug)]
struct Number {
    marked: bool,
    val: u128,
}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.val, self.marked as i8)
    }
}
fn main() {
    let mut lines: Vec<String> = read_lines();
    let moves: Vec<u128> = lines
        .remove(0)
        .split(",")
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    let mut boards: Vec<Vec<Vec<Number>>> = Vec::new();
    let mut board: Vec<Vec<Number>> = Vec::new();

    for line in lines.iter() {
        if line == "" {
            continue;
        };

        let row: Vec<Number> = line
            .split_whitespace()
            .map(|s: &str| Number {
                marked: false,
                val: s.parse::<u128>().unwrap(),
            })
            .collect();
        board.push(row.clone());
        if board.len() == 5 {
            boards.push(board.clone());
            board.clear();
            continue;
        }
    }
    'outer: for (c, call) in moves.iter().enumerate() {
        for b in 0..boards.len() {
            let (mut cm, mut rm): (bool, bool) = (true, true);
            for i in 0..5 {
                for j in 0..5 {
                    if boards[b][i][j].val == *call {
                        boards[b][i][j].marked = true;
                    }
                    if boards[b][j][i].val == *call {
                        boards[b][j][i].marked = true;
                    }
                    if !boards[b][i][j].marked {
                        cm = false;
                    }
                    if !boards[b][j][i].marked {
                        rm = false;
                    }
                }
                if rm || cm {
                    println!("Won on call {c}:{call} on board {}", b + 1);
                    break 'outer;
                }
            }
            
            
        }
        
    }
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
