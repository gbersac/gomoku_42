use std::fmt::{Formatter, Display, Error};

#[derive(Debug, PartialEq, Clone)]
pub enum Tile{
	BLACK,
	WHITE,
	FREE
}

impl Tile {
	pub fn is_free(&self) -> bool {
		*self == Tile::FREE
	}
}

impl Display for Tile
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		match self {
			&Tile::BLACK	=> {let _ = write!(f, "B");},
			&Tile::WHITE	=> {let _ = write!(f, "W");},
			&Tile::FREE		=> {let _ = write!(f, ".");},
		};
		Ok(())
	}
}
