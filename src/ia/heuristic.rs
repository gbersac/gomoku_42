/// Returns a numerical value which approximate how close the board is to
/// victory for the team.
///
/// The team must have captured set to the actual number of captured.

fn free_threes_right (
    grid: [[i32; 8]; 8],
    player: &Tile,
    x: usize,
    y: usize
) -> i32 {
    let mut decision: i32 = 0;

    for x in x..grid[y].len() {
        decision += match grid[y][x].get() {
            tile if tile == player => 1,
            tile if tile == player.ennemy() => -1,
            _ => continue ,
        }
    }
    decision
}

pub fn heuristic (
    board: &GoBoard,
    team: Team
) -> i32 {
    let situation: i32 = team.captured() as i32;
    let mut decision: i32 = 0;

    for y in (0..board.get_size()) {
        for x in (0..board.get_size()) {
            if board.get((x, y)).is_pawn() {
                return match board.is_win(x, y) {
                    Some(tile) if tile == team.get_tile() => std::i32::MAX,
                    Some(tile) if tile != team.get_tile() => std::i32::MIN,
                    _ => {
                        decision += free_threes_right(grid, team.get_tile(), x, y);
                        continue ;
                    },
                }
            }
        }
    }
    situation
}


#[test]
fn test_capture() {
    	let s = r#"19
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
    . . . . . . . . . . . . . . . . . . .
    		"#;
}
