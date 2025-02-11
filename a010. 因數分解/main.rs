use std::io;
fn main(){
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let mut num: i32 = num.trim().parse().expect("Please type a number!");   
    //前三段確保輸入的是數字並轉換成i32型態
    
    let mut divisor = 2; 
    let mut factors = Vec::new();
    //設除數以及陣列存放因數


    while num > 1 {
        if num % divisor == 0 {
            factors.push(divisor);
            num = num / divisor;
        } else {
            divisor += 1;
        }
    }
    //進行分解 流程大致如下先用2除 如果可以整除就存入因數陣列並將num除以該因數 如果不能整除就換下一個數字

    let mut count = 1; //計算因數出現次數
    let mut current = factors[0]; //紀錄目前因數
    print!("{}", current); //印出第一個因數
    
    for i in 1..factors.len() { //從第二個因數開始
        if factors[i] == current { //如果與目前因數相同
            count += 1; //次數加一
        } else {
            if count > 1 { //如果次數大於1
                print!("^{}", count); //印出次數
            }
            print!(" * {}", factors[i]); //印出因數
            current = factors[i]; //更新目前因數
            count = 1; //次數重置
        }
    }
    if count > 1 {
        print!("^{}", count); //印出最後一個因數的次數
    }
}


