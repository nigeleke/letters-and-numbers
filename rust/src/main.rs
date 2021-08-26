use crate::resolver::Resolver;

mod expression;
mod operator;
mod resolver;
mod result;

fn main() {
    let solutions = Resolver::find_solutions(&[3, 5, 5, 6, 1, 75], 559);
    let no_solution = String::from("No solution!");
    let solution = solutions.get(0).unwrap_or(&no_solution);
    println!("{}", solution);
}
