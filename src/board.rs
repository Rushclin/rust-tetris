//! Le plateau : grille des cases déjà posées, collisions et lignes complètes.

use crate::piece::{Piece, PieceKind};
use crate::settings::{COLS, ROWS};

const WIDTH: usize = COLS as usize;
const HEIGHT: usize = ROWS as usize;

type Row = [Option<PieceKind>; WIDTH];

pub struct Board {
    /// `cells[y][x]`, la ligne 0 étant en haut.
    cells: [Row; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[None; WIDTH]; HEIGHT],
        }
    }

    fn in_bounds(x: i32, y: i32) -> bool {
        (0..COLS).contains(&x) && (0..ROWS).contains(&y)
    }

    /// Vrai si la pièce est entièrement dans le plateau et ne chevauche aucune case posée.
    pub fn fits(&self, piece: &Piece) -> bool {
        piece
            .cells()
            .iter()
            .all(|&(x, y)| Self::in_bounds(x, y) && self.cells[y as usize][x as usize].is_none())
    }

    /// Fige la pièce dans la grille.
    pub fn lock(&mut self, piece: &Piece) {
        for (x, y) in piece.cells() {
            if Self::in_bounds(x, y) {
                self.cells[y as usize][x as usize] = Some(piece.kind);
            }
        }
    }

    /// Supprime les lignes complètes, fait descendre celles du dessus et
    /// renvoie le nombre de lignes supprimées.
    pub fn clear_full_lines(&mut self) -> u32 {
        let mut cleared = 0;
        let mut write = HEIGHT;
        for read in (0..HEIGHT).rev() {
            if self.cells[read].iter().all(Option::is_some) {
                cleared += 1;
            } else {
                write -= 1;
                self.cells[write] = self.cells[read];
            }
        }
        for row in &mut self.cells[..write] {
            *row = [None; WIDTH];
        }
        cleared
    }

    /// Toutes les cases posées, sous la forme `(x, y, type)`.
    pub fn filled_cells(&self) -> impl Iterator<Item = (i32, i32, PieceKind)> + '_ {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, cell)| cell.map(|kind| (x as i32, y as i32, kind)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fill_row(board: &mut Board, y: usize) {
        board.cells[y] = [Some(PieceKind::O); WIDTH];
    }

    #[test]
    fn piece_outside_the_board_does_not_fit() {
        let board = Board::new();
        assert!(board.fits(&Piece::spawn(PieceKind::T)));
        assert!(!board.fits(&Piece::at(PieceKind::T, -1, 0)));
        assert!(!board.fits(&Piece::at(PieceKind::T, COLS - 2, 0)));
        assert!(!board.fits(&Piece::at(PieceKind::T, 0, ROWS - 1)));
    }

    #[test]
    fn piece_does_not_fit_over_locked_cells() {
        let mut board = Board::new();
        let piece = Piece::at(PieceKind::O, 0, 0);
        board.lock(&piece);
        assert!(!board.fits(&piece));
        assert!(board.fits(&piece.moved(2, 0)));
    }

    #[test]
    fn full_lines_are_cleared_and_rows_above_fall() {
        let mut board = Board::new();
        fill_row(&mut board, HEIGHT - 1);
        fill_row(&mut board, HEIGHT - 3);
        board.cells[HEIGHT - 2][0] = Some(PieceKind::T);
        board.cells[HEIGHT - 4][5] = Some(PieceKind::L);

        assert_eq!(board.clear_full_lines(), 2);
        assert_eq!(board.cells[HEIGHT - 1][0], Some(PieceKind::T));
        assert_eq!(board.cells[HEIGHT - 2][5], Some(PieceKind::L));
        assert_eq!(board.filled_cells().count(), 2);
    }
}
