extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::types::Color;
use piston_window::*;
use game::Game;
use draw::to_co_ordinate_u32;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow = 
        WindowSettings::new(
            "Snake",
            [to_co_ordinate_u32(width), to_co_ordinate_u32(height)]
        ).exit_on_esc(true).build().unwrap();
    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |context, graphics, _| {
            clear(BACKGROUND_COLOR, graphics);
            game.draw(&context, graphics);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}