use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("無法讀取輸入");
    
    // 轉換輸入為數字
    let n: u32 = input.trim().parse()
        .expect("請輸入一個有效的正整數");
    
    // 計算最大區域數
    // 公式：R(n) = (n^3+5n+6)/6
    let regions = (n.pow(3) + 5 * n + 6) / 6;
    

    println!("{}",regions);
}