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

use display::event::_Event;
use display::draw;

use board::Tile;
use board::GoBoard;
use board::Team;

pub const CASE_WIDTH: graphics::types::Resolution = 40;

pub const ORANGE: graphics::types::Color = [0.97647065f32, 0.9450981f32, 0.854902f32, 1f32];
pub const BLACK: graphics::types::Color = [0f32, 0f32, 0f32, 1f32];
pub const WHITE: graphics::types::Color = [1f32, 1f32, 1f32, 1f32];

#[derive(Debug)]
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

pub struct Console {
    board: GoBoard,
    event: _Event,
    team: (Team, Team),
    player: [Player; 2],
    turn: bool, // Player one = true, player two = false.
    help: bool,
}

impl Console {

	/// The `new` constructor function returns the interface console.

    fn new (
        board: GoBoard,
        layer: usize,
        player: [Player; 2],
        help: bool,
    ) -> Self {
        let size: u32 = board.get_size() as u32;

		Console {
			board: board,
            event: _Event::new(piston::window::Size::from([CASE_WIDTH * size; 2])),
            team: Team::new_teams(),
            player: player,
            turn: true,
            help: help,
		}
    }

    fn get_size (
        &self
    ) -> piston::window::Size {
        let size: graphics::types::Resolution = self.board.get_size () as graphics::types::Resolution;
        let dimension = self.event.get_dimension();

        piston::window::Size::from([
            dimension.width / size,
            dimension.height / size,
        ])
    }

    fn play (
        &mut self,
        coordinate: piston::window::Size,
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
        let max: u32 = self.board.get_size() as u32;

        for event in window.clone().events() {
            let dimension = self.get_size();

            if let Some(resize) = event.resize(|w, h| [w as u32, h as u32]) {
                self.event.set_dimension(piston::window::Size::from(resize));
            }
            if let Some(coordinate) = event.mouse_cursor(|x, y| {
                piston::window::Size::from([x as u32, y as u32])
            }) {
                self.play(coordinate, max);
            }
            if let Some(Button::Mouse(_)) = event.press_args() {
                self.board.set_pawn_human(self.event.get_coordinate());
            }
            if let Some(args) = event.render_args() {
                gl.draw(args.viewport(), |context, g| {
                    graphics::clear(ORANGE, g);

                    for x in 0..max {
                        for y in 0..max {
                            draw::draw_line_color(BLACK, dimension, [x, y], (&context, g));
                        }
                    }
                    draw::draw_border_color(ORANGE, dimension, max, (&context, g));
                    for x in 0..max {
                        for y in 0..max {
                            match self.board.get((x as usize, y as usize)) {
                                Tile::WHITE => draw::draw_tile_color(BLACK, dimension, [x, y], (&context, g)),
                                Tile::BLACK => draw::draw_tile_color(WHITE, dimension, [x, y], (&context, g)),
                                _ => {},
                            }
                        }
                    }
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
        let size: u32 = board.get_size() as u32;

		Console {
			board: board,
            event: _Event::new(piston::window::Size::from([CASE_WIDTH * size; 2])),
            team: Team::new_teams(),
            player: [Player::Human, Player::Ia],
            turn: true,
            help: false,
		}
    }
}
