//! Logique de jeu : pièce courante, gravité, score, niveaux et fin de partie.
//! Ce module ne connaît ni la fenêtre ni le clavier : il reçoit des
//! `Action` et le temps écoulé.

use crate::bag::Bag;
use crate::board::Board;
use crate::piece::{Piece, PieceKind};
use crate::rng::Rng;
use crate::settings::{
    HARD_DROP_SCORE, LINE_SCORES, LINES_PER_LEVEL, MIN_FALL_INTERVAL, SOFT_DROP_SCORE,
};

/// Décalages essayés quand une rotation est bloquée (« wall kicks » simplifiés).
const ROTATION_KICKS: [(i32, i32); 6] = [(0, 0), (-1, 0), (1, 0), (-2, 0), (2, 0), (0, -1)];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateClockwise,
    RotateCounterClockwise,
    TogglePause,
    Restart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    board: Board,
    bag: Bag,
    current: Piece,
    next: PieceKind,
    state: State,
    score: u32,
    lines: u32,
    level: u32,
    fall_timer: f64,
}

impl Game {
    pub fn new() -> Self {
        let mut bag = Bag::new(Rng::from_time());
        let current = Piece::spawn(bag.next());
        let next = bag.next();
        Self {
            board: Board::new(),
            bag,
            current,
            next,
            state: State::Playing,
            score: 0,
            lines: 0,
            level: 1,
            fall_timer: 0.0,
        }
    }

    // --- Lecture de l'état (pour l'affichage) ---------------------------

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current(&self) -> &Piece {
        &self.current
    }

    pub fn next(&self) -> PieceKind {
        self.next
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn lines(&self) -> u32 {
        self.lines
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    /// Position où la pièce courante atterrirait en chute instantanée.
    pub fn ghost(&self) -> Piece {
        let mut ghost = self.current;
        while self.board.fits(&ghost.moved(0, 1)) {
            ghost = ghost.moved(0, 1);
        }
        ghost
    }

    // --- Évolution de l'état ---------------------------------------------

    pub fn handle(&mut self, action: Action) {
        match (self.state, action) {
            (State::Paused | State::GameOver, Action::Restart) => *self = Game::new(),
            (State::Playing, Action::TogglePause) => self.state = State::Paused,
            (State::Paused, Action::TogglePause) => self.state = State::Playing,
            (State::Playing, action) => self.apply(action),
            _ => {}
        }
    }

    /// Fait avancer la gravité de `dt` secondes.
    pub fn update(&mut self, dt: f64) {
        if self.state != State::Playing {
            return;
        }
        self.fall_timer += dt;
        let interval = self.fall_interval();
        if self.fall_timer >= interval {
            self.fall_timer -= interval;
            if !self.try_move(0, 1) {
                self.lock_current();
            }
        }
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::MoveLeft => {
                self.try_move(-1, 0);
            }
            Action::MoveRight => {
                self.try_move(1, 0);
            }
            Action::SoftDrop => {
                if self.try_move(0, 1) {
                    self.score += SOFT_DROP_SCORE;
                    self.fall_timer = 0.0;
                } else {
                    self.lock_current();
                }
            }
            Action::HardDrop => {
                let mut distance = 0;
                while self.try_move(0, 1) {
                    distance += 1;
                }
                self.score += distance * HARD_DROP_SCORE;
                self.lock_current();
            }
            Action::RotateClockwise => self.try_rotate(true),
            Action::RotateCounterClockwise => self.try_rotate(false),
            Action::TogglePause | Action::Restart => {}
        }
    }

    /// Temps entre deux descentes automatiques : accélère avec le niveau.
    fn fall_interval(&self) -> f64 {
        let l = (self.level - 1) as f64;
        (0.8 - l * 0.007).powf(l).max(MIN_FALL_INTERVAL)
    }

    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let moved = self.current.moved(dx, dy);
        let fits = self.board.fits(&moved);
        if fits {
            self.current = moved;
        }
        fits
    }

    fn try_rotate(&mut self, clockwise: bool) {
        let rotated = self.current.rotated(clockwise);
        if let Some(piece) = ROTATION_KICKS
            .iter()
            .map(|&(dx, dy)| rotated.moved(dx, dy))
            .find(|p| self.board.fits(p))
        {
            self.current = piece;
        }
    }

    /// Fige la pièce courante, compte les lignes et fait apparaître la suivante.
    fn lock_current(&mut self) {
        self.board.lock(&self.current);

        let cleared = self.board.clear_full_lines();
        if cleared > 0 {
            self.score += LINE_SCORES[cleared as usize] * self.level;
            self.lines += cleared;
            self.level = 1 + self.lines / LINES_PER_LEVEL;
        }

        self.current = Piece::spawn(self.next);
        self.next = self.bag.next();
        self.fall_timer = 0.0;
        if !self.board.fits(&self.current) {
            self.state = State::GameOver;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hard_drop_lands_on_ghost_position_and_scores() {
        let mut game = Game::new();
        let ghost = game.ghost();
        let distance = (ghost.y - game.current().y) as u32;
        let kind = game.current().kind;

        game.handle(Action::HardDrop);

        assert_eq!(game.score(), distance * HARD_DROP_SCORE);
        for (x, y) in ghost.cells() {
            assert!(game.board().filled_cells().any(|c| c == (x, y, kind)));
        }
    }

    #[test]
    fn paused_game_ignores_gravity_and_moves() {
        let mut game = Game::new();
        game.handle(Action::TogglePause);
        let before = *game.current();

        game.update(10.0);
        game.handle(Action::MoveLeft);

        assert_eq!(game.state(), State::Paused);
        assert_eq!(game.current().cells(), before.cells());
    }

    #[test]
    fn stacking_pieces_eventually_ends_the_game() {
        let mut game = Game::new();
        for _ in 0..200 {
            if game.state() == State::GameOver {
                return;
            }
            game.handle(Action::HardDrop);
        }
        panic!("la partie aurait dû se terminer");
    }
}
