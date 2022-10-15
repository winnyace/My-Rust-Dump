use text_io::read;

fn main() {
    println!("Give me a number. I will do the sum of all digits of the number.");
    let mut num: i32 = read!(); let mut sum = 0;
    while num != 0 {
        sum += num%10;
        num /= 10;
    }
    println!("{}", sum);
}
