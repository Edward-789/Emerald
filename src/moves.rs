use crate::utils::{Pieces, SQUARE_TO_STR};

#[derive(Copy, Clone)]
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
        format!("{}{}", SQUARE_TO_STR[self.from_square()], SQUARE_TO_STR[self.to_square()])
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

    pub const ROOK_FROM_CASTLING: [usize; 4] = [0, 7, 56, 63];
    pub const ROOK_TO_CASTLING: [usize; 4] = [3, 5, 59, 61];
    
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
        self.moves[self.length] = Move {move_value :  from as u16 | (to as u16) << 6 | flag << 12};
        self.length += 1;
    } 

}