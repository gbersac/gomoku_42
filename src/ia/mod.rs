extern crate std;

mod decision;
mod heuristic;
mod turn;
#[cfg(test)]
mod test_decision;
#[cfg(test)]
mod test_move_to_evaluate;
mod move_to_evaluate;

pub use self::decision::{Decision};
pub use self::heuristic::heuristic;

pub const INFINITE: i32 = std::i32::MAX;
pub const NINFINITE: i32 = std::i32::MIN;

/// Return the opposite of the value. Special case for infinity.
pub fn neg_infinite(value: i32) -> i32 {
    if value == INFINITE {
    	NINFINITE
    } else if value == NINFINITE {
    	INFINITE
    } else {
        -value
    }
}
