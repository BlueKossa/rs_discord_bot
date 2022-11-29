use std::collections::HashMap;

use crate::commands::chess::board_creator::Board;
use lazy_static::lazy_static;
use serenity::model::prelude::GuildId;

lazy_static! {
    pub static ref BOARD: HashMap<GuildId, Board> = HashMap::new();
}

struct ChessGame {
    board: Board,
    turn: bool,
    players: (Player, Player),
}

struct Player {
    name: String,
    id: u64,
    color: bool,
    check: bool,
}

fn parse_move(move_str: &str) {
    const PIECES: [char; 5] = ['N', 'B', 'R', 'Q', 'K'];
    let mut chars = move_str.chars();
    if let Some(piece) = chars.next() {
        if PIECES.contains(&piece) {}
    }
}

fn pawn_move(move_str: &str) {
    let chars = move_str.chars();
}

fn chess_to_coord((x, y): (char, char)) -> (u8, u8) {
    let x = x as u8 - 97;
    let y = y as u8 - 1;
    (x, y)
}

fn coord_to_chess((x, y): (u8, u8)) -> (char, u8) {
    let x = (x + 97) as char;
    let y = y + 1;
    (x, y)
}
