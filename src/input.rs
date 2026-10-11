//! Correspondance entre les touches du clavier et les actions du jeu.

use piston_window::Key;

use crate::game::Action;

pub fn action_for(key: Key) -> Option<Action> {
    let action = match key {
        Key::Left | Key::A => Action::MoveLeft,
        Key::Right | Key::D => Action::MoveRight,
        Key::Down | Key::S => Action::SoftDrop,
        Key::Space => Action::HardDrop,
        Key::Up | Key::W | Key::X => Action::RotateClockwise,
        Key::Z => Action::RotateCounterClockwise,
        Key::P => Action::TogglePause,
        Key::R | Key::Return => Action::Restart,
        _ => return None,
    };
    Some(action)
}
