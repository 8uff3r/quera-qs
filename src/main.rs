use std::io;

mod problems;

fn main() {
    println!("Input the id of the problem:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let input_number = input.trim_end().parse::<usize>().expect("");
    let solution = problems::get_problem_solution_fn(input_number);
    if let Some(func) = solution {
        println!("Solution:\n");
        (*func)()
    }
}
