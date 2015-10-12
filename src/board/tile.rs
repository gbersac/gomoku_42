use std::fmt::{Formatter, Display, Error};

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
	BLACK,
	WHITE,
	FREE,
    OVER
}

impl Default for Tile {
    fn default() -> Self {
        Tile::FREE
    }
}

impl Tile {
	pub fn from_str(s: &str) -> Tile {
		match s {
			"B" | "b" => Tile::BLACK,
			"W" | "w" => Tile::WHITE,
			_	      => Tile::FREE,
		}
	}

	pub fn is_free(&self) -> bool {
		*self == Tile::FREE
	}

	pub fn is_empty(&self) -> bool {
		*self == Tile::FREE || *self == Tile::OVER
	}

	pub fn is_pawn(&self) -> bool {
		*self == Tile::BLACK || *self == Tile::WHITE
	}

	/// Return the tile of the ennemy if there is one, return the tile itself
	/// otherwise.

	pub fn ennemy(&self) -> Tile {
		match *self {
		    Tile::BLACK => Tile::WHITE,
		    Tile::WHITE => Tile::BLACK,
		    _			=> self.clone()
		}
	}
}

impl Display for Tile {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			&Tile::BLACK => {let _ = write!(f, "B");},
			&Tile::WHITE => {let _ = write!(f, "W");},
			&Tile::FREE	 => {let _ = write!(f, ".");},
            &Tile::OVER  => {let _ = write!(f, "*");},
		};
		Ok(())
	}
}

impl Copy for Tile {
}
