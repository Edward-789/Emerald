#[derive(PartialEq)]
pub enum Colour {
    White = 0,
    Black = 1,
}

pub enum Pieces {
    None = 0,
    Pawn = 2,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 6,
    King = 7,
}

pub enum Castling {
    WhiteQueen = 8,
    WhiteKing = 4,
    BlackQueen = 2,
    BlackKing = 1,
    None = 0
}

pub struct Mask;
impl Mask {
    pub const FILE_A_MASK: u64 = 0x0101010101010101;
    pub const FILE_H_MASK: u64 = 0x8080808080808080;
    
    pub const RANK_1_MASK: u64 = 0x00000000000000FF;
    pub const RANK_8_MASK: u64 = 0xFF00000000000000;
}
