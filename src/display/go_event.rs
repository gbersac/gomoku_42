extern crate piston_window;

use self::piston_window::*;
use board::GoBoard;
use board::tile::{Tile};

pub struct GoEvent {
  board: GoBoard,
  overed: bool,
  old_cell: (usize, usize),
  win_size: (f64, f64),
}

impl GoEvent {

  /// The `new` constructor function returns the event interface.

  pub fn new (
    board: GoBoard,
    size: u32,
  ) -> Self {
    GoEvent {
      board: board,
      overed: false,
      old_cell: (0usize, 0usize),
      win_size: (size as f64, size as f64),
    }
  }

  /// The `active` function drops a WHITE pawn.

  pub fn active (
    &mut self,
    GoEvent: &PistonWindow,
  ) {
    if self.overed {
      if let Some(Button::Mouse(button)) = GoEvent.press_args() {
        self.board.set_pawn_human(self.old_cell);
      }
    }
  }

  /// The `active` function moves a WHITE pawn.

  pub fn over (
    &mut self,
    GoEvent: &PistonWindow,
  ) {
    let (win_x, win_y):(f64, f64) = self.win_size;
    let tile_size:f64 = self.board.get_size() as f64;

    match GoEvent.mouse_cursor(|x, y| {(
      x as f64,
      y as f64
    )}) {
      Some((x, y)) if 0f64 <= x && x < win_x && 0f64 <= y && y < win_y => {
        self.overed = true;
        match (
          {x / {win_x / tile_size}}.trunc() as usize,
          {y / {win_y / tile_size}}.trunc() as usize
        ) {
          new_cell if new_cell != self.old_cell => if self.board.set_over (
            new_cell,
            self.old_cell
          ) {
            self.old_cell = new_cell;
          },
          _ => {}, // continue to over the same tile.
        }
      },
      Some((_, _)) => self.overed = false, // out of the overzone.
      None => {}, // haven't moved.
    }
  }

  /// The `listen` function loops when the program isn't end
  /// and runs GoEvent when the mouse is clicked.

  pub fn listen (
    &mut self,
    event: &PistonWindow,
  ) {
    let tile_size = self.board.get_size();
    let (win_x, win_y):(f64, f64) = self.win_size;
    let (tile_size_x, tile_size_y):(f64, f64) = ({win_x / tile_size as f64}, {win_y / tile_size as f64});

    event.draw_2d(|c, g| {
      clear([1.0; 4], g);

      for x in 0..tile_size {
          for y in 0..tile_size {
              rectangle (
                  match self.board.get((x, y)) {
                    Tile::BLACK => [1.0, 0.0, 0.0, 1.0],
                    Tile::WHITE => [1.0, 0.0, 3.0, 1.0],
                    Tile::FREE  => [1.0, 9.0, 0.0, 1.0],
                    Tile::OVER  => [1.0, 22.0, 11.0, 1.0],
                  },
                  [tile_size_x * x as f64, tile_size_y * y as f64, tile_size_x, tile_size_y],
                  c.transform,
                  g
              );
          }
      }
    });
    self.over(event);
    self.active(event);
  }
}
