pub mod p275794;
pub mod p275795;

pub fn get_problem_solution_fn(id: usize) -> Option<Box<dyn Fn()>> {
    match id {
        275795 => Some(Box::new(p275795::main)),
        275794 => Some(Box::new(p275794::main)),
        _ => None,
    }
}
