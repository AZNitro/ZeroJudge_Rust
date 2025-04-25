use std::collections::HashMap;
use std::io::{self, Write};

/// 建立英文字母對應數字的對照表
fn get_letter_map() -> HashMap<char, u8> {
    [
        // 字母與對應的數字
        ('A', 10), ('B', 11), ('C', 12), ('D', 13), ('E', 14), ('F', 15), ('G', 16), ('H', 17),
        ('I', 34), ('J', 18), ('K', 19), ('L', 20), ('M', 21), ('N', 22), ('O', 35), ('P', 23),
        ('Q', 24), ('R', 25), ('S', 26), ('T', 27), ('U', 28), ('V', 29), ('W', 32), ('X', 30),
        ('Y', 31), ('Z', 33),
    ]
    .iter()
    .cloned()
    .collect()
}

/// 判斷身分證字號是否合法
fn is_real_id(id: &str) -> bool {
    // 檢查長度是否為10
    if id.len() != 10 {
        return false;
    }
    let letter_map = get_letter_map();
    // 取出第一個字母
    let first = id.chars().next().unwrap();
    // 取出後9碼數字
    let nums: Vec<u8> = id[1..]
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(255) as u8)
        .collect();
    // 檢查是否都是數字且長度正確
    if nums.len() != 9 || nums.iter().any(|&n| n > 9) {
        return false;
    }
    // 查詢字母對應的數字
    let code = match letter_map.get(&first) {
        Some(&v) => v,
        None => return false,
    };
    // 十位數與個位數
    let n1 = code / 10;
    let n2 = code % 10;
    // 計算加權總和（不包含最後一碼）
    let mut sum = n1 + n2 * 9;
    let weights = [8, 7, 6, 5, 4, 3, 2, 1];
    for (i, w) in weights.iter().enumerate() {
        sum += nums[i] * w;
    }
    // 檢查碼驗證
    let check_digit = nums[8];
    let remainder = sum % 10;
    remainder == 0 && check_digit == 0 || 10 - remainder == check_digit
}

fn main() {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let id = input.trim().to_uppercase(); 
    if is_real_id(&id) {
        println!("real");
    } else {
        println!("fake");
    }
}