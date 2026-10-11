use piston_window::*;

use crate::game::Game;
use crate::settings::{VERSION, WINDOW_HEIGHT, WINDOW_WIDTH};

mod bag;
mod board;
mod font;
mod game;
mod input;
mod piece;
mod rng;
mod settings;
mod ui;

fn main() {
    // declare windows title
    let w_title = format!("Jeu de tetris (juste pour apprentissage) {}", VERSION);
    let mut window: PistonWindow = WindowSettings::new(w_title, [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| panic!("Impossible de créer la fenêtre : {e}"));

    let mut game = Game::new();

    while let Some(event) = window.next() {
        // Entrées clavier
        if let Some(Button::Keyboard(key)) = event.press_args()
            && let Some(action) = input::action_for(key)
        {
            game.handle(action);
        }

        // Logique
        if let Some(u) = event.update_args() {
            game.update(u.dt);
        }

        // Affichage
        window.draw_2d(&event, |c, g, _| ui::draw(&game, c, g));
    }
}
