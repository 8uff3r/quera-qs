use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error in input");
    let input_number = input
        .trim_end()
        .parse::<i32>()
        .expect("Input is not a valid integer");

    let mut sum = 0;
    for i in 1..=input_number {
        sum += i;
        if i == 1 {
            print!("{i}");
            continue;
        }
        print!(" + {i}")
    }
    print!(" = {sum}")
}
