//! Affichage : plateau, pièces, panneau latéral et écrans de pause / fin.
//! Ce module se contente de lire l'état du jeu, il ne le modifie jamais.

use piston_window::graphics::{Context, Graphics, clear, rectangle};

use crate::font::{draw_text, text_width};
use crate::game::{Game, State};
use crate::piece::{Piece, PieceKind};
use crate::settings::*;

pub fn draw<G: Graphics>(game: &Game, c: Context, g: &mut G) {
    clear(BACKGROUND, g);
    draw_board(game, c, g);
    draw_sidebar(game, c, g);
    match game.state() {
        State::Playing => {}
        State::Paused => draw_overlay("PAUSE", "P: REPRENDRE", c, g),
        State::GameOver => draw_overlay("GAME OVER", "R: REJOUER", c, g),
    }
}

fn draw_board<G: Graphics>(game: &Game, c: Context, g: &mut G) {
    // Fond quadrillé
    for y in 0..ROWS {
        for x in 0..COLS {
            draw_board_cell(x, y, BOARD_CELL, c, g);
        }
    }

    for (x, y, kind) in game.board().filled_cells() {
        draw_board_cell(x, y, kind.color(), c, g);
    }

    if game.state() != State::GameOver {
        let mut ghost_color = game.current().kind.color();
        ghost_color[3] = GHOST_ALPHA;
        draw_piece(&game.ghost(), ghost_color, c, g);
        draw_piece(game.current(), game.current().kind.color(), c, g);
    }
}

fn draw_sidebar<G: Graphics>(game: &Game, c: Context, g: &mut G) {
    let x0 = BOARD_WIDTH;
    let width = SIDEBAR_COLS as f64 * CELL;
    let margin = 15.0;
    let left = x0 + margin;
    rectangle(
        SIDEBAR_BG,
        [x0, 0.0, width, WINDOW_HEIGHT as f64],
        c.transform,
        g,
    );

    draw_text("SUIVANT", left, 20.0, 4.0, TEXT_DIM, c, g);
    draw_preview(game.next(), x0, 50.0, width, c, g);

    let stats = [
        ("SCORE", game.score()),
        ("NIVEAU", game.level()),
        ("LIGNES", game.lines()),
    ];
    for (i, (label, value)) in stats.iter().enumerate() {
        let y = 160.0 + i as f64 * 75.0;
        draw_text(label, left, y, 4.0, TEXT_DIM, c, g);
        draw_text(&value.to_string(), left, y + 28.0, 5.0, TEXT, c, g);
    }

    let help = [
        "FLECHES: BOUGER",
        "HAUT: TOURNER",
        "BAS: DESCENDRE",
        "ESPACE: CHUTE",
        "P: PAUSE",
        "ECHAP: QUITTER",
    ];
    let help_top = WINDOW_HEIGHT as f64 - margin - help.len() as f64 * 16.0;
    for (i, line) in help.iter().enumerate() {
        draw_text(line, left, help_top + i as f64 * 16.0, 2.0, TEXT_DIM, c, g);
    }
}

/// Aperçu de la pièce suivante, centré horizontalement dans le panneau.
fn draw_preview<G: Graphics>(
    kind: PieceKind,
    x0: f64,
    top: f64,
    width: f64,
    c: Context,
    g: &mut G,
) {
    let size = CELL * 0.8;
    let piece = Piece::at(kind, 0, 0);
    let cells = piece.cells();

    let min_x = cells.iter().map(|&(x, _)| x).min().unwrap_or(0);
    let max_x = cells.iter().map(|&(x, _)| x).max().unwrap_or(0);
    let min_y = cells.iter().map(|&(_, y)| y).min().unwrap_or(0);
    let piece_width = (max_x - min_x + 1) as f64 * size;
    let left = x0 + (width - piece_width) / 2.0;

    for (x, y) in cells {
        let px = left + (x - min_x) as f64 * size;
        let py = top + (y - min_y) as f64 * size;
        draw_square(px, py, size, kind.color(), c, g);
    }
}

/// Voile sombre sur le plateau avec un titre et une indication.
fn draw_overlay<G: Graphics>(title: &str, hint: &str, c: Context, g: &mut G) {
    let height = WINDOW_HEIGHT as f64;
    rectangle(OVERLAY, [0.0, 0.0, BOARD_WIDTH, height], c.transform, g);

    let (title_px, hint_px) = (6.0, 3.0);
    let center_y = height / 2.0;
    let title_x = (BOARD_WIDTH - text_width(title, title_px)) / 2.0;
    let hint_x = (BOARD_WIDTH - text_width(hint, hint_px)) / 2.0;
    draw_text(title, title_x, center_y - 40.0, title_px, TEXT, c, g);
    draw_text(hint, hint_x, center_y + 10.0, hint_px, TEXT_DIM, c, g);
}

fn draw_piece<G: Graphics>(piece: &Piece, color: Color, c: Context, g: &mut G) {
    for (x, y) in piece.cells() {
        draw_board_cell(x, y, color, c, g);
    }
}

fn draw_board_cell<G: Graphics>(x: i32, y: i32, color: Color, c: Context, g: &mut G) {
    draw_square(x as f64 * CELL, y as f64 * CELL, CELL, color, c, g);
}

/// Carré avec une marge d'un pixel, ce qui dessine le quadrillage.
fn draw_square<G: Graphics>(x: f64, y: f64, size: f64, color: Color, c: Context, g: &mut G) {
    rectangle(
        color,
        [x + 1.0, y + 1.0, size - 2.0, size - 2.0],
        c.transform,
        g,
    );
}
