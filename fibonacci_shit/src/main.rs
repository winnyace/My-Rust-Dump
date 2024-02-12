use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let mut x: i32 = input.trim().parse().expect("Not a number");
    let mut next_elem;
    let mut first_elem = 0;
    let mut second_elem = 1; 
    let mut res: Vec<i32> = vec![0, 1];
    while x-1 > 0 {
        next_elem = first_elem + second_elem;
        first_elem = second_elem;
        second_elem = next_elem;
        x = x - 1;
        res.push(next_elem);
    }
    for x in res.iter() {
        print!("{} ", x);
    }
}
