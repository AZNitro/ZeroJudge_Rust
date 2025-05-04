use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("無法讀取輸入");
    
    // 將輸入轉換為數字
    let problems_solved: u32 = input.trim().parse().expect("請輸入有效的數字");
    

    let score = calculate_score(problems_solved);
    
    // 輸出結果
    println!("{}",score);
}

fn calculate_score(problems: u32) -> u32 {
    if problems <= 10 {
        // 答對題數在 0~10 者，每題給6分
        problems * 6
    } else if problems <= 20 {
        // 題數在 11~20 者，從第11題開始，每題給2分
        60 + (problems - 10) * 2
    } else if problems <= 40 {
        // 題數在 21~40 者，從第21題開始，每題給1分
        80 + (problems - 20)
    } else {
        // 題數在 40 以上者，一律100分
        100
    }
}