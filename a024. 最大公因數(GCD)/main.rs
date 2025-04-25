use std::io;

/// 計算兩個整數的最大公因數（GCD）
/// 使用歐幾里得演算法
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if nums.len() != 2 {
        println!("請正確輸入兩個整數！");
        return;
    }

    let result = gcd(nums[0], nums[1]);
    println!("{}", result);
}