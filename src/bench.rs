extern crate test;

use board::{GoBoard, Team};
use ia::{Decision, heuristic};
use ia;

fn stupid_heuristic(board: &GoBoard, team: Team) -> i32 {
	42
}

#[bench]
fn minmax_3_layers_easy(b: &mut test::Bencher) {
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
		Decision::get_optimal_move(&mut board, &(team_b, team_w.clone()), team_w, 3, stupid_heuristic);
    })
}

#[bench]
fn minmax_3_layer_bench(b: &mut test::Bencher) {
 	let s = r#"19
. . . . . . . . . . . . . . . . . . .
. B . . . . . . . . . . . . . . . . .
. . W . B . . . W . . . . . . . . . .
. . . W . . . . . . . . . . . . . . .
. . . . W . . . . . . . . . . . . . .
. . . . . B . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . W B W B W B W W . . . . . .
. . . . . . . . . W . . . . . . . . .
. . . . . . . . . . W . . . . . . . .
. . . . . . . . . B . B . . . . . . .
. . . . . . . . B . . . B . . . . . .
. . . . . . . B . . . . . W . . . . .
. . . . . . W . . . . . . . W . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
	"#;
	let mut board = GoBoard::parse_with_size(&s.to_string());
	let (team_b, team_w) = Team::new_teams();
	b.iter(|| {
		Decision::get_optimal_move(&mut board, &(team_b, team_w.clone()), team_w, 3, stupid_heuristic);
	})
}

#[bench]
fn heuristic_bench_easy(b: &mut test::Bencher) {
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
	let (_, team_w) = Team::new_teams();
	b.iter(|| {
		heuristic(&mut board, team_w.clone());
	})
}

#[bench]
fn heuristic_bench(b: &mut test::Bencher) {
 	let s = r#"19
. . . . . . . . . . . . . . . . . . .
. B . . . . . . . . . . . . . . . . .
. . W . B . . . W . . . . . . . . . .
. . . W . . . . . . . . . . . . . . .
. . . . W . . . . . . . . . . . . . .
. . . . . B . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . W B W B W B W W . . . . . .
. . . . . . . . . W . . . . . . . . .
. . . . . . . . . . W . . . . . . . .
. . . . . . . . . B . B . . . . . . .
. . . . . . . . B . . . B . . . . . .
. . . . . . . B . . . . . W . . . . .
. . . . . . W . . . . . . . W . . . .
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
