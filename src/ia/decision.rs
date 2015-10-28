use board::{GoBoard, Team, Tile};
use ia;
use ia::turn::Turn;
use ia::heuristic::HeuristicFn;

pub struct Decision {
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

	/// Return the tuple (team black, team white) with the new team
	fn updated_team(&(ref tb, ref tw): &(Team, Team), new_team: Team) -> (Team, Team) {
		match new_team.get_tile() {
		    Tile::BLACK => (new_team, tw.clone()),
		    Tile::WHITE => (tb.clone(), new_team),
		    Tile::FREE => panic!("error forbiden team tile")
		}
	}

	/// launch the recursive for one of the move to evaluate
	fn compute_one_move(&self,
		coords: (usize, usize),
		board: &GoBoard,
		turn: Turn,
		teams: &(Team, Team),
		nb_layers: u32
	) -> ((usize, usize), i32) {
		let mut playing_team = Decision::playing_team(&turn, teams, &self.player).clone();
	    let mut newb = board.clone();
	    newb.set(coords, &mut playing_team);
	    let teams = Decision::updated_team(teams, playing_team);
	    println!("for coords recursive {:?}", coords);
	    let (_, heur) = self.recursive(&newb, turn.other(), &teams, nb_layers - 1);
	    (coords, heur)
	}

	fn sort_move(
		acc: ((usize, usize), i32),
		item: &((usize, usize), i32),
		turn: Turn
	) -> ((usize, usize), i32) {
		let mut to_return = acc;
		if (acc.1 > item.1 && turn.is_min()) || (acc.1 < item.1 && !turn.is_min()) {
			to_return =  *item;
		}
		to_return
	}

	fn recursive(&self,
		board: &GoBoard,
		turn: Turn,
		teams: &(Team, Team),
		nb_layers: u32,
	) -> ((usize, usize), i32) {
		if nb_layers == 1 {
			println!("\n#############################");
		}
		println!("###recursive {}", nb_layers);
		let playing_team: Team = Decision::playing_team(&turn, teams, &self.player).clone();
		if nb_layers == 0 {
			let updated_player = match self.player.get_tile() {
			    Tile::BLACK => &teams.0,
			    Tile::WHITE => &teams.1,
			    Tile::FREE => panic!("bad team type"),
			};
			// is there moves where the coords value matter for this ?
			return ((0, 0), (self.heuristic)(board, updated_player));
		}
		let moves = super::move_to_evaluate::move_to_evaluate(board, &playing_team);
		if moves.len() == 0 {
			unimplemented!();
		}
		// test each playable
		let default_result = ((0, 0), turn.init());
		let to_return = moves.iter()
				.map(|x| self.compute_one_move(*x, &board, turn.clone(), teams, nb_layers))
				// select min or max according to convenience
				.fold(default_result, |acc, item| Decision::sort_move(acc, &item, turn.clone()));
		println!("to_return {:?}", to_return);
		to_return
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
		nb_layers: u32,
		heur: HeuristicFn
	) -> (usize, usize) {
		if board.is_empty() {
			return (9, 9);
		}
		let dec = Decision {
			heuristic: heur,
			player: player.clone()
		};
		let (coords, _) = dec.recursive(board, Turn::Player, teams, nb_layers);
		coords
	}
}

#[test]
fn test_playing_team() {
	let teams = Team::new_teams();
	let tadv = Turn::Adversary;
	let tpla = Turn::Player;
	let (adv, pla) = Team::new_teams(); // player is white

	assert!(Decision::playing_team(&tpla, &teams, &pla) == pla);
	assert!(Decision::playing_team(&tadv, &teams, &pla) == adv);
	assert!(Decision::playing_team(&tpla, &teams, &adv) == adv);
	assert!(Decision::playing_team(&tadv, &teams, &adv) == pla);
}
