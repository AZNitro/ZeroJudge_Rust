use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if let Ok(num) = s.trim().parse::<i32>() {
                println!("{:b}", num);
            }
        }
    }
}