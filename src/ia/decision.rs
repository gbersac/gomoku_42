use std::thread;
use std::sync::mpsc;
use std::fmt::{Formatter, Display, Error};
use std;
use board::{GoBoard, Team, Tile};
use ia;
use ia::turn::Turn;
use ia::heuristic::HeuristicFn;
use chrono::{UTC, Duration};

const NB_THREAD: usize = 4;

#[derive(Debug, PartialEq, Clone)]
pub struct Decision {
	player: Team,
	nb_layers: u32,
	nb_node: usize,
	nb_final: usize,
	time_in_heuristic: Duration,
	total_time: Duration,
	result: (usize, usize)
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

	fn splitted_moves_to_evaluate(board: &GoBoard) -> Vec<Vec<(usize, usize)>> {
		let mut moves = super::move_to_evaluate::move_to_evaluate(&board);
		let ttl_len = moves.len();
		let mut to_return = Vec::new();
		if moves.len() == 0 {
			panic!("No winner !");
		}

		let split = moves.len() / NB_THREAD;
		for i in 1..NB_THREAD {
			let y = moves;
			let (n, m) = if i < ttl_len % NB_THREAD {
				y.split_at(split + 1)
			} else {
				y.split_at(split)
			};
			moves = m.to_vec();
			to_return.push(n.to_vec());
		}
		to_return.push(moves.to_vec());
		to_return
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
		playing_team: Team,
		teams: (Team, Team),
		nb_layers: u32,
		turn: Turn,
		albet: (i32, i32),
		heuristic: HeuristicFn
	) -> ((usize, usize), i32) {
		board.set_raw(coords, playing_team.get_tile());
		let teams = Decision::updated_team(&teams, playing_team.clone());
		let (_, heur) = self.recursive(
				board, turn, teams.clone(), nb_layers, albet, heuristic);
		(coords, heur)
	}

	/// albet: alpha < beta
	/// [algo explication](https://en.wikipedia.org/wiki/Negamax)
	fn recursive(&mut self,
		board: &mut GoBoard,
		turn: Turn,
		teams: (Team, Team),
		nb_layers: u32,
		(mut alpha, beta) : (i32, i32),
		heuristic: HeuristicFn,
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
			let (coords, value) = ((0, 0), (heuristic)(&board, updated_player));
			let end = UTC::now();
			self.time_in_heuristic = self.time_in_heuristic + (end - begin);
			return (coords, value * turn.sign_alternation());
		}

		//else recursive to call each childs

		// get potential next moves
		let moves = super::move_to_evaluate::move_to_evaluate(&board);

