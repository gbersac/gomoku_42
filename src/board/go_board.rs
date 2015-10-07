use std::fmt::{Formatter, Display, Error};
use board::Tile;

static GO_WIDTH : usize = 19;

#[derive(Debug)]
pub struct GoBoard{
	tiles:		Vec<Tile>,
}

impl GoBoard {
	pub fn new() -> GoBoard {
		let mut to_return = GoBoard{
			tiles:	Vec::with_capacity(GO_WIDTH * GO_WIDTH),
		};
		for _ in 0..(GO_WIDTH * GO_WIDTH) {
			to_return.tiles.push(Tile::FREE);
		}
		to_return
	}

	/// Get the tiles which coordinates are [x, y]
	pub fn get(&self, x: usize, y: usize) -> Tile {
		self.tiles[(y * GO_WIDTH + x)].clone()
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
