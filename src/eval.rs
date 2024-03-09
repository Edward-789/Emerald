use crate::{board::Board, pop_lsb, utils::{Colour, Pieces}};

const PIECE_VALS: [i32; 8] = [0, 0, 100, 300, 300, 500, 900, 0];

pub fn eval(board : &Board) -> i32 {
    let mut eval = 0;
    for stm in [Colour::White, Colour::Black] {
        for i in 2..=7 {
            let piece_type = Pieces::convert_num_to_piece(i);

            let mut piece_colour_bb = board.get_piece_colour_bitboard(piece_type, stm);
            while piece_colour_bb > 0 {
                pop_lsb!(piece_colour_bb);
                eval += PIECE_VALS[i];
            }      
        }

        eval *= -1;
    }

    eval * if board.colour_to_move == Colour::White {1} else {-1}
}