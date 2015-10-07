use board::{GoBoard, Tile};

fn test_win(s: &str, x: usize, y: usize, is_win: Option<Tile>) {
	let board = GoBoard::parse_with_size(&s.to_string());
	println!("Test\n{}", board);
	assert!(board.is_win(x, y) == is_win);
}

#[test]
fn test_win_little() {
		let mut s = r#"5
. . . . .
. . . . .
. . . . .
. . . . .
. . . . .
		"#;
		test_win(s, 1, 1, None);
		let mut s = r#"5
. . . . .
. . . . .
W W W W W
. . . . .
. . . . .
		"#;
		test_win(s, 1, 1, None);
		test_win(s, 2, 2, Some(Tile::WHITE));
		test_win(s, 4, 2, Some(Tile::WHITE));
		let mut s = r#"5
W . . . .
. W . . .
W W W . W
. . . W .
. . . . .
		"#;
		test_win(s, 2, 2, None);
		let mut s = r#"5
W . . . .
. W . . .
W W W . W
. . . W .
. . . . W
		"#;
		test_win(s, 2, 2, Some(Tile::WHITE));
		let mut s = r#"5
W . . . W
. W . W .
W W W . W
. W . . .
W . . . W
		"#;
		test_win(s, 2, 2, Some(Tile::WHITE));
		let mut s = r#"5
W . W . .
. W W W .
W W W . W
. . W . .
W . W . W
		"#;
		test_win(s, 2, 2, Some(Tile::WHITE));
}

#[test]
fn test_win_big() {
		let mut s = r#"19
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . .
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
		test_win(s, 3, 7, Some(Tile::WHITE));
		test_win(s, 2, 7, None);
}
