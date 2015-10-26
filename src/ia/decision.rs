extern crate std;

use board::GoBoard;
use board::Team;

const INFINITE: i32 = std::i32::MAX;

enum Player {
    Me,
    Adversary
}

fn test_pawn(
		board: &GoBoard,
		(x, y): (usize, usize),
		team: &Team,
		vec_coord: &Vec<(usize, usize)>)
-> bool {
	board.check_index((x, y)) &&
			board.is_allow(x, y, team) &&
			vec_coord.iter().find(|&r| *r == (x, y)).is_some()
}

fn move_to_evaluate(board: &GoBoard, team: &Team) -> Vec<(usize, usize)> {
	let mut to_return = Vec::new();
	for (y, line) in board.tiles.iter().enumerate() {
		for (x, tile) in line.iter().enumerate() {
		    if tile.is_pawn() {
		    	let neighbors =
		    			vec!((x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1));
		    	for neighbor in neighbors {
			    	// is real pawn
			    	if !test_pawn(board, neighbor, team, &to_return) {
			    		continue ;
			    	}
			    	to_return.push(neighbor);
		    	}
		    }
	    }
	}
	to_return
}

fn recursive() {
	// list all available move for the crate

	// pour chacun des coups possibles tester la valeur

	// selectionner le min max
}

/// Return the coordinates of the move which is considered to maximise the
/// odds of victory for the team.
///
/// nb_layers is the number of move which is going to be evaluated by the ia
/// to evaluate the best move. The higher will improve the quality of result
/// but also computationnal time.
pub fn get_optimal_move (board: &GoBoard, team: &Team, nb_layers: u32)
		-> (i32, i32) {
	if board.is_empty() {
		return (9, 9);
	}
	recursive();
	(0i32, 0i32)
}
