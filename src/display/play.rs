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

use self::opengl_graphics::{GlGraphics, OpenGL};
use self::graphics::types::{Resolution, Color};
use std::rc::Rc;
use std::cell::RefCell;
use self::piston::window::{AdvancedWindow, WindowSettings, Size};
use self::piston::input::*;
use self::piston::event_loop::*;
#[cfg(feature = "include_sdl2")]
use self::sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]
use self::glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]
use self::glutin_window::GlutinWindow as Window;

use display::event::_Event;
use display::draw;

use board::Tile;
use board::GoBoard;
use board::Team;

pub const CASE_WIDTH: Resolution = 40;

pub const ORANGE: Color = [1f32, 0.5f32, 0f32, 1f32];
pub const BLACK: Color = [0f32, 0f32, 0f32, 1f32];
pub const WHITE: Color = [1f32, 1f32, 1f32, 1f32];

#[allow(dead_code)]
pub enum Status {
    PLAY,
    LOSS,
    WIN,
}

impl Default for Status {
    fn default() -> Self {
        Status::PLAY
    }
}

pub struct Play {
    status: Status,
    event: _Event,
    board: GoBoard,
    team: [Team; 2],
}

impl Play {

    fn get_size (
        &self
    ) -> Size {
        let size: Resolution = self.board.get_size() as Resolution;
        let dimension = self.event.get_dimension();

        Size::from([
            dimension.width / size,
            dimension.height / size,
        ])
    }

    pub fn start (
        &mut self,
    ) {
        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new (
            "Gomoku",
            self.event.get_dimension(),
        ).exit_on_esc(true).opengl(opengl).build().unwrap();
        let window = Rc::new(RefCell::new(window));
        let ref mut gl = GlGraphics::new(opengl);
        let max: u32 = self.board.get_size() as u32;

        for event in window.clone().events() {
            let dimension = self.get_size();

            if let Some(resize) = event.resize(|w, h| [w as u32, h as u32]) {
                self.event.set_dimension(Size::from(resize));
            }
            if let Some(coordinate) = event.mouse_cursor(|x, y| {
                Size::from([x as u32, y as u32])
            }) {
                if let Some(coordinate) = self.event.check_inside_window(coordinate, max) {
                    self.event.set_coordinate(coordinate);
                }
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

impl Default for Play {

	/// The `new` constructor function returns the interface play.

    fn default() -> Self {
        let board: GoBoard = Default::default();
        let size: u32 = board.get_size() as u32;

		Play {
			status: Default::default(),
            event: _Event::new(Size::from([CASE_WIDTH * size; 2])),
			board: board,
            team: Team::new_teams(),
		}
    }
}
