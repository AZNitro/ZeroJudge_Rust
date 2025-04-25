// 使用Shunting Yard演算法將中序表達式轉換為後序表達式，然後進行計算
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Token {
    Num(i64),    // 存儲數值型別
    Op(char),    // 存儲運算符：+, -, *, /, %
    LParen,      // 左括號
    RParen,      // 右括號
}

fn tokenize(line: &str) -> Vec<Token> {
    // 將輸入字串按空白分割並轉換為標記（Token）序列
    // 例如："3 + 4 * ( 2 - 1 )" 會轉換為
    // [Num(3), Op(+), Num(4), Op(*), LParen, Num(2), Op(-), Num(1), RParen]
    line.split_whitespace()
        .map(|s| match s {
            // 運算符
            "+" | "-" | "*" | "/" | "%" => Token::Op(s.chars().next().unwrap()),
            "(" => Token::LParen,
            ")" => Token::RParen,
            num => Token::Num(num.parse().unwrap()),
        })
        .collect()
}

// 演算法: Shunting Yard 轉後序，再計算
// Shunting Yard是由E. W. Dijkstra發明的演算法，用於將中序表達式轉換為後序表達式
// 例如: "3 + 4 * 2" -> "3 4 2 * +"
fn to_postfix(tokens: &[Token]) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();  // 輸出佇列，用於存儲後序表達式
    let mut stack = Vec::new();   // 運算符堆疊，用於臨時存儲運算符和左括號
    const MAX_STACK_SIZE: usize = 1000;  // 設定堆疊大小上限，防止無窮遞迴或惡意輸入

    // 內部函數：決定運算符優先級
    // 乘除取餘(* / %)的優先級高於加減(+ -)
    fn precedence(op: char) -> i32 {
        match op {
            '+' | '-' => 1,  // 加減運算優先級為1
            '*' | '/' | '%' => 2,  // 乘除取餘運算優先級為2
            _ => 0,          // 其他字符優先級為0
        }
    }


    for token in tokens {
        // 檢查堆疊是否過大，防止惡意輸入或循環引起的堆疊溢位
        if stack.len() > MAX_STACK_SIZE {
            return Err("表達式過於複雜，超出堆疊大小限制".to_string());
        }
        
        match token {
            // 數字直接放入輸出佇列
            Token::Num(_) => output.push(token.clone()),
            
            // 處理運算符
            Token::Op(op1) => {
                // 檢查堆疊頂部的運算符，如果優先級大於或等於當前運算符，
                // 則將堆疊頂部運算符彈出並加入輸出佇列
                while let Some(Token::Op(op2)) = stack.last() {
                    if precedence(*op2) >= precedence(*op1) {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                // 將當前運算符壓入堆疊
                stack.push(token.clone());
            }
            
            // 左括號直接壓入堆疊
            Token::LParen => stack.push(token.clone()),
            
            // 處理右括號：彈出堆疊中的運算符直到遇到左括號
            Token::RParen => {
                // 循環處理，直到找到匹配的左括號
                while let Some(top) = stack.pop() {
                    // 如果找到左括號，則結束循環
                    if let Token::LParen = top {
                        break;
                    } else {
                        // 否則將彈出的運算符加入輸出佇列
                        output.push(top);
                    }
                }
            }
        }
    }
    // 將堆疊中剩餘的運算符全部彈出並加入輸出佇列
    // 正確的表達式在處理完所有標記後，堆疊中不應有左括號或右括號
    while let Some(tok) = stack.pop() {
        output.push(tok);
    }
    
    // 返回後序表達式的標記序列
    Ok(output)
}

fn eval_postfix(tokens: &[Token]) -> i64 {
    // 計算後序表達式的結果
    // 使用堆疊來暫存計算過程中的數值
    let mut stack = Vec::new();
    
    // 逐一處理後序表達式中的標記
    for token in tokens {
        match token {
            // 遇到數字，直接入堆疊
            Token::Num(n) => stack.push(*n),
            
            // 遇到運算符，從堆疊中彈出兩個數進行運算，然後將結果壓回堆疊
            Token::Op(op) => {
                // 彈出兩個操作數，注意順序：後彈出的是第二個操作數
                let b = stack.pop().unwrap();  // 第二個操作數
                let a = stack.pop().unwrap();  // 第一個操作數
                
                // 根據運算符執行相應的運算
                let res = match op {
                    '+' => a + b,  // 加法
                    '-' => a - b,  // 減法（注意順序）
                    '*' => a * b,  // 乘法
                    '/' => a / b,  // 除法（注意可能的除零錯誤）
                    '%' => a % b,  // 取餘（同樣注意可能的除零錯誤）
                    _ => unreachable!(),  // 不應到達這裡，因為所有操作符都已處理
                };
                
                // 將計算結果壓入堆疊
                stack.push(res);
            }
            // 忽略其他標記（括號已在轉換為後序表達式時處理，此處不應出現）
            _ => {}
        }
    }
    
    // 計算完成後，堆疊中應只剩一個元素，即為表達式的最終結果
    stack[0]
}

fn main() {

    
    // 獲取標準輸入的引用
    let stdin = io::stdin();
    
    // 逐行處理輸入
    for line in stdin.lock().lines() {
        // 解包行內容，如果讀取失敗則會觸發panic
        let line = line.unwrap();
        
        // 忽略空行
        if line.trim().is_empty() {
            continue;
        }
        
        // 步驟1: 標記化 - 將輸入字串轉換為標記序列
        let tokens = tokenize(&line);
        
        // 步驟2: 轉換 - 使用Shunting Yard演算法將中序表達式轉換為後序表達式
        // unwrap()會在轉換失敗時觸發panic
        let postfix = to_postfix(&tokens).unwrap();
        
        // 步驟3: 計算 - 計算後序表達式的結果
        let result = eval_postfix(&postfix);
        
        // 步驟4: 輸出結果
        println!("{}", result);
    }
    

}