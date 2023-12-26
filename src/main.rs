mod board;
mod consts;


use crate::board::Board;

fn main() { 
    let board = Board::read_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -");
    board.print_board_info()
}
