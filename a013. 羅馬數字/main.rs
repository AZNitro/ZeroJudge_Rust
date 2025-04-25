use std::io::{self, BufRead};

// 將羅馬數字字串轉換為整數
fn roman_to_int(s: &str) -> i32 {    // 將羅馬數字字串轉換為整數
    let mut total = 0;              // 儲存總和
    let mut prev = 0;               // 儲存前一個字元的數值
    // 反向遍歷字串中的每個字元
    for c in s.chars().rev() {
        let val = match c {         // 根據字元取得對應的數值
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        // 如果當前數值小於前一個數值，則減去，否則加上
        if val < prev {
            total -= val;
        } else {
            total += val;
        }
        prev = val;                // 更新前一個數值
    }
    total
}

// 將整數轉換為羅馬數字字串
fn int_to_roman(mut num: i32) -> String {  // 將整數轉換為羅馬數字字串
    // 定義羅馬數字與對應數值的對照表，包含減法規則
    let vals = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
    ];
    let mut res = String::new();   // 儲存結果字串
    // 依序從大到小比對，能減就減，並將對應羅馬數字加入結果
    for &(v, sym) in &vals {
        while num >= v {
            res.push_str(sym);
            num -= v;
        }
    }
    res
}

fn main() {
    let stdin = io::stdin();   // 取得標準輸入
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim() == "#" {   // 若遇到 # 則結束
            break;
        }
        let parts: Vec<&str> = line.trim().split_whitespace().collect(); // 以空白分割兩個羅馬數字
        if parts.len() != 2 { continue; } // 若不是兩個則跳過
        let a = roman_to_int(parts[0]);   // 轉換第一個羅馬數字為整數
        let b = roman_to_int(parts[1]);   // 轉換第二個羅馬數字為整數
        let diff = (a - b).abs();         // 計算兩數之差的絕對值
        if diff == 0 {
            println!("ZERO");             // 若差為 0 則輸出 ZERO
        } else {
            println!("{}", int_to_roman(diff)); // 否則輸出羅馬數字
        }
    }
}