use std::io;
fn main() {
    let mut words = String::new();
    io::stdin()
    .read_line(&mut words)
    .expect("Error");                //此處可使用unwrap(),以這題來說是足夠，但不建議
    println!("hello, {}",words);
}

//最佳解(AI)
use std::io;

fn main() -> io::Result<()> {
    // 創建可變的 String
    let mut input = String::new();
    
    // 使用 ? 運算符進行錯誤處理
    io::stdin().read_line(&mut input)?;
    
    // 輸出結果
    println!("hello, {}", input.trim());
    
    // 返回成功結果
    Ok(())
}
