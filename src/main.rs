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
    let board = Board::read_fen("rnbqkbnr/pppp1ppp/4p3/8/8/BP6/P1PPPPPP/RN1QKBNR b KQkq - 1 2");

    perft::<true>(&board, 1);
}