use std::io;
fn main() {
    let mut words = String::new();
    io::stdin()
    .read_line(&mut words)
    .expect("Error");
    println!("Hello, {}",words);
}