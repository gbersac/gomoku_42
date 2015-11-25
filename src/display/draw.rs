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

use self::graphics::Context;
use self::graphics::Graphics;
use self::graphics::types::Color;
use self::graphics::draw_state::DrawState;
use self::piston::window::Size;

use board::Tile;
use board::GoBoard;

const BORDER_SIZE : f64 = 0.5f64;
const ORANGE: graphics::types::Color = [0.97647065f32, 0.9450981f32, 0.854902f32, 1f32];
const BLACK: graphics::types::Color = [0f32, 0f32, 0f32, 1f32];
const WHITE: graphics::types::Color = [1f32, 1f32, 1f32, 1f32];
const OVER: graphics::types::Color = [1f32, 1f32, 1f32, 0.7f32];

fn draw_tile_color<G> (
    color: Color,
    dimension: Size,
    coordinate: [u32; 2],
    (context, g): (&Context, &mut G),
) where G: Graphics {
    let _coordinate: Size = Size::from(coordinate);

    graphics::ellipse (
        color,
        graphics::ellipse::circle (
            {dimension.width * _coordinate.width + dimension.width / 2u32} as f64,
            {dimension.height * _coordinate.height + dimension.height / 2u32} as f64,
            {std::cmp::min(dimension.width, dimension.height) / 3u32} as f64,
        ),
        context.transform,
        g
    );
}

fn draw_border_color<G> (
    color: Color,
    dimension: Size,
    max: u32,
    (context, g): (&Context, &mut G),
) where G: Graphics {
    let rect_border = graphics::Rectangle::new_border(color, {
        std::cmp::min(dimension.width, dimension.height) / 2u32
    } as f64 - BORDER_SIZE);

    rect_border.draw(
        [
            0f64,
            0f64,
            {dimension.width * max} as f64,
            {dimension.height * max} as f64,
        ],
        &context.draw_state, context.transform, g);
}

fn draw_line_color<G> (
    color: Color,
    dimension: Size,
    coordinate: [u32; 2],
    (context, g): (&Context, &mut G),
) where G: Graphics {
    let _coordinate: Size = Size::from(coordinate);
    let line_border = graphics::Line::new (
        color,
        BORDER_SIZE,
    );

    line_border.draw (
        [
            {dimension.width * _coordinate.width + dimension.width / 2u32} as f64,
            {dimension.height * _coordinate.height} as f64,
            {dimension.width * _coordinate.width + dimension.width / 2u32} as f64,
            {dimension.height * _coordinate.height + dimension.height} as f64,
        ],
        &DrawState::new(),
        context.transform,
        g
    );
    line_border.draw (
        [
            {dimension.width * _coordinate.width} as f64,
            {dimension.height * _coordinate.height + dimension.height / 2u32} as f64,
            {dimension.width * _coordinate.width + dimension.width} as f64,
            {dimension.height * _coordinate.height + dimension.height / 2u32} as f64,
        ],
        &DrawState::new(),
        context.transform,
        g
    );
}

pub fn draw_render<G> (
    board: &GoBoard,
    dimension: Size,
    limit: u32,
    (context, g): (&Context, &mut G),
) where G: Graphics {
    for x in 0..limit {
        for y in 0..limit {
            draw_line_color(BLACK, dimension, [x, y], (&context, g)); //5F5845
        }
    }
    draw_border_color(ORANGE, dimension, limit, (&context, g));
    for x in 0..limit {
        for y in 0..limit {
            match board.get((x as usize, y as usize)) {
                Tile::WHITE => draw_tile_color(BLACK, dimension, [x, y], (&context, g)),
                Tile::BLACK => draw_tile_color(WHITE, dimension, [x, y], (&context, g)),
                _ => {},
            }
        }
    }
}

pub fn draw_over<G> (
    board: &GoBoard,
    dimension: Size,
    (x, y): (u32, u32),
    (context, g): (&Context, &mut G),
) where G: Graphics {
    match board.get((x as usize, y as usize)) {
        Tile::FREE => {
            draw_tile_color(OVER, dimension, [x, y], (&context, g))
        },
        _ => {},
    }
}

pub fn draw_help<G> (
    board: &GoBoard,
    dimension: Size,
    (x, y): (u32, u32),
    (context, g): (&Context, &mut G),
) where G: Graphics {
    match board.get((x as usize, y as usize)) {
        Tile::FREE => {
            draw_tile_color(OVER, dimension, [x, y], (&context, g))
        },
        _ => {},
    }
}
