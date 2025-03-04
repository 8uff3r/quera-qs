use std::{env, io};

mod problems;

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect arguments into a vector

    println!("Input the id of the problem:");
    let mut input = String::new();
    if args.len() > 1 {
        input = args.clone()[1].to_string();
    } else {
        io::stdin().read_line(&mut input).expect("Failed");
    }
    let input_number = input.trim_end().parse::<usize>().expect("");
    let solution = problems::get_problem_solution_fn(input_number);
    if let Some(func) = solution {
        println!("Solution:\n");
        (*func)()
    }
}
