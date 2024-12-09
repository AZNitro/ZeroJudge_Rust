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

//ai建議完整解法
// 引入標準函式庫中的 io 模組，用於處理輸入輸出
use std::io;

// 定義一個函式來判斷是否為閏年
// 這樣可以讓主程式更簡潔，且方便重複使用這個功能
fn is_leap_year(year: u16) -> bool {
    // 閏年規則：
    // 1. 可被 4 整除，且不能被 100 整除
    // 2. 或是可被 400 整除
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn main() {
    println!("=== 閏年判斷程式 ===");
    println!("輸入 'q' 可退出程式");

    // 主程式迴圈
    loop {
        // 建立一個新的空字串來儲存使用者輸入
        let mut input = String::new();
        
        // 顯示提示訊息
        println!("\n請輸入要判斷的年份：");

        // 讀取使用者輸入
        // read_line 會將輸入存到 input 字串中
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // 去除輸入字串前後的空白字元
                let trimmed = input.trim();

                // 檢查是否要退出程式
                if trimmed.to_lowercase() == "q" {
                    println!("程式結束！");
                    break;
                }

                // 嘗試將輸入轉換為數字
                match trimmed.parse::<u16>() {
                    Ok(year) => {
                        // 使用 if let 配合函式來輸出結果
                        if is_leap_year(year) {
                            println!("{} 年是閏年！", year);
                            println!("因為：");
                            // 額外顯示判斷的詳細原因
                            if year % 400 == 0 {
                                println!("此年份能被 400 整除");
                            } else {
                                println!("此年份能被 4 整除，但不能被 100 整除");
                            }
                        } else {
                            println!("{} 年是平年！", year);
                            println!("因為：");
                            if year % 4 != 0 {
                                println!("此年份不能被 4 整除");
                            } else if year % 100 == 0 {
                                println!("此年份能被 100 整除，但不能被 400 整除");
                            }
                        }
                    },
                    Err(_) => {
                        // 輸入無法轉換為數字時的錯誤處理
                        println!("錯誤：請輸入有效的年份數字（1-65535）或 'q' 來退出");
                        continue;
                    }
                }
            },
            Err(error) => {
                // 處理輸入讀取錯誤
                println!("發生錯誤：{}", error);
                continue;
            }
        }
    }
}
