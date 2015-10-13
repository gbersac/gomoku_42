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

pub const BORDER_SIZE : f64 = 0.5f64;

pub fn draw_tile_color<G: Graphics> (
    color: Color,
    dimension: Size,
    coordinate: [u32; 2],
    (context, g): (&Context, &mut G),
) {
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

pub fn draw_border_color<G: Graphics> (
    color: Color,
    dimension: Size,
    max: u32,
    (context, g): (&Context, &mut G),
) {
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

pub fn draw_line_color<G: Graphics> (
    color: Color,
    dimension: Size,
    coordinate: [u32; 2],
    (context, g): (&Context, &mut G),
) {
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
