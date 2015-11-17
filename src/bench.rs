extern crate test;

use board::{GoBoard, Team};
use ia::{Decision, heuristic};

fn stupid_heuristic(board: &GoBoard, team: Team) -> i32 {
	42
}

#[bench]
fn minmax_algo_3_layers(b: &mut test::Bencher) {
	let s = r#"19
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . W . . . . . . . . . .
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
	let mut board = GoBoard::parse_with_size(&s.to_string());
	let (team_b, team_w) = Team::new_teams();
    b.iter(|| {
		let result =
				Decision::get_optimal_move(&mut board, &(team_b, team_w.clone()), team_w, 3, stupid_heuristic);
    })
}

#[bench]
fn heuristic_bench(b: &mut test::Bencher) {
 	let s = r#"19
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . W . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . W B W B W B W W . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . B . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
	"#;
	let mut board = GoBoard::parse_with_size(&s.to_string());
	let (_, team_w) = Team::new_teams();
	b.iter(|| {
		heuristic(&mut board, team_w.clone());
	})
}
