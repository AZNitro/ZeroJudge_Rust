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
match s {
    0 => println!("普通"),
    1 => println!("吉"),
    2 => println!("大吉"),
    _ => unreachable!(),
}
}
}



//更完善的寫法

use std::io;

enum Fortune {
    Normal,
    Good,
    VeryGood
}

impl Fortune {
    fn from_number(n: u8) -> Fortune {
        match n {
            0 => Fortune::Normal,
            1 => Fortune::Good,
            2 => Fortune::VeryGood,
            _ => unreachable!()
        }
    }
    
    fn to_string(&self) -> &str {
        match self {
            Fortune::Normal => "普通",
            Fortune::Good => "吉",
            Fortune::VeryGood => "大吉"
        }
    }
}

fn is_valid_date(m: u8, d: u8) -> bool {
    if m == 0 || m > 12 || d == 0 {
        return false;
    }
    
    let days_in_month = match m {
        2 => 29,
        4 | 6 | 9 | 11 => 30,
        _ => 31
    };
    
    d <= days_in_month
}

fn calculate_fortune(m: u8, d: u8) -> Fortune {
    Fortune::from_number((m * 2 + d) % 3)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let numbers: Vec<u8> = match input
        .split_whitespace()
        .map(|s| s.parse::<u8>())
        .collect() {
            Ok(nums) if nums.len() == 2 => nums,
            _ => {
                println!("請輸入兩個有效的數字");
                return Ok(());
            }
        };
    
    let (m, d) = (numbers[0], numbers[1]);
    
    if !is_valid_date(m, d) {
        println!("日期無效");
        return Ok(());
    }
    
    let fortune = calculate_fortune(m, d);
    println!("{}", fortune.to_string());
    
    Ok(())
}
