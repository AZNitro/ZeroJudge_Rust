use num_bigint::BigInt; // 引入大整數庫，用於處理超大數字的計算
use std::io::{self, BufRead}; // 引入標準輸入輸出模組和緩衝讀取特性
use std::str::FromStr; // 引入從字串解析的特性

fn main() { 
    let stdin = io::stdin(); 
    let mut lines = stdin.lock().lines(); 
    
    if let Some(Ok(line)) = lines.next() { 
        let parts: Vec<&str> = line.trim().split_whitespace().collect(); // 將輸入行按空白分割成部分並收集到向量中
        
        if parts.len() == 3 { // 檢查是否有三個部分（第一個數字、運算符、第二個數字）
            let a = BigInt::from_str(parts[0]).expect("Invalid first number"); // 將第一個部分解析為大整數
            let op = parts[1]; // 取得運算符
            let b = BigInt::from_str(parts[2]).expect("Invalid second number"); // 將第三個部分解析為大整數
            
            let result = match op { // 根據運算符進行相應的數學運算
                "+" => a + b, // 加法運算
                "-" => a - b, // 減法運算
                "*" => a * b, // 乘法運算
                "/" => a / b, // 除法運算（取商）
                _ => panic!("Unsupported operation: {}", op), // 如果是不支持的運算符則拋出錯誤
            };
            
            println!("{}", result); // 輸出計算結果
        } else {
            println!("Invalid input format. Expected: number operation number"); 
        }
    }
}