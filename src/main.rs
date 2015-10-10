mod board;
mod display;
mod ia;

use board::{GoBoard};

fn main() {
	let board = GoBoard::new();
	println!("{}", board);

	display::main(board);
}
