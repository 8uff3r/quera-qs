pub mod p275795;

pub fn get_problem_solution_fn(id: usize) -> Option<Box<dyn Fn()>> {
    match id {
        275795 => Some(Box::new(p275795::main)),
        _ => None,
    }
}
