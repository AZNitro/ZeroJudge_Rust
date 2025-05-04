use std::io;

fn main() {
    // 建立字串儲存用戶輸入
    let mut input = String::new();
    // 讀取
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // 轉換為32位整數，並去除前後空白並轉換為無符號整數
    let num: u32 = input
        .trim()
        .parse()
        .expect("Failed to parse input as a number");

    // 轉換為字串並翻轉
    let reversed_str = num.to_string().chars().rev().collect::<String>();
    // 字串解析成整數
    let reversed: u32 = reversed_str
        .parse()
        .expect("Failed to parse reversed string as a number");

    println!("{}", reversed);
}
