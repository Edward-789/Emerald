use std::time::Instant;

use crate::{board::Board, eval::eval, moves::Move};


#[allow(dead_code)]
pub struct Searcher {
    max_time : u128,
    timer : Instant,
    zobrist_history : Vec<u64>,
    pub best_move : Move
}


impl Searcher {

    pub const SCORE_MATE: i32 = -10000;

    pub fn new(
        max_time : u128,
        zobrist_history : Vec<u64>,
    ) -> Self {
        Self {
            max_time,
            timer : Instant::now(),
            zobrist_history,
            best_move : Move::NULL
        }
    }

    pub fn search(&mut self, mut alpha : i32, beta : i32, depth: u8, board: &Board, ply: u8) -> i32 {
        let root = ply == 0;
        let leaf = depth == 1;
        let mut moves_played = 0;
        let mut best_score = -20000; 

        if leaf {
            return eval(board);
        }

        let moves = board.psuedolegal_movegen();
        for i in 0..moves.length {
            let mov = moves.moves[i];
            let mut next_board = *board;

            if !next_board.apply(mov) {
                continue;
            };

            moves_played += 1;
            let score = -self.search(-beta, -alpha, depth - 1, &next_board, ply + 1);
            
            if score > best_score {
                best_score = score;
                if score > alpha {
                    alpha = score;
                }
                if root {
                    self.best_move = mov;
                }
                if alpha > beta {
                    break;
                }
            }
        }

        if moves_played == 0 {
            return if board.in_check() {Self::SCORE_MATE} else {0}
        }

        best_score
    }
}