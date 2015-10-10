#[cfg(test)]

use board::{Tile, GoBoard};
use board::fn_str;

impl GoBoard {
	fn split_one_line(line: &str) -> Vec<Tile> {
		line.split(' ')
				.map(|x| x.trim())
				.filter(|x| x.len() > 0)
				.map(|x| {Tile::from_str(x)})
				.collect()
	}

	fn split_into_lines(to_parse: &String) -> Vec<&str> {
		to_parse.split('\n')
				.map(|x| x.trim())
				.filter(|x| x.len() > 0)
				.filter(|x| x.chars().next() != "#".chars().next())
				.collect::<Vec<&str>>()
	}

	/// Parse a string which describe the inital state of the npuzzle board.
	fn execute_parse(size: usize, lines: &Vec<&str>)
			-> GoBoard {

		// split lines into integer
		let mut tiles = Vec::with_capacity(size * size);
		for line in lines {
			let mut ntiles : Vec<Tile> = GoBoard::split_one_line(line);
			tiles.extend(ntiles);
		}

		//create board, no check
        let mut board = GoBoard::new();
		let size = board.get_size();
        for x in 0..size {
			for y in 0..size {
				board.set((x, y), tiles[y * size + x].clone());
			}
		}
		board
	}

	/// This function also parse the size of the Board.
	/// This function should only be used in test because there is no test.
	pub fn parse_with_size(to_parse: &String)
			-> GoBoard {
		let lines = GoBoard::split_into_lines(to_parse);
		let size = fn_str::atoi::<usize>(lines[0]).unwrap();
		let lines_reduce = (&lines[1..]).to_vec();
		GoBoard::execute_parse(size, &lines_reduce)
	}
}

mod test {
	use super::*;
	use board::{GoBoard, Tile};

	#[test]
	fn test_parse() {
		let mut str1 = r#"5
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
W . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . W . . . . . . . . . . . . . . .
. . . . W . . . . . . . . . . . . . .
. . . . . W . . . . . . . . . . . . .
. . . . . . W . . . . . . . . . . . .
. . . . . . . W . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
		"#;
		let board = GoBoard::parse_with_size(&str1.to_string());
		assert!(board.get((0, 2)) == Tile::WHITE);
	}
}
