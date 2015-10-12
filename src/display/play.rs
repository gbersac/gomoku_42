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

use self::opengl_graphics::{ GlGraphics, OpenGL };
use self::graphics::Context;
use self::graphics::Graphics;
use self::graphics::types::{Resolution, Color};
use self::graphics::line::Shape;
use self::graphics::draw_state::{DrawState};
use std::rc::Rc;
use std::collections::HashMap;
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

use display::{draw};

use board::GoBoard;
use board::tile::{Tile};

pub const CASE_WIDTH : Resolution = 40;
pub const BORDER_SIZE : f64 = 1f64;

pub const ORANGE : Color = [1f32, 0.5f32, 0f32, 1f32];
pub const BLACK  : Color = [0f32, 0f32, 0f32, 1f32];
pub const WHITE  : Color = [1f32, 1f32, 1f32, 1f32];
pub const INVISIBLE : Color = [0f32; 4];

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
    board: GoBoard,
    dimension: Size,
}

impl Play {

    fn get_size (
        &self
    ) -> Size {
        let size: Resolution = self.board.get_size() as Resolution;

        Size::from([
            self.dimension.width / size,
            self.dimension.height / size
        ])
    }

    pub fn start (
        &mut self,
    ) {
        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new (
            "Gomoku",
            self.dimension
        ).exit_on_esc(true).opengl(opengl).build().unwrap();
        let mut capture_cursor = false;
        let window = Rc::new(RefCell::new(window));
        let ref mut gl = GlGraphics::new(opengl);
        let mut cursor = [0.0, 0.0];
        let max: u32 = self.board.get_size() as u32;

        for event in window.clone().events() {
            let dimension = self.get_size();

            if let Some(resize) = event.resize(|w, h| [h as u32, w as u32]) {
                self.dimension = Size::from(resize);
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
                            draw::draw_tile_color(WHITE, dimension, [x, y], (&context, g));
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
			board: board,
            dimension: Size::from([CASE_WIDTH * size; 2]),
		}
    }
}
