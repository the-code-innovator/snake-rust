use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_co_ordinate(game_co_ordinate: i32) -> f64 {
	(game_co_ordinate as f64) * BLOCK_SIZE
}

pub fn to_co_ordinate_u32(game_co_ordinate: i32) -> u32 {
	to_co_ordinate(game_co_ordinate) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, graphics: &mut G2d) {
	let gui_x = to_co_ordinate(x);
	let gui_y = to_co_ordinate(y);
	rectangle(
		color,
		[gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
		context.transform,
		graphics
	);
}

pub fn draw_rectangle(
	color: Color, x: i32, y: i32, width: i32, height: i32, context: &Context, graphics: &mut G2d
) {
	let x = to_co_ordinate(x);
	let y = to_co_ordinate(y);
	rectangle(
		color,
		[
			x,
			y,
			BLOCK_SIZE * (width as f64),
			BLOCK_SIZE * (height as f64)
		],
		context.transform,
		graphics
	);
}