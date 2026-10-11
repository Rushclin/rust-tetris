//! Les tétrominos : forme, couleur, position et rotation.

use crate::settings::{COLS, Color};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceKind {
    pub const ALL: [PieceKind; 7] = [
        PieceKind::I,
        PieceKind::O,
        PieceKind::T,
        PieceKind::S,
        PieceKind::Z,
        PieceKind::J,
        PieceKind::L,
    ];

    pub fn color(self) -> Color {
        match self {
            PieceKind::I => [0.0, 0.85, 0.9, 1.0],
            PieceKind::O => [0.95, 0.85, 0.1, 1.0],
            PieceKind::T => [0.65, 0.25, 0.85, 1.0],
            PieceKind::S => [0.2, 0.85, 0.3, 1.0],
            PieceKind::Z => [0.9, 0.2, 0.2, 1.0],
            PieceKind::J => [0.2, 0.4, 0.95, 1.0],
            PieceKind::L => [0.95, 0.55, 0.1, 1.0],
        }
    }

    /// Côté de la boîte carrée dans laquelle la pièce tourne.
    pub fn box_size(self) -> i32 {
        match self {
            PieceKind::I => 4,
            PieceKind::O => 2,
            _ => 3,
        }
    }

    /// Cases occupées dans l'orientation initiale, relatives au coin
    /// haut-gauche de la boîte de rotation.
    fn base_cells(self) -> [(i32, i32); 4] {
        match self {
            PieceKind::I => [(0, 1), (1, 1), (2, 1), (3, 1)],
            PieceKind::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            PieceKind::T => [(1, 0), (0, 1), (1, 1), (2, 1)],
            PieceKind::S => [(1, 0), (2, 0), (0, 1), (1, 1)],
            PieceKind::Z => [(0, 0), (1, 0), (1, 1), (2, 1)],
            PieceKind::J => [(0, 0), (0, 1), (1, 1), (2, 1)],
            PieceKind::L => [(2, 0), (0, 1), (1, 1), (2, 1)],
        }
    }
}

/// Une pièce placée sur le plateau. `x`/`y` désignent le coin haut-gauche
/// de sa boîte de rotation.
#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub x: i32,
    pub y: i32,
    /// Nombre de quarts de tour dans le sens horaire (0 à 3).
    rotation: u8,
}

impl Piece {
    pub fn at(kind: PieceKind, x: i32, y: i32) -> Self {
        Self {
            kind,
            x,
            y,
            rotation: 0,
        }
    }

    /// Pièce centrée en haut du plateau.
    pub fn spawn(kind: PieceKind) -> Self {
        Self::at(kind, (COLS - kind.box_size()) / 2, 0)
    }

    /// Coordonnées absolues des 4 cases occupées.
    pub fn cells(&self) -> [(i32, i32); 4] {
        let n = self.kind.box_size();
        self.kind.base_cells().map(|(cx, cy)| {
            // Quart de tour horaire dans la boîte : (x, y) -> (n - 1 - y, x)
            let (rx, ry) = (0..self.rotation).fold((cx, cy), |(x, y), _| (n - 1 - y, x));
            (self.x + rx, self.y + ry)
        })
    }

    pub fn moved(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            ..*self
        }
    }

    pub fn rotated(&self, clockwise: bool) -> Self {
        let step = if clockwise { 1 } else { 3 };
        Self {
            rotation: (self.rotation + step) % 4,
            ..*self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut cells: [(i32, i32); 4]) -> [(i32, i32); 4] {
        cells.sort();
        cells
    }

    #[test]
    fn four_rotations_return_to_start() {
        for kind in PieceKind::ALL {
            let p = Piece::at(kind, 3, 5);
            let back = (0..4).fold(p, |p, _| p.rotated(true));
            assert_eq!(sorted(p.cells()), sorted(back.cells()));
        }
    }

    #[test]
    fn counter_clockwise_undoes_clockwise() {
        for kind in PieceKind::ALL {
            let p = Piece::at(kind, 0, 0);
            let back = p.rotated(true).rotated(false);
            assert_eq!(sorted(p.cells()), sorted(back.cells()));
        }
    }

    #[test]
    fn o_piece_does_not_change_when_rotated() {
        let p = Piece::at(PieceKind::O, 0, 0);
        assert_eq!(sorted(p.cells()), sorted(p.rotated(true).cells()));
    }
}
