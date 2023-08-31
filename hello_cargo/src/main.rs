use std::io;

fn main() {
    let n = vec![1, 2, 3];
    let y = plus_one(n);
    println!("The value of y is: {:?}", n);
}

fn plus_one(y: Vec<i32>) -> i32 {
    println!("{:?}", y);
    1
}