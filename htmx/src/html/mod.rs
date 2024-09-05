mod actions;
mod base;
mod goal;
mod index;
mod number;
mod numbers;
mod solve_action;

pub use self::actions::actions;
pub use self::goal::update_goal;
pub use self::index::index;
pub use self::number::update_number;
pub use self::numbers::validate_numbers;
// pub use self::solve_action::get_solution;
// pub use self::solve_action::reveal_goal;
pub use self::solve_action::solve_action;
