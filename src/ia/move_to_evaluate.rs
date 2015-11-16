use board::{GoBoard, Team};

fn test_pawn(
	board: &GoBoard,
	(x, y): (usize, usize),
	team: &Team,
	vec_coord: &Vec<(usize, usize)>
) -> bool {
	board.check_index((x, y)) &&
			board.is_allow(x, y, team) &&
			vec_coord.iter().find(|&r| *r == (x, y)).is_none()
}

fn get_neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
	if x > 0 && y > 0 {
		return vec!((x - 1, y),  (x, y - 1), (x - 1, y - 1), (x + 1, y - 1),
			(x + 1, y), (x, y + 1), (x + 1, y + 1), (x - 1, y + 1));
	}
	let mut to_return = vec!((x + 1, y), (x, y + 1), (x + 1, y + 1));
	if x > 0 {
		to_return.extend(vec!((x - 1, y), (x - 1, y + 1)));
	}
	if y > 0 {
		to_return.extend(vec!((x, y - 1), (x + 1, y - 1)));
	}
    to_return
}

pub fn move_to_evaluate(board: &GoBoard, team: &Team) -> Vec<(usize, usize)> {
	let mut to_return = Vec::new();
	for (y, line) in board.tiles.iter().enumerate() {
		for (x, tile) in line.iter().enumerate() {
			if tile.is_pawn() {
				// I don't really understand why x and y need to be reversed.
				let neighbors = get_neighbors(y, x);
				for neighbor in neighbors {
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
