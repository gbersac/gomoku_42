extern crate std;

mod decision;
mod heuristic;
mod turn;
#[cfg(test)]
mod test_decision;
#[cfg(test)]
mod test_move_to_evaluate;
mod move_to_evaluate;

pub const INFINITE: i32 = std::i32::MAX;
pub use self::decision::{Decision};
pub use self::heuristic::heuristic;
