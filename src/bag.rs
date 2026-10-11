//! Tirage des pièces selon le système du « sac de 7 » : chaque série de
//! 7 pièces contient exactement une pièce de chaque type, dans un ordre
//! aléatoire. Cela évite les longues séries sans barre (I).

use crate::piece::PieceKind;
use crate::rng::Rng;

pub struct Bag {
    rng: Rng,
    pieces: Vec<PieceKind>,
}

impl Bag {
    pub fn new(rng: Rng) -> Self {
        Self {
            rng,
            pieces: Vec::with_capacity(PieceKind::ALL.len()),
        }
    }

    pub fn next(&mut self) -> PieceKind {
        if self.pieces.is_empty() {
            self.refill();
        }
        self.pieces.pop().expect("le sac vient d'être rempli")
    }

    /// Remplit le sac et le mélange (algorithme de Fisher-Yates).
    fn refill(&mut self) {
        self.pieces.extend_from_slice(&PieceKind::ALL);
        for i in (1..self.pieces.len()).rev() {
            let j = self.rng.below(i + 1);
            self.pieces.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_series_of_seven_contains_every_piece() {
        let mut bag = Bag::new(Rng::new(42));
        for _ in 0..3 {
            let mut series: Vec<PieceKind> = (0..7).map(|_| bag.next()).collect();
            for kind in PieceKind::ALL {
                let pos = series
                    .iter()
                    .position(|&k| k == kind)
                    .expect("pièce manquante");
                series.remove(pos);
            }
            assert!(series.is_empty());
        }
    }
}
