use std::io;

fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    let mut splited_string = input_string.split_whitespace();
    let first_number: u32 = splited_string.next().unwrap().parse().unwrap();
    let second_number: u32 = splited_string.next().unwrap().parse().unwrap();
    println!("{}", first_number + second_number);
}
