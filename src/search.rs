use std::time::Instant;

use crate::{board::Board, eval::Evaluator, moves::Move, tt::TTable};


#[allow(dead_code)]
pub struct Searcher {
    pub max_time : u128,
    pub timer : Instant,
    pub zobrist_history : Vec<u64>,
    pub best_move : Move,
    pub tt : TTable
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
            best_move : Move::NULL,
            tt : TTable::new(524288)
        }
    }

    fn search(&mut self, mut alpha : i32, beta : i32, depth: u8, board: &Board, ply: u8) -> i32 {
        let root = ply == 0;
        let leaf = depth == 1;
        let mut moves_played = 0;
        let mut best_score = -20000; 
        let mut best_move = Move::NULL;

        if leaf {
            return self.qsearch(alpha, beta, board);
        }

        let tt_entry = self.tt.get_entry(board.zobrist);
        let mut moves = board.psuedolegal_movegen(false);
        let mut scores = [0; 218];
        
        for i in 0..moves.length {
            let mov = moves.moves[i];

            scores[i] = if mov == tt_entry.best_move && tt_entry.hash == board.zobrist{1_000_000} else 
                        if board.move_is_capture(mov) { (10_000 * (board.piece_type(mov.capture_square()).unwrap() as usize)) - (board.piece_type(mov.from_square()).unwrap() as usize)} else 
                        {0};
        }


        for i in 0..moves.length {
            if self.timer.elapsed().as_millis() * 30 > self.max_time {
                return -Self::SCORE_MATE;
            }

            //incremental sort

            for j in i + 1..moves.length {
                if scores[j] > scores[i] {
                    scores.swap(j, i);
                    moves.moves.swap(j, i);
                }
            }
            let mov = moves.moves[i];
            let mut next_board = *board;

            if !next_board.apply(mov) {
                continue;
            };

            moves_played += 1;
            let score = -self.search(-beta, -alpha, depth - 1, &next_board, ply + 1);
            
            if score > best_score {
                best_score = score;
                best_move = mov;
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

        self.tt.store(best_move, board.zobrist);

        best_score
    }
    
    fn qsearch(&self, mut alpha : i32, beta : i32, board : &Board) -> i32 {

        let mut eval = Evaluator::eval(board);

        if eval >= beta {
            return eval;
        }

        let tt_entry = self.tt.get_entry(board.zobrist);

        alpha = alpha.max(eval);
        let mut moves = board.psuedolegal_movegen(true);
        let mut scores = [0; 218];
        

        for i in 0..moves.length {
            let mov = moves.moves[i];

            scores[i] = if tt_entry.best_move == mov && tt_entry.hash == board.zobrist {1_000_000} else {
                    10000 * (board.piece_type(mov.capture_square()).unwrap() as usize) - (board.piece_type(mov.from_square()).unwrap() as usize)
            }    
        }

        for i in 0..moves.length {
            if self.timer.elapsed().as_millis() * 30 > self.max_time {
                return -Self::SCORE_MATE;
            }

            for j in i + 1..moves.length {
                if scores[j] > scores[i] {
                    scores.swap(j, i);
                    moves.moves.swap(j, i);
                }
            }
            let mov = moves.moves[i];
            let mut next_board = *board;

            if !next_board.apply(mov) {
                continue;
            };

            let score = -self.qsearch(-beta, -alpha, &next_board);

            if score > eval {
                eval = score;
                if score > alpha {
                    alpha = score;
                }
                if alpha > beta {
                    break;
                }
            }   
        }

        eval
    }
    pub fn iterative_deepening(&mut self, board : &Board) {
        for i in 2..255 {

            self.search(Self::SCORE_MATE, -Self::SCORE_MATE, i, board, 0);
            
            if self.timer.elapsed().as_millis() * 30 > self.max_time {
                break;
            }
        }
    }
}       