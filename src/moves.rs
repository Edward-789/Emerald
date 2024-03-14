use crate::{board::Board, utils::{Pieces, SQUARE_TO_STR}};

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    move_value : u16 
}

impl Move {
    pub const NULL : Move = Move {move_value : 0 };

    pub fn from_square(&self) -> usize {
        (self.move_value & 0b111111).into()
    }

    pub fn to_square(&self) -> usize {
        ((self.move_value >> 6) & 0b111111).into()
    }

    pub fn flag(&self) -> u16 {
        self.move_value >> 12   
    }

    pub fn to_uci(&self) -> String {
        format!("{}{}{}", SQUARE_TO_STR[self.from_square()], SQUARE_TO_STR[self.to_square()], self.flag_to_uci_char())
    }

    pub fn flag_to_uci_char(&self) -> &str {
        match self.flag() {
            Self::KNIGHT_PROMO => "n",
            Self::BISHOP_PROMO => "b",
            Self::ROOK_PROMO => "r",
            Self::QUEEEN_PROMO => "q",
            _ => ""
        }
    }
    pub fn new(from: usize, to: usize, flag: u16) -> Self {
        Self {move_value :  from as u16 | (to as u16) << 6 | flag << 12}
    }
    pub fn promo_piece(&self) -> Option<Pieces> {
        match self.flag() {
            Self::KNIGHT_PROMO => Some(Pieces::Knight),
            Self::BISHOP_PROMO => Some(Pieces::Bishop),
            Self::ROOK_PROMO => Some(Pieces::Rook),
            Self::QUEEEN_PROMO => Some(Pieces::Queen),
            _ => None
        }
    }

    pub fn capture_square(&self) -> usize {
        let to_square = self.to_square();
        if self.flag() != Self::EN_PASSANT {
            return to_square;
        }

        if Board::find_rank(to_square) == 5 {
            to_square - 8
        } else {
            to_square + 8
        }
    }

    pub const ROOK_FROM_CASTLING: [usize; 4] = [7, 0, 63, 56];
    pub const ROOK_TO_CASTLING: [usize; 4] = [5, 3, 61, 59];
    
    pub const NO_FLAG: u16 = 0b0000;
    pub const WHITE_KINGSIDE: u16 = 0b0001;
    pub const WHITE_QUEENSIDE: u16 = 0b0010;
    pub const BLACK_KINGSIDE: u16 = 0b0011;
    pub const BLACK_QUEENSIDE: u16 = 0b0100;
    pub const EN_PASSANT: u16 = 0b0101;
    pub const DOUBLE_PAWN_PUSH: u16 = 0b0110;
    pub const KNIGHT_PROMO: u16 = 0b0111;
    pub const BISHOP_PROMO: u16 = 0b1000;
    pub const ROOK_PROMO: u16 = 0b1001;
    pub const QUEEEN_PROMO: u16 = 0b1010;
    
}

pub struct MoveList {
    pub moves : [Move; 218],
    pub length : usize
}  

impl MoveList {
    pub const EMPTY : Self = Self {
        moves : [Move::NULL; 218],
        length : 0,
    };

    pub fn push(&mut self, from : usize, to : usize, flag : u16) {
        self.moves[self.length] = Move::new(from , to, flag);
        self.length += 1;
    }  
}