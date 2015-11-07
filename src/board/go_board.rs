#![allow(dead_code)]
use std::fmt::{Formatter, Display, Error};
use board::{Tile, Team};

pub const GO_WIDTH : usize = 19;
const TILES_TO_WIN : usize = 5;

#[derive(Debug, PartialEq, Clone)]
pub struct GoBoard {
	/// The grid
	///
	/// First iterate over lines and then element by element.
	pub tiles:	[[Tile; GO_WIDTH]; GO_WIDTH], // The grid
	size:		usize, // Side
}

// test for one free threes pattern match
macro_rules! test_goban_pattern {
	($board:ident, $team:ident, $coords:ident, $($ty:expr => $gap:expr),*) => {{
		let mut result = true;
		$(
			let expected = match $ty {
				"o" => $team.get_tile(),
				"e" => $team.get_ennemy_tile(),
				"x" => Tile::FREE,
				_	=> panic!("GoBoard::test_goban_pattern synthax error")
			};
			// println!("[{}, {}]{:?} exp {:?}", $coords.0, $coords.1,
					// $board.get(($coords.0, $coords.1)), expected);
			result = $board.is_exp($coords, $gap, expected) && result;
		)*
		result as u32
	}}
}

impl GoBoard {

	/// The `get` function returns the tiles coordinates [x; y].

    pub fn get(&self, (x, y): (usize, usize)) -> Tile {
		self.tiles[x][y].clone()
	}

	fn capture_dir(&mut self,
		x: usize,
		y: usize,
		rightdir: i32,
		downdir: i32,
		team: &mut Team,
	) {
		let coords = (x, y, downdir, rightdir);
		let left = test_goban_pattern!(self, team, coords,
				"o" => 3, "e" => 2, "e" => 1) > 0;
		let right = test_goban_pattern!(self, team, coords,
				"o" => -3, "e" => -2, "e" => -1) > 0;
		if  left {
			self.unset_gap(coords, 1);
			self.unset_gap(coords, 2);
			team.add_captured(2);
			println!("captured left {:?}", team);
		}
		if  right {
			self.unset_gap(coords, -1);
			self.unset_gap(coords, -2);
			team.add_captured(2);
			println!("captured right {:?}", team);
		}
	}

	/// Test if playing this tile capture ennemy tiles and remove ennemy tiles
	/// and update number of captured tiles in the team if needed.

	fn capture(&mut self, x: usize, y: usize, team: &mut Team) {
		self.capture_dir(x, y, 1, 0, team);
		self.capture_dir(x, y, 0, 1, team);
		self.capture_dir(x, y, 1, 1, team);
		self.capture_dir(x, y, 1, -1, team);
	}

	/// Assigns the value to tiles coordinates [x; y] without any check.

	pub fn set_raw(&mut self, (x, y): (usize, usize), tile: Tile) {
		self.tiles[x][y] = tile;
	}

	/// The `set` function assigns the value to tiles coordinates [x; y]
	/// if possible. Return false otherwise.

    pub fn set(&mut self, (x, y): (usize, usize), team: &mut Team) -> bool {
    	if !self.is_allow(x, y, team) {
    		return false;
    	}
    	self.capture(x, y, team);
    	self.set_raw((x, y), team.get_tile());
		true
	}

	/// The `unset` function assigns the FREE
	/// to tiles coordinates [x; y].

    pub fn unset(&mut self, cell: (usize, usize)) {
		self.set_raw(cell, Tile::FREE);
	}

    pub fn unset_gap(&mut self,
		coords: (usize, usize, i32, i32),
		gap: i32,
	) {
		let x = coords.0 as i32 - coords.2 * gap;
		let y = coords.1 as i32 - coords.3 * gap;
		if x < 0 || y < 0 || !self.check_index((x as usize, y as usize)) {
			return ;
		}
		// println!("x {:?} y {:?} type {:?}", x, y, self.get((x as usize, y as usize)));
		self.unset((x as usize, y as usize));
	}

    /// The `get_size` function returns the size of
	/// the grid side.

	pub fn get_size (&self) -> usize {
		self.size
	}

    /// The `check_index` function returns a boolean
	/// if the index is within the bounds of the board.

	pub fn check_index (&self, (x, y): (usize, usize)) -> bool {
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
				!self.check_index((x as usize, y as usize)) ||
				self.get((x as usize, y as usize)) != tile_type {
			return ttl;
		}
		self.is_win_recursive (
			x - rightdir,
			y - downdir,
			downdir,
			rightdir,
			tile_type,
			ttl + 1
		)
	}

	/// Test if the tile at position [x, y] is winning on the direction
	/// [x - rightdir, y - downdir].
	fn is_win_direction (
		&self,
		x: usize,
		y: usize,
		downdir: i32,
		rightdir: i32,
	) -> bool {
		let tiles_on_dir = self.is_win_recursive (
			x as i32,
			y as i32,
			downdir,
			rightdir,
			self.get((x, y)),
			0
		);
		let tiles_on_opposite_dir = self.is_win_recursive (
			x as i32,
			y as i32,
			-downdir,
			-rightdir,
			self.get((x, y)),
			0
		);

		// since both tiles_on_dir and tiles_on_opposite_dir include the
		// begin tiles, we must substract it.
		(tiles_on_dir + tiles_on_opposite_dir - 1) >= TILES_TO_WIN
	}

