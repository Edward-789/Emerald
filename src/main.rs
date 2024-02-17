mod board;
mod utils;
mod attacks;
mod magics;
mod moves;
mod perft;

use crate::{
    board::Board,
    perft::perft
};

fn main() {
    let board = Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    perft::<true>(&board, 3);
}