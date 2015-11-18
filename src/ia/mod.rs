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

pub fn neg_infinite(inf: i32) -> i32 {
    if inf == INFINITE {
    	NINFINITE
    } else {
    	NINFINITE
    }
}
