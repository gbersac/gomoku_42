use ia;

pub type SortFn = fn(
    acc: ((usize, usize), i32),
    item: &((usize, usize), i32)
) -> ((usize, usize), i32);

fn min(
    acc: ((usize, usize), i32),
    item: &((usize, usize), i32)
) -> ((usize, usize), i32) {
    if acc.1 > item.1 {
        return *item;
    }
    acc
}

fn max(
    acc: ((usize, usize), i32),
    item: &((usize, usize), i32)
) -> ((usize, usize), i32) {
    if acc.1 < item.1 {
        return *item;
    }
    acc
}

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
            Turn::Player => -ia::INFINITE,
            Turn::Adversary => ia::INFINITE,
        }
    }

    ///Return true if the Turn is looking to minimise the heuristic
    pub fn is_min(&self) -> bool {
    	*self == Turn::Adversary
    }

    ///Return the function to sort elements according to the type of turn.
    pub fn sort_fn(&self) -> SortFn {
        match self.is_min() {
            true => min,
            false => max,
        }
    }

    pub fn default_result(&self) -> ((usize, usize), i32) {
        match self.is_min() {
            true => ((0, 0), ia::INFINITE),
            false => ((0, 0), -ia::INFINITE),
        }
    }

    pub fn sign_alternation(&self) -> i32 {
        match *self {
            Turn::Player => -1,
            Turn::Adversary => 1,
        }
    }
}
