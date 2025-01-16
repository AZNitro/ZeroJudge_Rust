use std::io;

fn main() {
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();

        let mut input2 = input2.trim().split_whitespace();

        let a: f32 = input2.next().unwrap().parse().unwrap();
        let b: f32 = input2.next().unwrap().parse().unwrap();
        let c: f32 = input2.next().unwrap().parse().unwrap();

        let x1= (-b+(b*b-4.0*a*c).sqrt())/(2.0*a);
        let x2= (-b-(b*b-4.0*a*c).sqrt())/(2.0*a);
        

        if x1==x2{
            println!("Two same roots x={}",x1);
        }
        else if x2<x1{
            println!("Two different roots x1={} x2={}",x1,x2);
        }
        else 
        {
            println!("No real root");
        }
}
