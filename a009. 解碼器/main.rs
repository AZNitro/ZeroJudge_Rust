use std::io;
fn input() -> String{
    let mut string = String::new();
    io::stdin()
    .read_line(&mut string)
    .expect("在儲存字串時失敗");
    return string;
}
fn to_ascii(string: &str) -> String{
    let string = string.trim();
    string.chars()
    .map(|c| c as u32) // 將字元轉換為 u32 (ASCII 碼)
    .map(|ascii| ascii.to_string()) // 將 ASCII 碼轉換為字串
    .collect::<Vec<String>>() // 收集成字串向量
    .join(" ") // 使用空格連接字串
}
fn transfer_back(string: &str) -> String {
    string.split_whitespace()           // 分割空格
        .map(|s| s.parse::<u32>().expect("u32字串轉換出錯")) // 字串轉u32
        .map(|n| n.saturating_sub(7))   // 減去7
        .map(|n| char::from_u32(n).expect("char字串轉換出錯")) // u32轉char
        .collect::<String>()           // 收集成字串
}

fn main() {
    let user_input = input();
    let ascii_value=to_ascii(&user_input);
    let tras=transfer_back(&ascii_value);
   
    println!("asci code:{}",ascii_value);
    println!("轉換回來:{}",tras);
}
