use board::{GoBoard, Tile, Team};
use ia::Decision;
use ia::heuristic::HeuristicFn;

fn test_one(s: &str, heur: HeuristicFn, nb_layers: u32, expected: (usize, usize)) {
	let board = GoBoard::parse_with_size(&s.to_string());
	let (team_b, team_w) = Team::new_teams();
	println!("Test\n{}", board);
	let result =
			Decision::get_optimal_move(&board, &(team_b, team_w.clone()), team_w, nb_layers, heur);
	println!("result {:?}\n", result);
	assert!(expected == result);
}

fn heur_capture(board: &GoBoard, team: &Team) -> i32 {
	println!("heur_capture {:?}", team.captured());
	team.captured() as i32
}

///test if the team captured is updated
#[test]
fn test_team_capture() {
	let s = r#"19
. . . . . . . . . . . . . . . . . . .
. W B B . . . . . . . . . . . . . . .
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
	test_one(s, heur_capture, 1, (4, 1));
}
