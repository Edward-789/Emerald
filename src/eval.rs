use crate::{board::Board, pop_lsb, utils::{Colour, Pieces}};

pub struct Evaluator;

impl Evaluator {

    pub fn eval(board : &Board) -> i32 {
        let mut mg = 0;
        let mut eg = 0;
        let mut phase = 0;
        for stm in [Colour::White, Colour::Black] {
            for i in 2..=7 {
                let piece_type = Pieces::convert_num_to_piece(i);

                let mut piece_colour_bb = board.get_piece_colour_bitboard(piece_type, stm);
                while piece_colour_bb > 0 {
                    let idx = pop_lsb!(piece_colour_bb) ^ if stm == Colour::White {56} else {0};

                    mg += Self::MG_PSTS[i][idx] + Self::MG_PIECE_VALS[i];
                    eg += Self::EG_PSTS[i][idx] + Self::EG_PIECE_VALS[i];
                    phase += Self::PHASE_VALS[i];
                }      
            }
            mg *= -1;
            eg *= -1;
        }

        (mg * phase + eg * (24 - phase)) / 24 * (if board.colour_to_move == Colour::White {1} else {-1})
    }
}