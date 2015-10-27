use ia;

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

    /// Return the initial value of the
    pub fn init(&self) -> i32 {
        match *self {
            Turn::Player => ia::INFINITE,
            Turn::Adversary => -ia::INFINITE,
        }
    }

    ///Return true if the Turn is looking to minimise the heuristic
    pub fn is_min(&self) -> bool {
    	*self == Turn::Adversary
    }
}
