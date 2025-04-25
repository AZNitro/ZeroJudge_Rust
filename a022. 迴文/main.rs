use std::io::{self, BufRead};

/// 檢查給定字串是否為回文（palindrome）
/// 會先去除前後空白，再逐字元比較首尾
fn is_palindrome(s: &str) -> bool {
    let s = s.trim(); // 去除前後空白
    let chars: Vec<char> = s.chars().collect(); // 將字串轉為字元向量
    let len = chars.len();
    
    // 比較首尾字元，直到中間
    for i in 0..(len / 2) {
        if chars[i] != chars[len - i - 1] {
            return false; // 若有不同則不是回文
        }
    }
    true // 全部相同則為回文
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // 判斷是否為回文並輸出結果
    if is_palindrome(&input) {
        println!("yes");
    } else {
        println!("no");
    }
}