extern crate piston_window;

use self::piston_window::*;
use board::GoBoard;

pub struct _Event {
  board: GoBoard,
  overed: bool,
  last_pos: (usize, usize),
  win_size: (f64, f64),
}

impl _Event {

  /// The `new` constructor function returns the draw interface.

  pub fn new (
    board: GoBoard,
    size: u32,
  ) -> Self {
    _Event {
      board: board,
      overed: false,
      last_pos: (0usize, 0usize),
      win_size: (size as f64, size as f64),
    }
  }

  pub fn active (
    &self,
    _Event: &PistonWindow,
  ) {
    if self.overed {
      if let Some(Button::Mouse(button)) = _Event.press_args() {
        let (x, y):(usize, usize) = self.last_pos;

        println!("Have actived [{}; {}]", x, y);
      }
    }
  }

  pub fn over (
    &mut self,
    _Event: &PistonWindow,
  ) {
    let (win_x, win_y):(f64, f64) = self.win_size;

    if let Some((x, y)) = _Event.mouse_cursor(|x, y| {(x, y)}) {
      if 0f64 <= x && x < win_x
      && 0f64 <= y && y < win_y {
        let tile_size:f64 = self.board.get_size() as f64;
        let size_x:f64 = win_x / tile_size;
        let size_y:f64 = win_y / tile_size;
        let last_pos_x:usize = {x / size_x}.trunc() as usize;
        let last_pos_y:usize = {y / size_y}.trunc() as usize;
        println!("Have overed [{}; {}]",
          last_pos_x,
          last_pos_y
        );
        self.last_pos = (last_pos_x, last_pos_y);
        self.overed = true;
      }
      else {
        self.overed = false;
      }
    }
  }

  /// The `listen` function loops when the program isn't end
  /// and runs _Event when the mouse is clicked.

  pub fn listen (
    &mut self,
    _Event: &PistonWindow,
  ) {
    _Event.draw_2d(|c, g| {
      clear([1.0; 4], g);
    });
    self.over(_Event);
    self.active(_Event);
  }
}


pub fn main(board: GoBoard) {
  let plaid:u32 = 30;
  let limit:u32 = board.get_size() as u32;
  let size = plaid * limit;
  let mut _event = _Event::new(board, size);
  let mut window: PistonWindow = WindowSettings::new("Gomoku", [
    size,
    size
  ]).exit_on_esc(true).build().unwrap_or_else(|e| {
    panic!("Failed to build PistonWindow: {}", e)
  });

  for e in window {
    _event.listen(&e);
  }
}
