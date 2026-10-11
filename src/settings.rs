//! Paramètres globaux du jeu : dimensions, règles de score et couleurs.

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// --- Dimensions ---------------------------------------------------------

/// Taille d'une case en pixels.
pub const CELL: f64 = 30.0;
/// Nombre de colonnes du plateau.
pub const COLS: i32 = 10;
/// Nombre de lignes du plateau.
pub const ROWS: i32 = 20;
/// Largeur du panneau latéral (pièce suivante, score…), en cases.
pub const SIDEBAR_COLS: i32 = 6;

pub const BOARD_WIDTH: f64 = COLS as f64 * CELL;
pub const WINDOW_WIDTH: u32 = ((COLS + SIDEBAR_COLS) as f64 * CELL) as u32;
pub const WINDOW_HEIGHT: u32 = (ROWS as f64 * CELL) as u32;

// --- Règles -------------------------------------------------------------

/// Nombre de lignes à compléter pour passer au niveau suivant.
pub const LINES_PER_LEVEL: u32 = 10;
/// Intervalle de chute le plus court possible, en secondes.
pub const MIN_FALL_INTERVAL: f64 = 0.05;
/// Points pour 0, 1, 2, 3 ou 4 lignes complétées d'un coup (multipliés par le niveau).
pub const LINE_SCORES: [u32; 5] = [0, 100, 300, 500, 800];
/// Points par case descendue en chute accélérée (flèche bas).
pub const SOFT_DROP_SCORE: u32 = 1;
/// Points par case descendue en chute instantanée (espace).
pub const HARD_DROP_SCORE: u32 = 2;

// --- Couleurs -----------------------------------------------------------

pub type Color = [f32; 4];

pub const BACKGROUND: Color = [0.06, 0.06, 0.08, 1.0];
pub const BOARD_CELL: Color = [0.12, 0.12, 0.15, 1.0];
pub const SIDEBAR_BG: Color = [0.09, 0.09, 0.12, 1.0];
pub const TEXT: Color = [0.95, 0.95, 0.95, 1.0];
pub const TEXT_DIM: Color = [0.55, 0.55, 0.6, 1.0];
pub const OVERLAY: Color = [0.0, 0.0, 0.0, 0.7];
/// Opacité de la pièce fantôme (aperçu de l'atterrissage).
pub const GHOST_ALPHA: f32 = 0.25;
