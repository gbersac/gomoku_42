use std::fmt::{Formatter, Display, Error};
use board::Tile;

const GO_WIDTH: usize = 19;

#[derive(Debug)]
pub struct GoBoard {
	tiles: [[Tile; GO_WIDTH]; GO_WIDTH],
}

impl GoBoard {
	pub fn new() -> GoBoard {
		GoBoard {
			tiles: [[Tile::FREE; GO_WIDTH]; GO_WIDTH],
		}
	}

	/// Get the tiles which coordinates are [x, y]
	pub fn get(&self, x: usize, y: usize) -> Tile {
		self.tiles[x][y].clone()
	}

	pub fn get_size(&self) -> usize {
		GO_WIDTH
	}
}

impl Display for GoBoard
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
	{
		let mut to_return = Ok(());
		for y in (0..self.get_size()) {
			for x in (0..self.get_size()) {
				to_return = to_return.and(write!(f, "{} ", self.get(x, y)));
			}
			to_return = to_return.and(write!(f, "\n"));
		}
		to_return
	}
}
