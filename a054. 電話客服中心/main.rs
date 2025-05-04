use std::collections::HashMap;
use std::io;

fn main() {
    // 建立字母與對應代碼的對照表
    let letter_codes: HashMap<char, u32> = [
        ('A', 10),
        ('B', 11),
        ('C', 12),
        ('D', 13),
        ('E', 14),
        ('F', 15),
        ('G', 16),
        ('H', 17),
        ('I', 34),
        ('J', 18),
        ('K', 19),
        ('L', 20),
        ('M', 21),
        ('N', 22),
        ('O', 35),
        ('P', 23),
        ('Q', 24),
        ('R', 25),
        ('S', 26),
        ('T', 27),
        ('U', 28),
        ('V', 29),
        ('W', 32),
        ('X', 30),
        ('Y', 31),
        ('Z', 33),
    ]
    .iter()
    .cloned()
    .collect();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("無法讀取輸入");
    let input = input.trim();

    if input.len() != 9 || !input.chars().all(|c| c.is_digit(10)) {
        println!("輸入無效，請輸入9位數字。");
        return;
    }

    let last_9_digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // 尋找可能的首位字母
    let mut possible_letters = Vec::new();
    for (letter, code) in &letter_codes {
        let first_digit = code / 10;
        let second_digit = code % 10;

        // 根據公式計算總和
        let mut sum = first_digit * 1 + second_digit * 9;
        for i in 0..8 {
            sum += last_9_digits[i] * (8 - i as u32);
        }

        // 計算檢查碼
        let m = sum % 10;
        let calculated_check_digit = if m == 0 { 0 } else { 10 - m };

        if calculated_check_digit == last_9_digits[8] {
            possible_letters.push(*letter);
        }
    }

    // 將可能的字母按字母順序排序
    possible_letters.sort();

    // 輸出結果
    if possible_letters.is_empty() {
        println!("找不到可能的第一個字母。");
    } else {
        for letter in possible_letters {
            print!("{}", letter);
        }
        println!();
    }
}

//此程式碼只可初步判斷前面字母，對於更嚴格的檢查無法適用