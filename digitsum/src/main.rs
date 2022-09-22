#[macro_use]
extern crate text_io;

fn main() {
    println!("Give me a number. I will do the sum of all digits of the number.");
    let num: i32 = read!();
    let mut num_c = num; let mut sum = 0;
    while num_c != 0 {
        sum += num_c%10;
        num_c /= 10;
    }
    println!("{}", sum);
}