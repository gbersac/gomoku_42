use board::{GoBoard, Team};
use ai::{Decision, heuristic};

fn stupid_heuristic(board: &GoBoard, team: Team) -> i32 {
    42
}

#[test]
fn one_test_minmax() {
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
    let result = Decision::get_optimal_move(&mut board,
                                            &(team_b, team_w.clone()),
                                            team_w,
                                            3,
                                            stupid_heuristic);
}
