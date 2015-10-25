/// Returns a numerical value which approximate how close the board is to
/// victory for the team.
///
/// The team must have captured set to the actual number of captured.

pub fn heuristic (board: &GoBoard, team: Team) -> i32 {
    let situation: i32 = team.captured() as i32;

    for y in (0..board.get_size()) {
        for x in (0..board.get_size()) {
            if board.get((x, y)).is_pawn() {
                return match board.is_win(x, y) {
                    Some(tile) if tile == team.get_tile() => std::i32::MAX,
                    Some(tile) if tile != team.get_tile() => std::i32::MIN,
                    _ => {

                        continue ;
                    },
                }
            }
        }
    }
    situation
}
