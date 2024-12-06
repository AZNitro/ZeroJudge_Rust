use std::io;
fn main(){
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("讀取該行失敗");
    
    let mut s = input.split_whitespace();
    let a : i32 = s
    .next().expect("請確保輸入兩個值")
    .parse().expect("請確保輸入為數字");
    let b : i32 = s
    .next().expect("請確保輸入兩個值")
    .parse().expect("請確保輸入為數字");
    
    println!("{}",a+b);
    
}

//不同解　
//使用Vec 收集數字
use std::io;

fn main() {
    // 創建一個新的空字串來存儲輸入
    let mut input = String::new();
    
    // 從標準輸入讀取一行
    io::stdin().read_line(&mut input).unwrap();
    
    // 使用函數式編程的方式處理輸入：
    let sum: i32 = input
        .split_whitespace()  // 將輸入字串按空白分割
        .map(|x| x.parse::<i32>().unwrap())  // 將每個子字串轉換為 i32
        .sum();  // 對所有數字求和
    
    // 輸出結果
    println!("{}", sum);
}

//
