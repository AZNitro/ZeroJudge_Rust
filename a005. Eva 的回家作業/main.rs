use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    
    let int1: i32 = input1.trim().parse().unwrap();

    for _ in 0..int1 {
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();

        let mut input2 = input2.trim().split_whitespace();

        let a: i16 = input2.next().unwrap().parse().unwrap();
        let b: i16 = input2.next().unwrap().parse().unwrap();
        let c: i16 = input2.next().unwrap().parse().unwrap();
        let d: i16 = input2.next().unwrap().parse().unwrap();

        if b-a==d-c {
            println!("{}", d+(d-c));
        }
        else if  b/a==d/c {
            println!("{}", d*(d/c));
        }
        else{
            println!("{{Error}}");
        }
    }
}
