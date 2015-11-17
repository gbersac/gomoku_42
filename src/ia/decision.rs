use std::thread;
use std::sync::mpsc;
use board::{GoBoard, Team, Tile};
use ia;
use ia::turn::Turn;
use ia::heuristic::HeuristicFn;
use chrono::{UTC, Duration};

#[derive(Clone)]
pub struct Decision {
	player: Team,
	nb_node: usize,
	nb_final: usize,
	time_in_heuristic: Duration
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
	fn compute_one_move(&mut self,
		coords: (usize, usize),
		board: &mut GoBoard,
		mut playing_team: Team,
		teams: (Team, Team),
		nb_layers: u32,
		turn: Turn,
		albet: (i32, i32),
		heuristic: HeuristicFn
	) -> ((usize, usize), i32) {
		board.set(coords, &mut playing_team);
		let teams = Decision::updated_team(&teams, playing_team.clone());
		let (_, heur) = self.recursive(
				board, turn.other(), teams.clone(), nb_layers - 1, albet, heuristic);
		(coords, heur)
	}

	fn select_best_move(&mut self,
		moves: &Vec<(usize, usize)>,
		board: &mut GoBoard,
		mut playing_team: Team,
		teams: &(Team, Team),
		nb_layers: u32,
		turn: Turn,
		albet: &(i32, i32),
		heuristic: HeuristicFn
	) -> ((usize, usize), i32) {
		let mut results = Vec::with_capacity(moves.len());
		for mov in moves {
			let res = self.compute_one_move(
					*mov, &mut board.clone(), playing_team.clone(), teams.clone(),
					nb_layers, Turn::Adversary, *albet, heuristic);
			results.push(res);
		}
		// println!("nb iter {:?} nb_layers {}", moves.len(), nb_layers);
		let res = results.iter().fold(turn.default_result(), turn.sort_fn());
		res
	}

	fn algo_min(&mut self,
		moves: &Vec<(usize, usize)>,
		board: &mut GoBoard,
		playing_team: &Team,
		teams: &(Team, Team),
		nb_layers: u32,
		(alpha, mut beta) : (i32, i32),
		heuristic: HeuristicFn
	) -> ((usize, usize), i32) {
		let (mut coords, mut val) = self.select_best_move(
				moves, board, playing_team.clone(), &teams,
				nb_layers, Turn::Adversary, &(alpha, beta), heuristic);
		if alpha > val { //alpha cut. We know this branch won't be selected
			println!("alpha cut");
			return (coords, val);
		}
		if beta < val { // beta = Min(beta, Val)
			beta = val;
		}
		(coords, val)
	}

	fn algo_max(&mut self,
		moves: &Vec<(usize, usize)>,
		board: &mut GoBoard,
		playing_team: &Team,
		teams: &(Team, Team),
		nb_layers: u32,
		(mut alpha, beta) : (i32, i32),
		heuristic: HeuristicFn
	) -> ((usize, usize), i32) {
		let (mut coords, mut val) = self.select_best_move(
				moves, board, playing_team.clone(), &teams,
				nb_layers, Turn::Player, &(alpha, beta), heuristic);
		if val > beta { //beta cut. We know this branch won't be selected
			println!("beta cut");
			return (coords, val);
		}
		if alpha > val { // alpha = Max(alpha, Val)
			alpha = val;
		}
		(coords, val)
	}

	/// albet: alpha < beta
	fn recursive(&mut self,
		board: &mut GoBoard,
		turn: Turn,
		teams: (Team, Team),
		nb_layers: u32,
		albet : (i32, i32),
		heuristic: HeuristicFn
	) -> ((usize, usize), i32) {
		self.nb_node += 1;
		let playing_team: Team = Decision::playing_team(&turn, &teams, &mut self.player).clone();

		// if it is a leaf return heuristic value for this board
		if nb_layers == 0 {
			let updated_player = match self.player.get_tile() {
				Tile::BLACK => teams.0,
				Tile::WHITE => teams.1,
				Tile::FREE => panic!("bad team type"),
			};
			// is there moves where the coords value matter for this ?
			self.nb_final += 1;
			let begin = UTC::now();
			let res = ((0, 0), (heuristic)(&board, updated_player));
			let end = UTC::now();
			// println!("begin {:?} end {:?} diff {:?}", begin, end, (end - begin));
			self.time_in_heuristic = self.time_in_heuristic + (end - begin);
			return res;
		}

		// get potential next moves
		let moves = super::move_to_evaluate::move_to_evaluate(&board, &playing_team);
		if moves.len() == 0 {
			unimplemented!();
		}

		// test each move
		let default_result = ((0, 0), turn.init());
		let to_return = match turn.is_min() {
			true => {
				self.algo_min(&moves, board, &playing_team, &teams, nb_layers, albet, heuristic)
			},
			false => {
				self.algo_max(&moves, board, &playing_team, &teams, nb_layers, albet, heuristic)
			},
		};
		to_return
	}

	fn print_result(&self,
		(x_result, y_result): (usize, usize),
		total_time: Duration
	) {
		println!("###IA search best move for team {}", self.player);
		println!("Number of heuristic calls {}", self.nb_final);
		println!("Number of node            {}", self.nb_node);
		println!("Time to compute       {: >#2}s {}ms", total_time.num_seconds(), total_time.num_milliseconds());
		println!("Time in heuristic     {: >#2}s {}ms", self.time_in_heuristic.num_seconds(), self.time_in_heuristic.num_milliseconds());
		let time_out_of_heuristic = total_time - self.time_in_heuristic;
		println!("Time out of heuristic {: >#2}s {}ms", time_out_of_heuristic.num_seconds(), time_out_of_heuristic.num_milliseconds());
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
		board: &mut GoBoard,
		teams: &(Team, Team),
		player: Team,
		nb_layers: u32,
		heuristic: HeuristicFn
	) -> (usize, usize) {
		if board.is_empty() {
			return (9, 9);
		}
		let mut dec = Decision {
			player: player.clone(),
			nb_node: 0,
			nb_final: 0,
			time_in_heuristic: Duration::zero()
		};
		let begin = UTC::now();
		let (coords, _) = dec.recursive(board, Turn::Player, *teams, nb_layers,
				(-ia::INFINITE, ia::INFINITE), heuristic);
		let end = UTC::now();
		dec.print_result(coords, end - begin);
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
