use std::fmt::{Formatter, Display, Error};
use board::Tile;

pub const GO_WIDTH : usize = 19;
const TILES_TO_WIN : usize = 5;

#[derive(Debug)]
pub struct GoBoard {
	tiles: [[Tile; GO_WIDTH]; GO_WIDTH],
	size:	usize,
}

impl GoBoard {
	pub fn new() -> GoBoard {
		GoBoard {
			tiles: [[Tile::FREE; GO_WIDTH]; GO_WIDTH],
			size:  GO_WIDTH,
		}
	}

	/// Get the tiles which coordinates are [x, y]
	pub fn get(&self, x: usize, y: usize) -> Tile {
		self.tiles[x][y].clone()
	}

	/// Get the tiles which coordinates are [x, y]
	pub fn set(&mut self, x: usize, y: usize, val: Tile) {
		self.tiles[x][y] = val;
	}

	pub fn get_size(&self) -> usize {
		self.size
	}

	/// Return true if the index is within the bounds of the board.
	pub fn index_is_correct(&self, x:usize, y:usize) -> bool {
		x <= self.size - 1 && y <= self.size - 1
	}

	fn is_win_recursive (
		&self, x: i32,
		y: i32,
		downdir: i32,
		rightdir: i32,
		tile_type: Tile,
		ttl: usize,
	) -> usize {
		if x < 0 || y < 0  ||
				!self.index_is_correct(x as usize, y as usize) ||
				self.get(x as usize, y as usize) != tile_type {
			return ttl;
		}
		self.is_win_recursive(x - rightdir, y - downdir,
				downdir, rightdir, tile_type, ttl + 1)
	}

	/// Test if the tile at position [x, y] is winning on the direction
	/// [x - rightdir, y - downdir].
	fn is_win_direction(
		&self,
		x: usize,
		y: usize,
		downdir: i32,
		rightdir: i32,
	) -> bool {
		let tiles_on_dir = self.is_win_recursive(x as i32, y as i32, downdir,
				rightdir, self.get(x, y), 0);
		let tiles_on_opposite_dir = self.is_win_recursive(x as i32, y as i32,
				-downdir, -rightdir, self.get(x, y), 0);

		// since both tiles_on_dir and tiles_on_opposite_dir include the
		// begin tiles, we must substract it.
		(tiles_on_dir + tiles_on_opposite_dir - 1) >= TILES_TO_WIN
	}

	/// Test if the tile at [x, y] is a winning one.
	///
	/// The type of the tile represent the winning team
	/// (if Tile::WHITE, the white team has won).
	pub fn is_win(&self, x: usize, y: usize) -> Option<Tile> {
		if self.get(x, y) == Tile::FREE {
			return None;
		}
		if self.is_win_direction(x, y, 1, 0) ||
				self.is_win_direction(x, y, 0, 1) ||
				self.is_win_direction(x, y, 1, 1) ||
				self.is_win_direction(x, y, 1, -1) {
			return Some(self.get(x, y))
		}
		None
	}
}

impl Display for GoBoard {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
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
