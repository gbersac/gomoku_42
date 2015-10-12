pub struct Event {
  overed: bool,
  old_cell: (usize, usize),
  win_size: (f64, f64),
}

impl Event {

  /// The `new` constructor function returns the event interface.

  pub fn new (
    size: u32,
  ) -> Self {
    GoEvent {
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

    self.over(event);
    self.active(event);
  }
}