	/// Test if the tile at [x, y] is a winning one.
	///
	/// The type of the tile represent the winning team
	/// (if Tile::WHITE, the white team has won).
	pub fn is_win(&self, x: usize, y: usize) -> Option<Tile> {
		if self.get((x, y)) == Tile::FREE {
			return None;
		}
		if self.is_win_direction(x, y, 1, 0) ||
				self.is_win_direction(x, y, 0, 1) ||
				self.is_win_direction(x, y, 1, 1) ||
				self.is_win_direction(x, y, 1, -1) {
			return Some(self.get((x, y)))
		}
		None
	}

	/// Return true if the tile which is positionned at gap tiles from the
	/// tested tile on the direction defined by coords is of the expected type.

	fn is_exp(&self,
		coords: (usize, usize, i32, i32),
		gap: i32,
		expected: Tile,
	) -> bool {
		let x = coords.0 as i32 - coords.2 * gap;
		let y = coords.1 as i32 - coords.3 * gap;
		if x < 0 || y < 0 || !self.check_index((x as usize, y as usize)) {
			return false;
		}
		// println!("x {:?} y {:?} type {:?} expected {:?}",
		// 		x, y, self.get((x as usize, y as usize)), expected);
		self.get((x as usize, y as usize)) == expected
	}

	/// Return the number of free threes in this direction.
	/// Assume that tile[x, y] is free.
	fn free_threes_dir(&self,
		x: usize,
		y: usize,
		downdir: i32,
		rightdir: i32,
		team: &Team,
	) -> u32 {
		let mut nb_free_three = 0;
		let coords = (x, y, downdir, rightdir);

		// x = Tile::FREE, o = Tile::Team, c = current postion (Tile::FREE)
		//rule 1
		// for xocox
		nb_free_three += test_goban_pattern!(self, team, coords,
				"x" => -2, "o" => -1, "o" => 1, "x" => 2);

		//rule 2
		// for xoocx
		nb_free_three += test_goban_pattern!(self, team, coords,
				"x" => -3, "o" => -2, "o" => -1, "x" => 1);
		// for xoocx (opposite)
		nb_free_three += test_goban_pattern!(self, team, coords,
				"x" => 3, "o" => 2, "o" => 1, "x" => -1);

		//rule 3
		// for xooxcx
		nb_free_three += test_goban_pattern!(self, team, coords,
				"x" => -4, "o" => -3, "o" => -2, "x" => -1, "x" => 1);
		// for xooxcx (opposite)
		nb_free_three += test_goban_pattern!(self, team, coords,
				"x" => 4, "o" => 3, "o" => 2, "x" => 1, "x" => -1);

		//return
		nb_free_three
	}

	/// Return true if the free threes rule allow this move.
	///
	/// A free-three is an alignement of three stones that, if not immediately
	/// blocked, allows for an indefendable alignment of four stones
	/// (that’s to say an alignment of four stones with two unobstructed
	/// extremities).
	fn free_threes(&self, x: usize, y: usize, team: &Team) -> bool {
		let nb_free_three = self.free_threes_dir(x, y, 1, 0, team) +
				self.free_threes_dir(x, y, 0, 1, team) +
				self.free_threes_dir(x, y, 1, 1, team) +
				self.free_threes_dir(x, y, 1, -1, team);
		nb_free_three < 2
	}

	/// Return true if it is allowed to add a tile on the position [x, y].
	/// x and y are supposed to be valid index
	pub fn is_allow(&self, x: usize, y: usize, team: &Team) -> bool {
		self.get((x, y)) == Tile::FREE && self.free_threes(x, y, team)
	}

	pub fn is_empty(&self) -> bool {
	    for line in self.tiles.iter() {
		    for tile in line.iter() {
		        if tile.is_pawn() {
		            return false;
		        }
		    }
	    }
	    true
	}

	/// Return the (x, y) coordinates out of the index of the tile in the
	/// GoBoard::tiles array.

	pub fn coord_out_of_index(index: usize) -> (usize, usize) {
		(index % GO_WIDTH, index / GO_WIDTH)
	}
}

impl Display for GoBoard {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		let mut to_return = Ok(());
		for y in (0..self.get_size()) {
			for x in (0..self.get_size()) {
				to_return = to_return.and(write!(f, "{} ", self.get((x, y))));
			}
			to_return = to_return.and(write!(f, "\n"));
		}
		to_return
	}
}

impl Default for GoBoard {

	/// The `new` constructor function returns the empty board.

    fn default() -> Self {
		GoBoard {
			tiles: [[Default::default(); GO_WIDTH]; GO_WIDTH],
			size:  GO_WIDTH,
		}
    }
}
