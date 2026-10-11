//! Générateur pseudo-aléatoire minimal (xorshift64), suffisant pour un jeu
//! et sans dépendance externe.

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        // xorshift reste bloqué sur 0 : on impose une graine non nulle.
        Self { state: seed.max(1) }
    }

    /// Graine tirée de l'horloge système, pour une partie différente à chaque lancement.
    pub fn from_time() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        Self::new(nanos)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Entier dans l'intervalle `0..n`.
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}
