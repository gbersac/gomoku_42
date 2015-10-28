extern crate std;
extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

#[cfg(feature = "include_sdl2")]
use self::sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]
use self::glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]
use self::glutin_window::GlutinWindow as Window;

use self::graphics::Transformed;

use self::piston::input::*;
use self::piston::event_loop::*;

use display::mouse::Mouse;
use display::draw;

use board::Tile;
use board::GoBoard;
use board::Team;

use ia::decision;

pub const CASE_WIDTH: graphics::types::Resolution = 40;

#[derive(Debug, PartialEq, Clone)]
pub enum Player {
    Human,
    Ia,
}

impl Player {
	pub fn from_str(s: &str) -> Player {
	    match s {
	        "ia"	=> Player::Ia,
	        "human"	=> Player::Human,
	        _		=> panic!("Player cli option must be either ia, solo or multi")
	    }
	}
}

#[derive(Debug, Clone)]
pub struct Console {
    board: GoBoard,
    event: Mouse,
    player: (Team, Player),
    friend: (Team, Player),
    layer: u32,
    turn: bool, // Player one = true, player two = false.
    help: bool,
}

impl Console {

	/// The `new` constructor function returns the interface console.

    pub fn new (
        board: GoBoard,
        layer: u32,
        (player, friend): (Player, Player),
        help: bool,
    ) -> Self {
        let size: u32 = board.get_size() as u32;
        let (team_player, team_friend) = Team::new_teams();

		Console {
			board: board,
            event: Mouse::new((CASE_WIDTH * size, CASE_WIDTH * size)),
            player: (team_player, player),
            friend: (team_friend, friend),
            turn: true,
            layer: layer,
            help: help,
		}
    }

    fn get_size (
        &self
    ) -> piston::window::Size {
        let size: graphics::types::Resolution = self.board.get_size (
        ) as graphics::types::Resolution;
        let dimension = self.event.get_dimension();

        piston::window::Size::from([
            dimension.0 / size,
            dimension.1 / size,
        ])
    }

    fn play (
        &mut self,
        coordinate: (u32, u32),
        length: u32,
    ) {
        if let Some(coordinate) = self.event.check_inside_window (
            coordinate,
            length,
        ) {
            self.event.set_coordinate(coordinate);
        }
    }

    pub fn start (
        &mut self,
    ) {
        let opengl = opengl_graphics::OpenGL::V3_2;
        let window: Window = piston::window::WindowSettings::new (
            "Gomoku",
            self.event.get_dimension(),
        ).exit_on_esc(true).opengl(opengl).build().unwrap();
        let window = std::rc::Rc::new(std::cell::RefCell::new(window));
        let ref mut gl = opengl_graphics::GlGraphics::new(opengl);
        let limit: u32 = self.board.get_size() as u32;

        for event in window.clone().events() {
            let dimension = self.get_size();

            if let Some(resize) = event.resize(|w, h| (w as u32, h as u32)) {
                self.event.set_dimension(resize);
            }
            if let Some(coordinate) = event.mouse_cursor(|x, y| {
                (x as u32, y as u32)
            }) {
                self.play(coordinate, limit);
            }
            match (self.turn, self.player.clone(), self.friend.clone()) {
                (true, (team, Player::Ia), (_, _)) => {
                    let position = decision::get_optimal_move (
                        &self.board,
                        &team,
                        self.layer
                    );
                    self.board.set_raw (
                        (
                            position.0 as usize,
                            position.1 as usize
                        ),
                        self.player.0.get_tile()
                    );
                    self.turn = !self.turn;
                },
                (false, (_, _), (team, Player::Ia)) => {
                    let position = decision::get_optimal_move (
                        &self.board,
                        &team,
                        self.layer
                    );
                    self.board.set_raw (
                        (
                            position.0 as usize,
                            position.1 as usize
                        ),
                        self.friend.0.get_tile()
                    );
                    self.turn = !self.turn;
                },
                (true, (_, Player::Human), (_, _)) => {
                    if let Some(Button::Mouse(_)) = event.press_args() {
                        if self.board.set(self.event.get_coordinate(), &mut self.player.0) {
                            self.turn = !self.turn;
                        }
                    }
                },
                (false, (_, _), (_, Player::Human)) => {
                    if let Some(Button::Mouse(_)) = event.press_args() {
                        if self.board.set(self.event.get_coordinate(), &mut self.friend.0) {
                            self.turn = !self.turn;
                        }
                    }
                },
            }
            if let Some(args) = event.render_args() {
                gl.draw(args.viewport(), |context, g| {
                    draw::draw_render(&self.board, dimension, limit, (&context, g));
                });
            }
            event.update(|_| {});
        }
    }
}

impl Default for Console {

	/// The `new` constructor function returns the interface console.

    fn default () -> Self {
        let board: GoBoard = Default::default();
        let (team_player, team_friend) = Team::new_teams();
        let size: u32 = board.get_size() as u32;

		Console {
			board: board,
            event: Mouse::new((CASE_WIDTH * size, CASE_WIDTH * size)),
            player: (team_player, Player::Human),
            friend: (team_friend, Player::Ia),
            layer: 3,
            turn: true,
            help: false,
		}
    }
}
