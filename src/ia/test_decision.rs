use board::{GoBoard, Tile, Team};
use ia::Decision;
use ia::heuristic::HeuristicFn;

fn test_one(s: &str, heur: HeuristicFn, nb_layers: u32, expected: (usize, usize)) {
	let mut board = GoBoard::parse_with_size(&s.to_string());
	let (team_b, team_w) = Team::new_teams();
	println!("Test\n{}", board);
	let result =
			Decision::get_optimal_move(&mut board, &(team_b, team_w.clone()), team_w, nb_layers, heur);
	println!("result {:?}\n", result.get_result());
	assert!(expected == result.get_result());
}

// Does not work anymore because we don't update team captured.
// fn heur_capture(board: &GoBoard, team: Team) -> i32 {
// 	team.captured() as i32
// }

// ///test if the team captured is updated
// #[test]
// fn test_team_capture() {
// 	let s = r#"19
// . . . . . . . . . . . . . . . . . . .
// . W B B . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// . . . . . . . . . . . . . . . . . . .
// 	"#;
// 	test_one(s, heur_capture, 1, (4, 1));
// }

fn heur_tile_coords(board: &GoBoard, team: Team) -> i32 {
	let mut ttl = 0;
    for (x, line) in board.tiles.iter().enumerate() {
	    for (y, tile) in line.iter().enumerate() {
	        if tile.is_pawn() {
	        	ttl += (y * 19) + x;
	        }
	    }
    }
    ttl as i32
}

#[test]
fn test_decision() {
	let s = r#"19
W . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
	"#;
	test_one(s, heur_tile_coords, 2, (1, 1));
	test_one(s, heur_tile_coords, 3, (1, 1));
	test_one(s, heur_tile_coords, 4, (1, 1));
	// assert!(false);
}
