pub type SortFn = fn(
    acc: ((usize, usize), i32),
    item: &((usize, usize), i32)
) -> ((usize, usize), i32);

#[derive(Debug, PartialEq, Clone)]
pub enum Turn {
	/// Player is looking to maximise the value of the heuristic
	Player,
	/// Adversary is looking to minimise the value of the heuristic
	Adversary
}

impl Turn {
    pub fn other(&self) -> Turn {
        match *self {
            Turn::Player => Turn::Adversary,
            Turn::Adversary => Turn::Player,
        }
    }

    pub fn sign_alternation(&self) -> i32 {
        match *self {
            Turn::Player => -1,
            Turn::Adversary => 1,
        }
    }
}
