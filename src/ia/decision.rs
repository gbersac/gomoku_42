use board::{GoBoard, Team, Tile};
use ia;
use ia::turn::Turn;
use ia::heuristic::HeuristicFn;


struct Decision {
    heuristic: HeuristicFn,
    player: Team
}

impl Decision {
	fn playing_team(
		turn: &Turn,
		teams: &(Team, Team),
		player: &Team,
	) -> Team {
		match *turn {
		    Turn::Adversary => {
		    	match player.get_tile() {
		    	    Tile::BLACK => teams.1.clone(),
		    	    Tile::WHITE => teams.0.clone(),
		    	    _ => panic!("No Free tile allowed"),
		    	}
		    },
		    Turn::Player => {
		    	match player.get_tile() {
		    	    Tile::BLACK => teams.0.clone(),
		    	    Tile::WHITE => teams.1.clone(),
		    	    _ => panic!("No Free tile allowed"),
		    	}
		    },
		}
	}

	fn compute_one_move(
		coords: (usize, usize),
		board: &GoBoard,
		turn: Turn,
		teams: &(Team, Team),
		player: &Team,
		nb_layers: u32
	) -> ((usize, usize), i32) {
		let mut playing_team = Decision::playing_team(&turn, teams, &player).clone();
	    let mut newb = board.clone();
	    newb.set(coords, &mut playing_team);
	    Decision::recursive(&newb, turn.other(), teams, player, nb_layers - 1)
	}

	fn sort_move(
		acc: ((usize, usize), i32),
		item: &((usize, usize), i32),
		turn: Turn
	) -> ((usize, usize), i32) {
		if (acc.1 > item.1 && turn.is_min()) || (acc.1 < item.1 && !turn.is_min()) {
			return *item;
		}
		acc
	}

	fn recursive(
		board: &GoBoard,
		turn: Turn,
		teams: &(Team, Team),
		player: &Team,
		nb_layers: u32,

	) -> ((usize, usize), i32) {
		let playing_team: Team = Decision::playing_team(&turn, teams, &player).clone();
		if nb_layers == 0 {
			// is there moves where the coords value matter for this ?
			return ((0, 0), ia::heuristic::heuristic(board, playing_team));
		}
		let moves = super::move_to_evaluate::move_to_evaluate(board, &playing_team);
		if moves.len() == 0 {
			unimplemented!();
		}
		// test each playable
		let default_result = ((0, 0), turn.init());
		moves.iter()
				.map(|x| Decision::compute_one_move(*x, &board, turn.clone(), teams, &playing_team, nb_layers))
				// select min or max according to convenience
				.fold(default_result, |acc, item| Decision::sort_move(acc, &item, turn.clone()))
	}



	/// Return the coordinates of the move which is considered to maximise the
	/// odds of victory for the team.
	///
	/// teams the two teams, black first and white second
	///
	/// player is the team for which we want to find the optimal move
	///
	/// nb_layers is the number of move which is going to be evaluated by the ia
	/// to evaluate the best move. The higher will improve the quality of result
	/// but also computationnal time.
	pub fn get_optimal_move (
		board: &GoBoard,
		teams: &(Team, Team),
		player: Team,
		nb_layers: u32
	) -> (usize, usize) {
		if board.is_empty() {
			return (9, 9);
		}
		let (coords, _) = Decision::recursive(board, Turn::Player, teams, &player, nb_layers);
		coords
	}
}

#[test]
fn test_playing_team() {
	let teams = Team::new_teams();
	let tadv = Turn::Adversary;
	let tpla = Turn::Player;
	let (adv, pla) = Team::new_teams(); // player is white

	assert!(playing_team(&tpla, &teams, &pla) == pla);
	assert!(playing_team(&tadv, &teams, &pla) == adv);
	assert!(playing_team(&tpla, &teams, &adv) == adv);
	assert!(playing_team(&tadv, &teams, &adv) == pla);
}
