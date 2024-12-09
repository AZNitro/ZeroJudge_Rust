use std::io;
fn main(){

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Error");
    
    let mut output = input.split_whitespace();
    let m :u8 = output
    .next().expect("請確保輸入兩個值")
    .parse().expect("請確保輸入為數字");
    let d :u8 = output
    .next().expect("請確保輸入兩個值")
    .parse().expect("請確保輸入為數字");
    
    if m>12 || d>31
    {
        println!("你輸入的月份或是日期錯誤")
    }
    else{
        
    let s :u8 =(m*2+d)%3;
    
    if s==0{
        println!("普通")
    }
    else if s==1
    {
        println!("吉")
    }
    else
    {
        println!("大吉")
    }
}
    

}