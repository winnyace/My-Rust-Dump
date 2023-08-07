use text_io::read;

fn leftpad(input: String, len: i32, ch: char) -> String {
    let mut output = String::from("");
    for _n in 1..len {
        output.push(ch);
    }
    output.push_str(&input);
    return output;
}

fn main() {
    println!("Give me a string and a number:");
    let input: String = read!("{}\n");
    let num: i32 = read!("{}\n");
    println!("New string: {}", leftpad(input, num, ' '));
}
