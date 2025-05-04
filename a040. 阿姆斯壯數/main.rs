use std::io;

fn is_armstrong_number(num: u32) -> bool {
    let str_num = num.to_string();
    let num_digits = str_num.len() as u32;
    
    let sum: u32 = str_num
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_digits))
        .sum();
    
    sum == num
}

fn main() {
    // 讀取輸入
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("無法讀取輸入");
    

    let bounds: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("無效的數字"))
        .collect();
    
    if bounds.len() != 2 {
        panic!("輸入應包含恰好兩個數字");
    }
    
    let start = bounds[0];
    let end = bounds[1];
    
    // 尋找範圍內的 Armstrong 數字
    let armstrong_numbers: Vec<u32> = (start..=end)
        .filter(|&num| is_armstrong_number(num))
        .collect();
    
    // 輸出結果
    for (i, num) in armstrong_numbers.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", num);
    }
    println!();
}