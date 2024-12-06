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