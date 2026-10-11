use piston_window::graphics::{clear, rectangle};
use piston_window::*;

use settings::*;
mod settings;

fn main() {
    // declare windows title
    let w_title = format!("Jeu de tetris (juste pour apprentissage) {}", VERSION);
    let mut window: PistonWindow = WindowSettings::new(
        w_title,
        [(COLS as f64 * CELL) as u32, (ROWS as f64 * CELL) as u32],
    )
    .exit_on_esc(true)
    .vsync(true)
    .build()
    .unwrap();

    let (mut x, mut y) = (COLS / 2, 0);
    let mut timer = 0.0;

    while let Some(event) = window.next() {
        // Entrées clavier
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Left if x > 0 => x -= 1,
                Key::Right if x < COLS - 1 => x += 1,
                _ => {}
            }
        }

        // Logique : chute toutes les 0,5 s
        if let Some(u) = event.update_args() {
            timer += u.dt;
            if timer >= 0.5 {
                timer = 0.0;
                if y < ROWS - 1 {
                    y += 1;
                } else {
                    y = 0; // retour en haut
                }
            }
        }

        // Affichage
        window.draw_2d(&event, |c, g, _| {
            clear([0.1, 0.1, 0.1, 1.0], g);
            rectangle(
                [0.9, 0.3, 0.2, 1.0],
                [x as f64 * CELL, y as f64 * CELL, CELL - 1.0, CELL - 1.0],
                c.transform,
                g,
            );
        });
    }

    // println!("Hello, world!");
}
