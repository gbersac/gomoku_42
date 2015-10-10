mod board;
mod display;
mod ia;

use board::{GoBoard, Team};

fn main() {
	let board = GoBoard::new();
	println!("{}", board);
	let mut teams = Team::new_teams();
	display::main(board, &mut teams);
}
