use std::io;

fn main() {

        loop{
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("讀取輸入失敗");
            
        match input.trim().parse::<u16>() {
            Ok(year) => {
                if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                    println!("閏年");
                } else {
                    println!("平年");
                }
            },
            Err(_) => {
                println!("請輸入有效的年份！");
        
            }    
            
        }
            
    }
}