		// best heuristic value for one move set to -infinite
		let mut best_value = ia::NINFINITE;
		let mut best_coord = (0, 0);
		for mov in moves {
			let (one_coord, one_val) = self.compute_one_move(mov,
					&mut board.clone(),
					playing_team.clone(),
					teams.clone(),
					nb_layers - 1,
					turn.other(),
					(ia::neg_infinite(beta), ia::neg_infinite(alpha)),
					heuristic);
			if one_val > best_value {
				best_value = one_val;
				best_coord = one_coord;
				alpha = std::cmp::max(alpha, best_value);
				if alpha >= beta {
					// println!("elagage alpha beta");
					break ;
				}
			}
		}
		(best_coord, -best_value)
	}

	pub fn print_result(&self) {
		println!("###IA search best move for team {}, num of layers {}", self.player, self.nb_layers);
		println!("Number of heuristic calls {}", self.nb_final);
		println!("Number of node            {}", self.nb_node);
		println!("Time to compute   {: >#2}s {}ms", self.total_time.num_seconds(), self.total_time.num_milliseconds());
		println!("Time in heuristic {: >#2}s {}ms", self.time_in_heuristic.num_seconds(), self.time_in_heuristic.num_milliseconds());
		let time_out_of_heuristic = self.total_time - self.time_in_heuristic;
		println!("Time out of heuristic {: >#2}s {}ms\n", time_out_of_heuristic.num_seconds(), time_out_of_heuristic.num_milliseconds());
	}

	/// albet: alpha < beta
	/// [algo explication](https://en.wikipedia.org/wiki/Negamax)
	fn thread_recursive(&mut self,
		moves: Vec<(usize, usize)>,
		board: &mut GoBoard,
		turn: Turn,
		teams: (Team, Team),
		nb_layers: u32,
		(mut alpha, beta) : (i32, i32),
		heuristic: HeuristicFn,
	) -> ((usize, usize), i32) {
		let playing_team: Team = Decision::playing_team(&turn, &teams, &mut self.player).clone();

		// best heuristic value for one move set to -infinite
		let mut best_value = ia::NINFINITE;
		let mut best_coord = (0, 0);
		for mov in moves {
			let (one_coord, one_val) = self.compute_one_move(mov,
					&mut board.clone(),
					playing_team.clone(),
					teams.clone(),
					nb_layers,
					turn.other(),
					(ia::neg_infinite(beta), ia::neg_infinite(alpha)),
					heuristic);
			if one_val > best_value && board.is_allow(one_coord.0, one_coord.1, &playing_team) {
				best_value = one_val;
				best_coord = one_coord;
				alpha = std::cmp::max(alpha, best_value);
				if alpha >= beta {
					// println!("elagage alpha beta");
					break ;
				}
			}
		}
		(best_coord, -best_value)
	}


	/// albet: alpha < beta
	/// [algo explication](https://en.wikipedia.org/wiki/Negamax)
	fn first_recursive(&mut self,
		board: &mut GoBoard,
		turn: Turn,
		teams: (Team, Team),
		nb_layers: u32,
		(alpha, beta) : (i32, i32),
		heuristic: HeuristicFn
	) -> ((usize, usize), i32) {
		let list_moves = Decision::splitted_moves_to_evaluate(board);

		// best heuristic value for one move set to -infinite
		let (tx, rx) = mpsc::channel();

		//spawn one resolution thread for each move
		for mov in &list_moves {
			let tx = tx.clone();

			// //clone a lot of stuff so that we could send them to the thread
			let mut self_c = self.clone();
			let mut board_c = board.clone();
			let mov_c = mov.clone();
			let turn_o = turn.other();

			thread::spawn(move || {
				let res = self_c.thread_recursive(mov_c,
												  &mut board_c,
												  turn_o,
												  teams.clone(),
												  nb_layers,
												  (ia::neg_infinite(beta),
												   ia::neg_infinite(alpha)),
												  heuristic);
				let _ = tx.send(res);
			});
		}

		let mut results = Vec::with_capacity(NB_THREAD);
		for _ in 0..NB_THREAD {
			let res = rx.recv().unwrap();
			results.push(res);
		}

		// select min or max according to convenience
		let res = results.iter().max_by_key(|x| x.1);
		*(res.unwrap())
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
	) -> Decision {
		let mut dec = Decision {
			player: player.clone(),
			nb_node: 0,
			nb_final: 0,
			time_in_heuristic: Duration::zero(),
			total_time: Duration::zero(),
			result: (0, 0),
			nb_layers: nb_layers
		};
		if board.is_empty() {
			dec.result = (9, 9);
			return dec;
		}
		let begin = UTC::now();
		let (coords, _) = dec.first_recursive(board, Turn::Player, *teams, nb_layers,
				(ia::NINFINITE, ia::INFINITE), heuristic);
		let end = UTC::now();
		dec.result = coords;
		dec.total_time = end - begin;
		dec
	}

	pub fn get_result(&self) -> (usize, usize) {
		self.result
	}
}

impl Default for Decision {

	/// The `new` constructor function returns the interface decision.
	fn default () -> Self {
		Decision {
			player: Team::default(),
			nb_layers: 0u32,
			nb_node: 0usize,
			nb_final: 0usize,
			time_in_heuristic: Duration::zero(),
			total_time: Duration::zero(),
			result: (0usize, 0usize)
		}
	}
}

impl Display for Decision {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		let _ = write!(f, "{:?}", self.result);
		Ok(())
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
