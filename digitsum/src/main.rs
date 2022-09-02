#[macro_use]
extern crate text_io;

fn main() {
    println!("Give me a number. I will do the sum of all digits of the number.");
    let num: i32 = read!();
    let numC = num, mut sum = 0;
    while numC != 0 {
        sum += numC%10;
        numC /= 10;
    }
    println!({}, sum);
}