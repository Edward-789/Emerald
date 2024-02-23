use std::time::Instant;

use crate::{
    board::Board,
    perftsuite::PERFT_SUITE
};

pub fn perft<const ROOT : bool, const SPLIT : bool>(board : &Board, depth : u8) -> u64 {
    let movelist = board.psuedolegal_movegen();

    let mut positions = 0;
    let leaf = depth == 1;

    for i in 0..movelist.length {
        let mut next_board = *board;

        if next_board.apply(movelist.moves[i]) {

            let num = if leaf {1} else {
                perft::<false, false>(&next_board, depth - 1)
            };

            positions += num;

            
            if ROOT && SPLIT {
                println!("{}: {num}", movelist.moves[i].to_uci());
            }
        }
    }

    positions
}

pub fn run_perft_suite() {
    let mut total_nodes = 0;
    let start_time = Instant::now();
    for i in 0..755 {
        let test = &PERFT_SUITE[i];
        let board = Board::read_fen(test.fen);
        let nodes = perft::<true, false>(&board, test.depth);
        if nodes != test.nodes {
            panic!("{}{}", "test failed at fen : ", test.fen)
        }
        total_nodes += nodes;
        println!("{}{}{}{}", "passed test ", test.fen, " depth : ", test.depth);
    }
    let nps = total_nodes / start_time.elapsed().as_secs();
    println!("{}{}", "passed test suite at nps ", nps)
}

