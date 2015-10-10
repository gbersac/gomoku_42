mod go_event;

pub use self::go_event::{GoEvent};

extern crate piston_window;

use self::piston_window::*;
use board::{GoBoard, Team};


pub fn main(board: GoBoard, teams: &mut (Team, Team)) {
  let plaid:u32 = 30;
  let limit:u32 = board.get_size() as u32;
  let size = plaid * limit;
  let mut GoEvent = GoEvent::new(board, size);
  let mut window: PistonWindow = WindowSettings::new("Gomoku", [
    size,
    size
  ]).exit_on_esc(true).build().unwrap_or_else(|e| {
    panic!("Failed to build PistonWindow: {}", e)
  });

  for e in window {
    GoEvent.listen(&e);
  }
}
